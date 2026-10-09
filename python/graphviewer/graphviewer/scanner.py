"""Descobre funções marcadas com @graphviewer.node num diretório de projeto.

Introspecção via import real (não parsing estático de AST) — ver SPEC-0, seção
"Mecanismo": mais simples e mais preciso, ao custo de rodar o código do módulo
na hora de escanear.
"""

from __future__ import annotations

import dataclasses
import importlib.util
import inspect
import json
import sys
from pathlib import Path

from . import REGISTRY_ATTR

_SKIP_DIRS = {".venv", "venv", "node_modules", "__pycache__", ".git", ".graphviewer"}


@dataclasses.dataclass
class DiscoveredNode:
    name: str
    function: str
    module: str
    file: str
    line: int
    signature: str
    docstring: str | None


def _iter_python_files(root: Path):
    for path in root.rglob("*.py"):
        if any(part in _SKIP_DIRS for part in path.parts):
            continue
        yield path


def _import_module_from_path(path: Path, root: Path):
    rel = path.relative_to(root).with_suffix("")
    module_name = "graphviewer_scan." + ".".join(rel.parts)
    spec = importlib.util.spec_from_file_location(module_name, path)
    if spec is None or spec.loader is None:
        return None
    module = importlib.util.module_from_spec(spec)
    sys.modules[module_name] = module
    spec.loader.exec_module(module)
    return module


def discover(root_dir: str) -> list[DiscoveredNode]:
    root = Path(root_dir).resolve()
    found: list[DiscoveredNode] = []

    for path in _iter_python_files(root):
        try:
            module = _import_module_from_path(path, root)
        except Exception as e:  # projeto do usuário pode ter módulos que falham ao importar
            print(f"[graphviewer] aviso: falha importando {path}: {e}", file=sys.stderr)
            continue
        if module is None:
            continue

        for _, obj in inspect.getmembers(module, inspect.isfunction):
            if obj.__module__ != module.__name__:
                continue
            meta = getattr(obj, REGISTRY_ATTR, None)
            if meta is None:
                continue
            try:
                sig = str(inspect.signature(obj))
            except (TypeError, ValueError):
                sig = "(...)"
            try:
                line = inspect.getsourcelines(obj)[1]
            except OSError:
                line = 0

            found.append(
                DiscoveredNode(
                    name=meta["name"],
                    function=obj.__name__,
                    module=module.__name__,
                    file=str(path.relative_to(root)),
                    line=line,
                    signature=sig,
                    docstring=inspect.getdoc(obj),
                )
            )

    return found


def main() -> None:
    root_dir = sys.argv[1] if len(sys.argv) > 1 else "."
    nodes = discover(root_dir)
    json.dump([dataclasses.asdict(n) for n in nodes], sys.stdout, ensure_ascii=False, indent=2)
    print(file=sys.stderr)  # garante que stdout termina sem lixo se alguém encadear


if __name__ == "__main__":
    main()
