"""Cliente HTTP/SSE pra API REST do `langgraph dev`.

O servidor MCP é **um cliente a mais** dessa API, igual o webview
(`vscode-extension/media/viewer.html`) e o app Rust já são — não inventa transporte novo, não
duplica o motor de grafo (ver SPEC-1).
"""

from __future__ import annotations

import json
from typing import Any

import httpx

from .process import BASE_URL

_REQUEST_TIMEOUT = 10.0
_RUN_TIMEOUT = 120.0

_IGNORED_NODE_IDS = {"__start__", "__end__"}


async def get_first_assistant_id() -> str | None:
    async with httpx.AsyncClient(timeout=_REQUEST_TIMEOUT) as client:
        resp = await client.post(f"{BASE_URL}/assistants/search", json={})
        resp.raise_for_status()
        assistants = resp.json()
    if not assistants:
        return None
    return assistants[0]["assistant_id"]


async def get_graph(assistant_id: str) -> dict[str, Any]:
    async with httpx.AsyncClient(timeout=_REQUEST_TIMEOUT) as client:
        resp = await client.get(f"{BASE_URL}/assistants/{assistant_id}/graph")
        resp.raise_for_status()
        return resp.json()


async def run_wait(assistant_id: str, input_: dict[str, Any]) -> dict[str, Any]:
    async with httpx.AsyncClient(timeout=_RUN_TIMEOUT) as client:
        resp = await client.post(
            f"{BASE_URL}/runs/wait",
            json={"assistant_id": assistant_id, "input": input_},
        )
        resp.raise_for_status()
        return resp.json()


async def stream_events(assistant_id: str, input_: dict[str, Any]) -> list[dict[str, Any]]:
    """Roda `/runs/stream` e devolve a lista de eventos de início/fim de cada nó já coletados.

    Mesmo parsing de SSE e mesmo filtro (`on_chain_start`/`on_chain_end`) que `runGraph()` aplica
    em `vscode-extension/media/viewer.html` — só que aqui devolvidos de uma vez no fim em vez de
    atualizar uma UI ao vivo: tools MCP são request/resposta, não tem por que virar streaming de
    verdade nessa v1 (ver SPEC-1).
    """
    collected: list[dict[str, Any]] = []
    buffer = ""

    async with httpx.AsyncClient(timeout=_RUN_TIMEOUT) as client:
        async with client.stream(
            "POST",
            f"{BASE_URL}/runs/stream",
            json={"assistant_id": assistant_id, "input": input_, "stream_mode": ["events"]},
        ) as resp:
            resp.raise_for_status()
            async for raw_chunk in resp.aiter_text():
                buffer += raw_chunk.replace("\r\n", "\n")
                *chunks, buffer = buffer.split("\n\n")

                for chunk in chunks:
                    lines = chunk.split("\n")
                    event_line = next((l for l in lines if l.startswith("event:")), None)
                    data_line = next((l for l in lines if l.startswith("data:")), None)
                    if not event_line or not data_line:
                        continue
                    if event_line.replace("event:", "", 1).strip() != "events":
                        continue

                    data = json.loads(data_line.replace("data:", "", 1).strip())
                    event_type = data.get("event")
                    if event_type not in ("on_chain_start", "on_chain_end"):
                        continue

                    # só `langgraph_node` identifica um nó de verdade — a chamada da chain do
                    # grafo inteiro (ex: "demo") não carrega esse metadado e não deve entrar,
                    # mesmo filtro que o `nodeOrder.includes(nodeName)` de viewer.html aplica.
                    node_name = (data.get("metadata") or {}).get("langgraph_node")
                    if not node_name or node_name in _IGNORED_NODE_IDS:
                        continue

                    payload_key = "input" if event_type == "on_chain_start" else "output"
                    payload = (data.get("data") or {}).get(payload_key)

                    collected.append({"node": node_name, "event": event_type, "payload": payload})

    return collected
