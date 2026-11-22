"""Format-specific conversion into Markdown with explicit source limitations."""

from __future__ import annotations

import re
import shutil
import subprocess
from pathlib import Path


def convert(path: Path) -> str:
    suffix = path.suffix.lower()
    if suffix in {".html", ".htm"}:
        return html_to_markdown(path.read_text(encoding="utf-8", errors="replace"))
    if suffix in {".txt", ".md", ".markdown"}:
        return text_to_markdown(path.read_text(encoding="utf-8", errors="replace"))
    if suffix == ".pdf":
        return pdf_to_markdown(path)
    raise ValueError(f"Formato não suportado: {suffix or '(sem extensão)'}")


def html_to_markdown(source: str) -> str:
    try:
        from markdownify import markdownify
    except ImportError as exc:
        raise RuntimeError("Suporte HTML opcional ausente. Instale com: pip install 'mdprint[html]'") from exc
    result = markdownify(source, heading_style="ATX", bullets="-")
    return normalize_markdown(result)


def text_to_markdown(source: str) -> str:
    """Keep existing Markdown; turn plain text paragraph breaks into Markdown paragraphs."""
    return normalize_markdown(source)


def pdf_to_markdown(path: Path) -> str:
    executable = shutil.which("pdftotext")
    if not executable:
        raise RuntimeError("Conversão de PDF requer pdftotext (Poppler) instalado no sistema.")
    result = subprocess.run(
        [executable, "-layout", str(path), "-"],
        check=False,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    if result.returncode:
        message = result.stderr.strip() or f"pdftotext terminou com código {result.returncode}"
        raise RuntimeError(message)
    # Preserve paragraph and rough column spacing from pdftotext; Markdown cannot
    # reproduce page layout or reliably infer heading levels from a rendered PDF.
    return normalize_markdown(result.stdout)


def normalize_markdown(source: str) -> str:
    source = source.replace("\r\n", "\n").replace("\r", "\n")
    source = "\n".join(line.rstrip() for line in source.split("\n"))
    source = re.sub(r"\n{3,}", "\n\n", source)
    return source.strip() + ("\n" if source.strip() else "")
