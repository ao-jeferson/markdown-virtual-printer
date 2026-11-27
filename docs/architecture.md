# Architecture

The product is a virtual printer exposed by each operating system. Its native adapter receives a spooled print job, calls the shared Rust core, and asks the user where to save the `.md` file.

```text
Application -> native print queue -> OS adapter -> Rust core -> native Save dialog -> .md
```

## Shared core

The core currently accepts UTF-8 plain text, Markdown, and PDF. PDF text extraction is local and bounded to 256 MiB. It does not write files or make network requests. Its C ABI is intended for platform front ends.

The native **Keep formatting** checkbox is enabled by default. Checked, the core organizes headings, lists, quotes, and paragraphs; unchecked, it retains extracted text without report formatting. The print pipeline may flatten styling or structure before the core receives the job; the core cannot recreate information that is no longer present. Scanned PDFs require OCR, which remains on the roadmap.

## Native adapters

- Linux: CUPS receives print jobs. A per-user helper is needed for the native save dialog because CUPS backends run outside the logged-in desktop session.
- macOS: the current experimental package targets CUPS, but its post-install script cannot place a backend in `/usr/libexec/cups/backend` on a SIP-protected system. A supported printer registration design is still required.
- Windows: the supported direction is a Print Support virtual printer packaged as MSIX; its workflow must call the Rust core and use the OS save destination.
- Android: a native `PrintService` receives the system-spooled PDF. The user chooses the `.md` destination through Android's Storage Access Framework after tapping a notification; Rust conversion runs in-process through JNI.

The Android APK now builds in CI for ARM64 and x86_64, but it has not yet been validated on a physical device. The repository is still validating packaging and printer registration on desktop platforms; current installers are experimental.

## Privacy

Jobs are processed locally. Never execute extracted content. Adapters must validate formats and sizes and must not log document contents.
