import * as fs from "node:fs";
import * as path from "node:path";

/** Sem dependência de `vscode` de propósito — só assim dá pra testar fora do extension host. */

export function resolveLanggraphBin(projectDir: string): string | undefined {
  const winBin = path.join(projectDir, ".venv", "Scripts", "langgraph.exe");
  const nixBin = path.join(projectDir, ".venv", "bin", "langgraph");
  if (fs.existsSync(winBin)) return winBin;
  if (fs.existsSync(nixBin)) return nixBin;
  return undefined;
}

export function resolvePythonBin(projectDir: string): string | undefined {
  const winBin = path.join(projectDir, ".venv", "Scripts", "python.exe");
  const nixBin = path.join(projectDir, ".venv", "bin", "python");
  if (fs.existsSync(winBin)) return winBin;
  if (fs.existsSync(nixBin)) return nixBin;
  return undefined;
}
