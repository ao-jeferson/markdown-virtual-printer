# Windows Print Support virtual printer

Windows adapter implementation status: **prototype source; not yet built or validated on Windows**.

This prototype uses Microsoft's Print Support virtual printer workflow, packaged as a UWP/MSIX app. It declares `.md` as its output type so Windows displays its Save As dialog. It receives OXPS, converts to PDF with the Windows print workflow API, calls `mdvp_convert_job` in the Rust DLL, and writes the selected `.md` target. The PDC declares **Keep formatting** as a printer option.

This path requires a Windows 11 24H2 or newer validation host, Windows SDK/Visual Studio packaging tools, and package signing for distribution. The Rust core and the app package must both be built and tested on Windows before an installable release can be claimed. The release workflow currently uploads the Windows core library only; the app package is built by CI but not distributed as an installer.
