# Linux packages

The release pipeline produces an experimental Debian `.deb` for x86_64. It installs the CUPS backend, PPD, helper and a systemd user unit. The CUPS queue is registered by package maintainer scripts when CUPS is available. Other distributions can use the `.tar.gz` archive and `install.sh`.

Printer registration and desktop save-dialog behavior still require testing on supported distributions and desktop sessions.
