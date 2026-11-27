# Changelog

User-facing changes are documented here for each published release. See GitHub Releases for version notes.

## 0.1.1

- Add experimental native installer packages to the release pipeline: Debian `.deb`, macOS `.pkg`, and Windows MSIX.
- Build installer packages in pull request workflows and publish them from tagged releases.
- Windows package uses an ephemeral self-signed certificate; macOS package is unsigned and not notarized.
- Printer registration and real print flows still require validation on supported OS versions.
