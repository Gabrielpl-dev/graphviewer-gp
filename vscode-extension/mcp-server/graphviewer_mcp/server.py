"""Servidor MCP: expõe o que o `langgraph dev` já sabe fazer como tools MCP.

Público-alvo é agente (Claude Code, ATLAS), não humano — permite abrir/rodar/inspecionar um
grafo LangGraph programaticamente, sem depender de clique em UI (ver SPEC-1).
"""

from __future__ import annotations

from typing import Any

from mcp.server.fastmcp import FastMCP

from . import client, process

mcp = FastMCP("graphviewer-gp")


@mcp.tool()
async def list_discovered_nodes(project_dir: str) -> list[dict[str, Any]]:
    """Lista as funções marcadas com @graphviewer.node no projeto (não precisa do langgraph dev rodando)."""
    return await process.discover_nodes(project_dir)


async def _ensure_assistant(project_dir: str) -> str:
    ok = await process.ensure_graph_server_running(project_dir)
    if not ok:
        raise RuntimeError(
            f"não consegui subir/conectar no langgraph dev de {project_dir} "
            f"(confira se {project_dir}/.venv tem langgraph-cli[inmem] instalado)"
        )

    assistant_id = await client.get_first_assistant_id()
    if assistant_id is None:
        raise RuntimeError(f"langgraph dev de {project_dir} não tem nenhum assistant registrado")
    return assistant_id


@mcp.tool()
async def get_graph_structure(project_dir: str) -> dict[str, Any]:
    """Sobe (ou reaproveita) o langgraph dev do projeto e devolve a estrutura do grafo (nós/arestas)."""
    assistant_id = await _ensure_assistant(project_dir)
    return await client.get_graph(assistant_id)


@mcp.tool()
async def run_graph(project_dir: str, input: dict[str, Any]) -> dict[str, Any]:
    """Roda uma execução completa (POST /runs/wait) e devolve o estado final."""
    assistant_id = await _ensure_assistant(project_dir)
    return await client.run_wait(assistant_id, input)


@mcp.tool()
async def stream_run_events(project_dir: str, input: dict[str, Any]) -> list[dict[str, Any]]:
    """Roda uma execução (POST /runs/stream) e devolve a lista de eventos de início/fim de cada nó."""
    assistant_id = await _ensure_assistant(project_dir)
    return await client.stream_events(assistant_id, input)


def main() -> None:
    mcp.run()


if __name__ == "__main__":
    main()
