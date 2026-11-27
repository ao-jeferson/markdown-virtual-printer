# macOS installer package

The release pipeline produces an experimental `.pkg` for Apple silicon. It installs the command-line components and CUPS resources under `/usr/local`. It is not signed or notarized. The current post-install script tries to install the CUPS backend in `$(cups-config --serverbin)/backend`; on current macOS this resolves to `/usr/libexec/cups/backend`, which is protected by System Integrity Protection. As a result, the package does not register a working printer queue on this Mac. Do not treat this package as an installable printer until the macOS integration is redesigned and validated.

For public distribution without Gatekeeper warnings, the package needs an Apple Developer ID Installer identity and notarization credentials stored as GitHub Actions secrets. Never put signing identities or private keys in the repository.
