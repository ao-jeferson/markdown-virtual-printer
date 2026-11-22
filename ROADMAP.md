# Roadmap

This roadmap describes product outcomes, not a development diary. Order is intentional; there are no promised dates.

[Português (Brasil)](ROADMAP.pt-BR.md)

## 1. Shared Rust core

- [ ] Define the canonical document model and conversion interfaces.
- [ ] Convert HTML and plain text into structured Markdown.
- [ ] Add PDF and print-job adapters with explicit conversion warnings.
- [ ] Add report templates for chat responses and other structured content.
- [ ] Package the core without requiring a Python installation or separate Python runtime.

## 2. macOS

- [ ] Build the native desktop helper and save dialog.
- [ ] Register a virtual printer through a supported macOS printing path.
- [ ] Pass print jobs safely from the printing service to the user-session app.
- [ ] Provide a signed installer and a clean uninstall path.
- [ ] Validate jobs from representative applications and document conversion limits.

## 3. Windows

- [ ] Implement the virtual printer with Microsoft's supported Print Support App architecture.
- [ ] Connect the Windows print workflow to the shared Rust core.
- [ ] Package and install through the supported Windows application mechanism.
- [ ] Validate the native print dialog, save flow, updates, and removal.

## 4. Linux

- [ ] Implement CUPS integration and a desktop save helper.
- [ ] Select initial distributions and desktop environments to support.
- [ ] Package installation and removal without leaving a broken print queue.

## 5. Android and iOS — no target date

- [ ] Revisit feasibility after desktop workflows and the shared core are stable.
- [ ] Choose platform-specific print integration only after confirming OS capabilities and user demand.

## 6. Release readiness

- [ ] Publish signed, self-contained platform installers.
- [ ] Document supported OS versions, permissions, upgrades, and removal.
- [ ] Automate release builds and provide checksums.