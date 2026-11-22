# Markdown Virtual Printer

**Imprima em Markdown pelo diálogo nativo de impressão.**

Um projeto livre de impressora virtual, planejado para aparecer junto às impressoras físicas no diálogo nativo. Os aplicativos enviam trabalhos pelo fluxo do sistema operacional; a impressora prepara um documento Markdown e permite salvá-lo.

Ordem de desenvolvimento: macOS, Windows e Linux. Android e iOS estão no roadmap, sem previsão.

O projeto está no início. O núcleo compartilhado em Rust define o modelo do documento e a saída Markdown. A integração, interface e instalação serão nativas em cada sistema. O produto é a impressora virtual; o núcleo não é um aplicativo conversor independente.

Contribuições: pull requests são bem-vindos. Consulte [CONTRIBUTING.md](CONTRIBUTING.md), [ROADMAP.md](ROADMAP.md) e [docs/BRANCHING.md](docs/BRANCHING.md).

[English](README.md) · [Arquitetura](docs/ARCHITECTURE.md) · [Processo de releases](docs/RELEASING.md) · Licença MIT.
