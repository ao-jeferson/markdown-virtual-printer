# Markdown Virtual Printer

**Imprima em Markdown pelo diálogo nativo de impressão.**

Uma impressora virtual livre, feita para aparecer junto às impressoras físicas no diálogo do sistema. Os aplicativos enviam trabalhos pelo fluxo nativo de impressão; a impressora gera um documento Markdown e abre a janela do sistema para salvá-lo.

Ordem de desenvolvimento: macOS, Windows e Linux. Android e iOS estão no roadmap, sem previsão.

O núcleo compartilhado em Rust lê o documento recebido pela fila de impressão e gera Markdown. A caixa **Manter formatação**, marcada por padrão, organiza o texto legível como relatório; desmarcada, mantém o texto extraído como código-fonte Markdown. A fila pode achatar a semântica do documento, então nem sempre é possível reconstruir a aparência visual exata.

## Estado do desenvolvimento

O núcleo Rust e um protótipo de backend CUPS estão em desenvolvimento. Os instaladores ainda não são uma versão estável. Consulte o [ROADMAP.md](ROADMAP.md) para ver o estado de cada sistema.

Contribuições: pull requests são bem-vindos. Consulte [CONTRIBUTING.md](CONTRIBUTING.md), [ROADMAP.md](ROADMAP.md) e [docs/BRANCHING.md](docs/BRANCHING.md).

[English](README.md) · [Arquitetura](docs/architecture.md) · [Processo de releases](docs/releases.md) · Licença MIT.
