"""Marca funções como nós descobríveis pelo graphviwer-gp (SPEC-0)."""

from __future__ import annotations

from typing import Callable, TypeVar

F = TypeVar("F", bound=Callable)

REGISTRY_ATTR = "__graphviwer_node__"


def node(name: str | None = None) -> Callable[[F], F]:
    """Decorator: marca a função como um nó descobrível pelo scanner.

    Não altera o comportamento da função em nada — só anexa metadado que o
    scanner (`graphviwer.scanner`) lê depois via `inspect`.
    """

    def decorator(func: F) -> F:
        node_name = name or func.__name__
        setattr(func, REGISTRY_ATTR, {"name": node_name})
        return func

    return decorator
