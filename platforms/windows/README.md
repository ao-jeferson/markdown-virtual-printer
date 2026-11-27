# Windows Print Support virtual printer

Windows adapter implementation status: **experimental; CI builds and packages the app as a self-signed MSIX**.

This prototype uses Microsoft's Print Support virtual printer workflow, packaged as a UWP/MSIX app. It declares `.md` as its output type so Windows displays its Save As dialog. It receives OXPS, converts to PDF with the Windows print workflow API, calls `mdvp_convert_job` in the Rust DLL, and writes the selected `.md` target. The PDC declares **Keep formatting** as a printer option.

The release ZIP contains the MSIX, its public self-signed publisher certificate, and an install script. Extract the ZIP and run the script in PowerShell to trust that certificate for the current user and install the MSIX. Only use the certificate shipped with the matching release. This development certificate is not a production publisher identity. The app has not yet been validated on a Windows 11 24H2 or newer host, so printer registration and end-to-end print behavior remain experimental.
