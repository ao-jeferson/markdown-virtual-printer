# Release Process

[Português (Brasil)](releases.pt-BR.md)

## Versioning and release notes

Use Semantic Versioning as compatibility stabilizes: MAJOR.MINOR.PATCH. Before 1.0.0, incompatible changes may occur in minor releases and must be called out in the release notes.

Each release should summarize features, fixes, known limitations, installation and upgrade instructions, supported operating systems, and downloadable artifacts. Keep CHANGELOG.md focused on released product changes; do not use it as a development diary.

## Gitflow release steps

1. Review merged Issues and the roadmap items included in the release.
2. Create release/X.Y.Z from develop and prepare the version, changelog, translations, and documentation there.
3. Open a pull request from the release branch to main and merge after review.
4. Create the annotated vX.Y.Z tag on the merged main commit.
5. Publish the GitHub Release with generated notes and artifacts.
6. Open a pull request to bring release-only changes back into develop.

## Current automation

The existing GitHub Actions workflow builds the experimental Python package when a v-prefixed tag is pushed. It does not yet build the planned native installers. The Rust workspace and per-platform packaging will replace this prototype workflow as those deliverables become available.

Native releases should provide signed, self-contained installers and checksums. Do not claim that an installer is available for a platform until its print integration and install/uninstall flows have been validated.