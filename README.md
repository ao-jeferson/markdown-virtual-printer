# mdprint

**Imprima para Markdown.** Projeto comunitário para converter documentos em Markdown estruturado e, mais adiante, oferecer uma impressora virtual nativa para macOS, Linux e Windows.

> Estado: protótipo inicial do conversor. A integração com a fila de impressão ainda não foi implementada.

## Objetivos

- Converter HTML, texto simples e PDF para Markdown, preservando a estrutura que a origem permitir.
- Oferecer interface para escolher destino e nome do arquivo.
- Aplicar modelos de relatório a respostas de chat.
- Integrar impressão virtual, começando pelo macOS.
- Documentar limitações de conversão e formatação inferida.

## Conversor inicial

Requer Python 3.10 ou mais recente. PDF requer `pdftotext` (Poppler). HTML usa `markdownify` quando instalado.

```sh
python3 -m venv .venv
source .venv/bin/activate
python -m pip install -e '.[html]'
mdprint arquivo.html -o saida.md
mdprint documento.pdf -o documento.md
mdprint notas.txt -o notas.md
```

Relatório a partir de texto ou Markdown de uma conversa:

```sh
mdprint resposta.txt --template report -o relatorio.md --title "Resumo da conversa"
```

A versão atual não instala impressoras nem altera configurações do sistema.

## Qualidade e limites

Markdown descreve conteúdo e estrutura, não o layout exato da página. HTML semântico pode manter títulos, listas, links, tabelas e código. PDF contém principalmente texto posicionado; a conversão preserva a ordem textual disponível, mas não recupera com confiabilidade estilos, colunas, imagens ou semântica ausente. PDFs digitalizados exigem OCR, ainda não incluído.

## Desenvolvimento

```sh
python -m pip install -e '.[dev,html]'
python -m mdprint --help
```

Consulte [arquitetura](docs/architecture.md), [roadmap](ROADMAP.md), [estratégia de branches](docs/branch-strategy.md), [contribuições](CONTRIBUTING.md) e [releases](docs/releases.md).

## Comunidade e licença

Forks e pull requests estão abertos. PRs são revisados antes de entrar em `main`; veja [CONTRIBUTING.md](CONTRIBUTING.md). A licença MIT permite usar, modificar e distribuir o software, inclusive comercialmente, mantendo os avisos de copyright e licença. Veja [LICENSE](LICENSE).
