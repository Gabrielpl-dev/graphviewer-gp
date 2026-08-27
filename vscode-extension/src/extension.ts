import * as vscode from "vscode";
import * as fs from "node:fs";
import * as path from "node:path";
import * as http from "node:http";
import { ChildProcess, spawn } from "node:child_process";

const PORT = 2024;
const BASE_URL = `http://127.0.0.1:${PORT}`;

/** Webview fixo na Activity Bar — só guarda a referência pra poder mandar `postMessage`
 * depois que um grafo for aberto via o comando `graphviwerGp.openGraph`. */
class GraphViewProvider implements vscode.WebviewViewProvider {
  private view: vscode.WebviewView | undefined;

  constructor(private readonly extensionUri: vscode.Uri) {}

  resolveWebviewView(webviewView: vscode.WebviewView): void {
    this.view = webviewView;
    webviewView.webview.options = { enableScripts: true };
    webviewView.webview.html = this.renderHtml(webviewView.webview);
  }

  setBaseUrl(url: string): void {
    this.view?.webview.postMessage({ command: "setBaseUrl", url });
  }

  reveal(): void {
    this.view?.show?.(true);
  }

  private renderHtml(webview: vscode.Webview): string {
    const htmlPath = vscode.Uri.joinPath(this.extensionUri, "media", "viewer.html");
    const raw = fs.readFileSync(htmlPath.fsPath, "utf-8");
    const nonce = randomNonce();
    return raw.replaceAll("__NONCE__", nonce);
  }
}

function randomNonce(): string {
  const chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let out = "";
  for (let i = 0; i < 32; i++) {
    out += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return out;
}

/** Verifica se já tem um `langgraph dev` respondendo em /ok. */
function probeHealthy(): Promise<boolean> {
  return new Promise((resolve) => {
    const req = http.get(`${BASE_URL}/ok`, { timeout: 1000 }, (res) => {
      resolve(res.statusCode === 200);
      res.resume();
    });
    req.on("error", () => resolve(false));
    req.on("timeout", () => {
      req.destroy();
      resolve(false);
    });
  });
}

async function waitUntilHealthy(timeoutMs: number): Promise<boolean> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (await probeHealthy()) return true;
    await new Promise((r) => setTimeout(r, 500));
  }
  return false;
}

let runningProcess: ChildProcess | undefined;
let runningProjectDir: string | undefined;

function resolveLanggraphBin(projectDir: string): string | undefined {
  const winBin = path.join(projectDir, ".venv", "Scripts", "langgraph.exe");
  const nixBin = path.join(projectDir, ".venv", "bin", "langgraph");
  if (fs.existsSync(winBin)) return winBin;
  if (fs.existsSync(nixBin)) return nixBin;
  return undefined;
}

async function ensureGraphServerRunning(projectDir: string): Promise<boolean> {
  // já tem um servidor nosso rodando pro mesmo projeto e ele responde? reusa.
  if (runningProcess && runningProjectDir === projectDir && (await probeHealthy())) {
    return true;
  }

  // projeto diferente do que já tava rodando (ou processo morreu) — mata o antigo.
  if (runningProcess) {
    runningProcess.kill();
    runningProcess = undefined;
    runningProjectDir = undefined;
  }

  const bin = resolveLanggraphBin(projectDir);
  if (!bin) {
    vscode.window.showErrorMessage(
      `graphviwer-gp: não achei ".venv" com langgraph instalado em ${projectDir}. ` +
        `Configure o projeto igual em ~/dev/lg-demo antes de abrir por aqui.`,
    );
    return false;
  }

  runningProcess = spawn(bin, ["dev", "--no-browser"], {
    cwd: projectDir,
    stdio: "ignore",
  });
  runningProjectDir = projectDir;

  runningProcess.on("exit", () => {
    runningProcess = undefined;
    runningProjectDir = undefined;
  });

  const ok = await waitUntilHealthy(20_000);
  if (!ok) {
    vscode.window.showErrorMessage(
      `graphviwer-gp: o langgraph dev não respondeu em ${BASE_URL}/ok depois de 20s.`,
    );
  }
  return ok;
}

export function activate(context: vscode.ExtensionContext): void {
  const provider = new GraphViewProvider(context.extensionUri);

  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider("graphviwerGp.graphView", provider),
    vscode.commands.registerCommand("graphviwerGp.openGraph", async (uri: vscode.Uri) => {
      const projectDir = path.dirname(uri.fsPath);
      provider.reveal();

      const ok = await ensureGraphServerRunning(projectDir);
      if (ok) {
        provider.setBaseUrl(BASE_URL);
      }
    }),
  );
}

export function deactivate(): void {
  runningProcess?.kill();
}
