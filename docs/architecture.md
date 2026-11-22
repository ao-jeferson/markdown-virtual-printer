# Architecture

The product is a **native virtual printer that exports print jobs as Markdown**. The operating system presents it in the normal print flow; a user selects it, the app converts the received job, then presents the native save dialog. This differs from converting a Markdown file to PDF.

[Português (Brasil)](architecture.pt-BR.md)

## Recommended pattern: hexagonal architecture

Use ports and adapters around a platform-independent Rust core. The core owns document semantics and conversion. Platform adapters own OS print queues, native UI, job lifecycle, and installation. This keeps OS-specific code replaceable without coupling parsing and Markdown output to one platform.


    Print dialog
        -> native printer adapter
        -> normalized print-job input
        -> Rust conversion pipeline
        -> Markdown document + warnings
        -> native save dialog
        -> .md file

## Component boundaries

- **Document model:** a canonical UTF-8 model for headings, paragraphs, lists, tables, links, code, images, and metadata where present.
- **Input adapters:** convert HTML, plain text, PDF, and OS print-job payloads into the canonical model. Each adapter reports information it could not preserve.
- **Conversion pipeline:** normalize content, apply optional report templates, and produce Markdown plus structured warnings.
- **Output port:** writes the selected Markdown file safely and atomically.
- **Platform adapters:** register and remove the virtual printer, receive or spool jobs, communicate with the user-session app, and invoke native save dialogs.
- **Installer:** package and install the application and platform adapter using the OS-supported mechanism.

Keep the core crates independent of GUI frameworks and OS printer SDKs. Prefer an explicit Rust workspace with a small core crate and separate adapter crates; add a platform crate only when its implementation starts.

## Platform integration

### macOS

Use a supported CUPS printing integration and a native macOS helper for the save dialog. Printing filters may run in a restricted service context without access to the user's desktop, so the print component should hand the completed job to the user-session app over a narrow local IPC interface. Do not depend on legacy PPD-only drivers.

### Windows

Use the supported virtual printer and Print Support App architecture. Keep the Windows print API and packaging code in a Windows adapter; call the Rust core through a stable library boundary or a local worker process.

### Linux

Use CUPS for queue/job integration and a desktop helper for the save dialog. Keep packaging details separate so supported distributions can be selected explicitly.

### Android and iOS

These are future roadmap targets without dates. Reassess their print APIs and application distribution requirements after the desktop core and workflows are stable.

## Conversion limits

A print job may contain visual output without semantic headings, links, tables, or source metadata. The converter must preserve what it can, emit warnings for uncertain or lossy transformations, and never claim to reconstruct information absent from the job. Scanned PDFs may require OCR and are outside the first desktop milestone.

## Security and privacy

Process jobs locally by default. Treat every print payload as untrusted input; limit memory, CPU, recursion, and temporary storage; never execute extracted content. Use restricted local IPC and remove temporary job files after completion or cancellation.

## Current implementation status

The repository contains an experimental Python CLI prototype. It is not the target runtime. The planned desktop releases use the Rust core and platform-native packaging; users should not need Python installed.