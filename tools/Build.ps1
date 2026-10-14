param(
    # Also archive the NRO, its ELF and a manifest with tools/package.py.
    [switch]$Package,
    # Artifact directory for -Package; keep it outside the checkout for releases.
    [string]$Output
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$cargoDirectory = Join-Path ([Environment]::GetFolderPath('UserProfile')) '.cargo/bin'
$env:PATH = $cargoDirectory + [IO.Path]::PathSeparator + $env:PATH
Push-Location -LiteralPath $root
try {
    & (Join-Path $cargoDirectory 'cargo') skyline build --release
    if ($LASTEXITCODE -ne 0) { throw "Skyline build failed: $LASTEXITCODE" }
    if ($Package) {
        $arguments = @((Join-Path $PSScriptRoot 'package.py'), '--root', $root)
        if ($Output) { $arguments += @('--output', $Output) }
        & /usr/bin/python3 @arguments
        if ($LASTEXITCODE -ne 0) { throw "Packaging failed: $LASTEXITCODE" }
    }
} finally {
    Pop-Location
}
