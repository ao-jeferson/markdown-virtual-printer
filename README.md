# Markdown Virtual Printer

**Print to Markdown from the native print dialog.**

An open source virtual printer designed to appear alongside physical printers in the operating system print dialog. Applications submit print jobs through the native system pipeline; the printer prepares a Markdown document and opens the system save dialog.

Development focus: macOS, Windows, and Linux. An experimental native Android print-service prototype is being added; iOS remains on the roadmap without a target date.

The shared Rust core reads the document delivered by the print system and generates Markdown. The **Keep formatting** checkbox is enabled by default and structures readable text as a report; when unchecked, extracted text is kept as Markdown source. Print pipelines may flatten document semantics, so exact visual fidelity cannot always be reconstructed from a print job.

## Architecture

- Shared Rust core: reads text and PDF print jobs and generates Markdown.
- Native OS adapter: registers and handles the virtual printer through the system print pipeline.
- Native UI: presents the platform save dialog and formatting option.

## Development status

The Rust core and a CUPS backend prototype are under active development. Platform installers are not yet a stable release. See [ROADMAP.md](ROADMAP.md) for platform status.

## Community

Contributions are welcome through pull requests. See [CONTRIBUTING.md](CONTRIBUTING.md), [ROADMAP.md](ROADMAP.md), and [docs/BRANCHING.md](docs/BRANCHING.md).

[Português (Brasil)](README.pt-BR.md) · [Architecture](docs/architecture.md) · [Release process](docs/releases.md)

Android prototype instructions: [platforms/android/README.md](platforms/android/README.md).

MIT License. See [LICENSE](LICENSE).
