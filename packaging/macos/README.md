# macOS installer package

The release pipeline produces an experimental `.pkg` for Apple silicon. It installs the command-line components and CUPS resources under `/usr/local`. It is not signed or notarized, and the current CUPS adapter has not been validated against current macOS printer registration policies. Users must review the package and follow its post-install instructions.

For public distribution without Gatekeeper warnings, the package needs an Apple Developer ID Installer identity and notarization credentials stored as GitHub Actions secrets. Never put signing identities or private keys in the repository.
