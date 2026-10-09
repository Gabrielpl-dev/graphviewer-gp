"""Formaliza a verificação manual feita durante o desenvolvimento: o scanner acha as funções
marcadas com @graphviewer.node em examples/example_graph.py e ignora as não-marcadas."""

import sys
from pathlib import Path

from graphviewer.scanner import discover

EXAMPLES_DIR = Path(__file__).parent.parent / "examples"


def test_discovers_only_marked_functions():
    nodes = discover(str(EXAMPLES_DIR))
    names = {n.name for n in nodes}

    assert names == {"scan", "process", "s2_execute"}


def test_node_without_explicit_name_falls_back_to_function_name():
    nodes = discover(str(EXAMPLES_DIR))
    s2 = next(n for n in nodes if n.function == "s2_execute")

    assert s2.name == "s2_execute"


def test_signature_and_docstring_extracted():
    nodes = discover(str(EXAMPLES_DIR))
    scan = next(n for n in nodes if n.name == "scan")

    assert scan.signature == "(state: dict) -> dict"
    assert scan.docstring == "Faz o scan inicial do estado."


def test_imported_node_is_discovered_only_in_defining_module(tmp_path, monkeypatch):
    (tmp_path / "module_a.py").write_text(
        'from graphviewer import node\n'
        '@node("shared")\n'
        'def shared(state: dict) -> dict:\n'
        '    return state\n'
    )
    (tmp_path / "module_b.py").write_text("from module_a import shared\n")
    monkeypatch.syspath_prepend(str(tmp_path))
    # O import de B também registra A em sys.modules; restaura após o teste.
    monkeypatch.delitem(sys.modules, "module_a", raising=False)

    try:
        nodes = discover(str(tmp_path))
    finally:
        sys.modules.pop("module_a", None)

    assert len(nodes) == 1
    assert nodes[0].name == "shared"
    assert nodes[0].function == "shared"
    assert nodes[0].module == "graphviewer_scan.module_a"
    assert nodes[0].file == "module_a.py"
