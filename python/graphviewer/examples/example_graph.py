"""Exemplo mínimo pra testar o scanner do graphviewer — três funções marcadas."""

import graphviewer


@graphviewer.node("scan")
def s0_scan(state: dict) -> dict:
    """Faz o scan inicial do estado."""
    return {"etapa": "scan concluído"}


@graphviewer.node("process")
def s1_process(state: dict) -> dict:
    """Processa o resultado do scan."""
    return {"etapa": "processado"}


@graphviewer.node()
def s2_execute(state: dict) -> dict:
    return {"etapa": "executado"}


def nao_marcada(x: int) -> int:
    """Não deve aparecer na descoberta — sem decorator."""
    return x + 1
