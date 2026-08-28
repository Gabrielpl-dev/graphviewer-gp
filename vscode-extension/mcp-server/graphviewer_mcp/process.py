"""Sobe e reaproveita um `langgraph dev` local pro projeto pedido.

Mesma lógica de `ensureGraphServerRunning`/`discoverNodes` em
`vscode-extension/src/extension.ts`, reimplementada aqui porque o servidor MCP é um processo
Python independente do webview — duplicar essa lógica pequena é aceitável (ver SPEC-1, não vale
a complexidade de compartilhar código entre TS e Python agora).

`discover_nodes` roda o scanner via subprocess usando o **python do `.venv` do projeto alvo**
(não o interpretador deste servidor MCP) — é o mesmo truque de `resolvePythonBin` no extension.ts:
o projeto alvo é quem tem `graphviewer` e as próprias dependências (`langgraph`, etc.) instaladas,
então importar os módulos do usuário só funciona rodando com o Python dele.
"""

from __future__ import annotations

import asyncio
import json
import subprocess
import sys
import time
from pathlib import Path

import httpx

PORT = 2024
BASE_URL = f"http://127.0.0.1:{PORT}"

_HEALTHY_TIMEOUT = 1.0
_STARTUP_TIMEOUT = 20.0
_SCANNER_TIMEOUT = 15.0

_running_process: asyncio.subprocess.Process | None = None
_running_project_dir: str | None = None


def _resolve_bin(project_dir: str, win_name: str, nix_name: str) -> Path | None:
    root = Path(project_dir)
    win_bin = root / ".venv" / "Scripts" / win_name
    nix_bin = root / ".venv" / "bin" / nix_name
    if win_bin.exists():
        return win_bin
    if nix_bin.exists():
        return nix_bin
    return None


def resolve_langgraph_bin(project_dir: str) -> Path | None:
    return _resolve_bin(project_dir, "langgraph.exe", "langgraph")


def resolve_python_bin(project_dir: str) -> Path | None:
    return _resolve_bin(project_dir, "python.exe", "python")


async def probe_healthy() -> bool:
    try:
        async with httpx.AsyncClient(timeout=_HEALTHY_TIMEOUT) as client:
            resp = await client.get(f"{BASE_URL}/ok")
            return resp.status_code == 200
    except httpx.HTTPError:
        return False


async def _wait_until_healthy(timeout: float) -> bool:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if await probe_healthy():
            return True
        await asyncio.sleep(0.5)
    return False


async def ensure_graph_server_running(project_dir: str) -> bool:
    """Garante um `langgraph dev` respondendo em `BASE_URL` pro `project_dir` dado.

    Só um servidor por vez (mesma limitação do extension.ts): se já tem um rodando pra outro
    projeto, mata e sobe o novo.
    """
    global _running_process, _running_project_dir

    if (
        _running_process is not None
        and _running_project_dir == project_dir
        and await probe_healthy()
    ):
        return True

    if _running_process is not None:
        _running_process.kill()
        _running_process = None
        _running_project_dir = None

    bin_path = resolve_langgraph_bin(project_dir)
    if bin_path is None:
        print(
            f"[graphviewer-mcp] não achei .venv com langgraph instalado em {project_dir}",
            file=sys.stderr,
        )
        return False

    _running_process = await asyncio.create_subprocess_exec(
        str(bin_path),
        "dev",
        "--no-browser",
        cwd=project_dir,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    _running_project_dir = project_dir

    ok = await _wait_until_healthy(_STARTUP_TIMEOUT)
    if not ok:
        print(
            f"[graphviewer-mcp] o langgraph dev não respondeu em {BASE_URL}/ok "
            f"depois de {_STARTUP_TIMEOUT:.0f}s",
            file=sys.stderr,
        )
    return ok


async def discover_nodes(project_dir: str) -> list[dict]:
    """Roda `python -m graphviewer.scanner <project_dir>` no `.venv` do projeto alvo.

    Devolve `[]` (sem erro visível) se o `.venv` não existir ou o pacote `graphviewer` não
    estiver instalado nele — feature opcional, não deve travar quem só quer o grafo/execução.
    """
    python_bin = resolve_python_bin(project_dir)
    if python_bin is None:
        return []

    try:
        proc = await asyncio.create_subprocess_exec(
            str(python_bin),
            "-m",
            "graphviewer.scanner",
            project_dir,
            cwd=project_dir,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        stdout, stderr = await asyncio.wait_for(proc.communicate(), timeout=_SCANNER_TIMEOUT)
    except (OSError, asyncio.TimeoutError) as e:
        print(f"[graphviewer-mcp] scanner de nós não rodou: {e}", file=sys.stderr)
        return []

    if proc.returncode != 0:
        print(
            f"[graphviewer-mcp] scanner de nós falhou: {stderr.decode(errors='replace')}",
            file=sys.stderr,
        )
        return []

    try:
        return json.loads(stdout)
    except json.JSONDecodeError as e:
        print(f"[graphviewer-mcp] scanner devolveu JSON inválido: {e}", file=sys.stderr)
        return []
