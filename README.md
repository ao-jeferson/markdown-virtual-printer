# Markdown Virtual Printer

**Print to Markdown from the native print dialog.**

An open source virtual printer designed to appear alongside physical printers in the operating system print dialog. Applications submit print jobs through the native system pipeline; the printer prepares a Markdown document and lets the user save it.

Development order: macOS, Windows, Linux. Android and iOS are roadmap items without target dates.

This project is in early development. The shared Rust core defines the document model and Markdown output boundary. Native printer integration, interface, and installer are implemented per operating system. The product is the virtual printer; the core is not a standalone converter application.

## Architecture

- Shared Rust core: print-job document model and Markdown serialization.
- Native OS adapter: registers and handles the printer through the system print pipeline.
- Native UI and installer: use platform conventions for save flow and installation.

## Community

Contributions are welcome through pull requests. See [CONTRIBUTING.md](CONTRIBUTING.md), [ROADMAP.md](ROADMAP.md), and [docs/BRANCHING.md](docs/BRANCHING.md).

[Português (Brasil)](README.pt-BR.md) · [Architecture](docs/ARCHITECTURE.md) · [Release process](docs/RELEASING.md)

MIT License. See [LICENSE](LICENSE).
