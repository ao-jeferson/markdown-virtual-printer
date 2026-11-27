# Roadmap

GitHub Issues and the public project board track individual work items.

## In progress

- Shared Rust core: text and PDF input, two Markdown output modes, native adapter C ABI.
- Linux: CUPS backend, per-user save helper, PPD setting, and install script prototype; installation needs validation on a clean Linux host.
- macOS: an experimental `.pkg` exists, but it cannot register the queue on this Mac. Its post-install script tries to copy the backend into `/usr/libexec/cups/backend`, which is protected by System Integrity Protection. A supported macOS printer integration and installer are still needed.
- Windows: Print Support virtual printer MSIX source prototype exists; package build and printer workflow have not yet been validated on Windows.
- Android: native PrintService app and CI APK build are implemented for ARM64 and x86_64; installation and print-job flow still need device validation.

## Planned

- Validate and package Linux CUPS installation and save flow.
- Implement and validate a macOS printer integration that does not require writing to SIP-protected CUPS directories.
- Implement the Windows Print Support virtual printer MSIX and installer.
- Preserve document structure where the OS print job provides it.
- Evaluate OCR for image-based and scanned print jobs.
- Add report formatting options for chat output.
- Validate Android print-service registration, notification-to-save flow, and Rust JNI conversion on a device.

## Later

- iOS support, without a target date.
