import { test } from "node:test";
import * as assert from "node:assert/strict";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";

import { resolveLanggraphBin, resolvePythonBin } from "../binResolver";

function makeTempProject(binRelPath: string): string {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "graphviewer-test-"));
  const binPath = path.join(dir, binRelPath);
  fs.mkdirSync(path.dirname(binPath), { recursive: true });
  fs.writeFileSync(binPath, "");
  return dir;
}

test("resolveLanggraphBin acha o binário quando o .venv existe", () => {
  const dir = makeTempProject(path.join(".venv", "Scripts", "langgraph.exe"));
  assert.equal(resolveLanggraphBin(dir), path.join(dir, ".venv", "Scripts", "langgraph.exe"));
});

test("resolveLanggraphBin retorna undefined sem .venv", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "graphviewer-test-"));
  assert.equal(resolveLanggraphBin(dir), undefined);
});

test("resolvePythonBin acha o binário quando o .venv existe", () => {
  const dir = makeTempProject(path.join(".venv", "Scripts", "python.exe"));
  assert.equal(resolvePythonBin(dir), path.join(dir, ".venv", "Scripts", "python.exe"));
});

test("resolvePythonBin retorna undefined sem .venv", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "graphviewer-test-"));
  assert.equal(resolvePythonBin(dir), undefined);
});
