"""Command-line entry point for mdprint."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from mdprint.converter import convert, normalize_markdown
from mdprint.templates import render_report


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="mdprint", description="Converte arquivos para Markdown.")
    parser.add_argument("input", type=Path, help="Arquivo .html, .txt, .md ou .pdf")
    parser.add_argument("-o", "--output", type=Path, help="Destino .md (padrão: ao lado da origem)")
    parser.add_argument("--template", choices=["report"], help="Modelo para saída de relatório")
    parser.add_argument("--title", default="Relatório", help="Título do relatório")
    return parser


def main() -> int:
    args = build_parser().parse_args()
    if not args.input.is_file():
        print(f"Arquivo não encontrado: {args.input}", file=sys.stderr)
        return 2
    output = args.output or args.input.with_suffix(".md")
    try:
        markdown = convert(args.input)
        if args.template == "report":
            markdown = render_report(markdown, args.title)
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(normalize_markdown(markdown), encoding="utf-8")
    except (OSError, RuntimeError, ValueError) as exc:
        print(f"mdprint: {exc}", file=sys.stderr)
        return 1
    print(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
