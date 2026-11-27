# Roadmap

GitHub Issues and the public project board track individual work items.

## In progress

- Shared Rust core: text and PDF input, two Markdown output modes, native adapter C ABI.
- Linux: CUPS backend, per-user save helper, PPD setting, and install script prototype; installation needs validation on a clean Linux host.
- macOS: Rust helper and CUPS backend compile, but native printer registration/package installation is not yet implemented.
- Windows: Print Support virtual printer MSIX source prototype exists; package build and printer workflow have not yet been validated on Windows.
- Android: native PrintService prototype is being added; device installation and print-job flow still require validation.

## Planned

- Validate and package Linux CUPS installation and save flow.
- Implement/test the macOS native printer registration and installer.
- Implement the Windows Print Support virtual printer MSIX and installer.
- Preserve document structure where the OS print job provides it.
- Evaluate OCR for image-based and scanned print jobs.
- Add report formatting options for chat output.
- Validate Android print-service registration, notification-to-save flow, and Rust JNI conversion on a device.

## Later

- iOS support, without a target date.
