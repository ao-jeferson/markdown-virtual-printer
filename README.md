# Markdown Virtual Printer

**A native virtual printer that saves print jobs as Markdown.**

This project aims to install a printer in the operating system's print dialog. Choose it from an application's normal **Print** command, and the job is converted into a Markdown file with a native save dialog. It is not a Markdown-to-PDF converter or a document editor.

> Status: early converter prototype. The virtual printer and native installers are not implemented yet.

[Leia em português](README.pt-BR.md)

## Product direction

The application has a shared Rust core that receives a print job, extracts the content and structure available in its input, and writes Markdown. Small platform-specific adapters register the virtual printer, receive jobs, show the native save interface, and install or remove the integration.

The first platform is macOS, followed by Windows and Linux. Android and iOS are roadmap possibilities with no target date.

The converter cannot recreate semantics that the source job does not contain. A print job may preserve appearance while losing the original headings, links, tables, or other document structure.

## Current prototype

The existing command-line converter is an experimental Python prototype. It accepts HTML, text/Markdown, and PDF; PDF extraction currently uses pdftotext. This prototype is for exploring conversion quality. It is not the planned self-contained product and does not install a printer.

The target application will use a compiled Rust core and platform-native packages so users do not need Python installed. Each platform package will include the project's runtime components and explain any operating-system permission needed to add or remove a printer.

## Architecture

We propose a **hexagonal architecture (ports and adapters)**:

- **Rust core:** canonical document model, format conversion, Markdown output, report templates, and conversion warnings.
- **Format adapters:** parse HTML, plain text, PDF, and print-job formats into the canonical model.
- **Native platform adapters:** printer registration, job intake, save dialogs, and system integration.
- **Installer packages:** install, update, and remove the app and its virtual printer using each OS's supported mechanism.

The core must not depend on macOS, Windows, or Linux UI APIs. See [architecture](docs/architecture.md) for component boundaries and [roadmap](ROADMAP.md) for platform order.

## Development

The current Python prototype requires Python 3.10 or newer. PDF conversion requires Poppler's pdftotext; HTML conversion uses the optional markdownify dependency.


    python3 -m venv .venv
    source .venv/bin/activate
    python -m pip install -e '.[html]'
    mdprint document.html -o document.md

These steps run the prototype only. They do not install a virtual printer.

## Contributing

Ideas are welcome as GitHub Issues using the **Idea** template. Add the relevant English labels, such as type: idea, platform: macos, or area: virtual-printer. Feature branches target develop; releases target main. See [contributing](CONTRIBUTING.md) and the [Gitflow strategy](docs/branch-strategy.md).

## License

MIT. The license allows use, modification, and redistribution subject to its notice. See [LICENSE](LICENSE).