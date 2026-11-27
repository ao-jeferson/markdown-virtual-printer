param(
    [Parameter(Mandatory = $true)]
    [string]$PackagePath,
    [Parameter(Mandatory = $true)]
    [string]$CertificatePath
)

$ErrorActionPreference = 'Stop'

if (-not (Test-Path -LiteralPath $PackagePath)) {
    throw "MSIX package not found: $PackagePath"
}
if (-not (Test-Path -LiteralPath $CertificatePath)) {
    throw "Publisher certificate not found: $CertificatePath"
}

$certificate = [System.Security.Cryptography.X509Certificates.X509Certificate2]::new($CertificatePath)
$manifestPath = Join-Path $PSScriptRoot 'AppxManifest.xml'
if (Test-Path -LiteralPath $manifestPath) {
    [xml]$manifest = Get-Content -LiteralPath $manifestPath
    $publisher = $manifest.Package.Identity.Publisher
    if ($certificate.Subject -ne $publisher) {
        throw "Certificate subject '$($certificate.Subject)' does not match package publisher '$publisher'."
    }
}

$store = New-Object System.Security.Cryptography.X509Certificates.X509Store('TrustedPeople', 'CurrentUser')
$store.Open([System.Security.Cryptography.X509Certificates.OpenFlags]::ReadWrite)
try {
    $store.Add($certificate)
} finally {
    $store.Close()
}

Add-AppxPackage -Path (Resolve-Path -LiteralPath $PackagePath).Path
Write-Host 'Markdown Virtual Printer was installed for the current Windows user.'
