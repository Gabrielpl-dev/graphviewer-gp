"""Formaliza a verificação manual feita durante o desenvolvimento: o scanner acha as funções
marcadas com @graphviewer.node em examples/example_graph.py e ignora as não-marcadas."""

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
