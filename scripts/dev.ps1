# Development helper only. cb and packaged apps do not use this script.
param([Parameter(ValueFromRemainingArguments = $true)][string[]]$CargoArguments)
$ErrorActionPreference = 'Stop'
if (-not $CargoArguments) { $CargoArguments = @('test', '--offline') }
$projectRoot = Split-Path -Parent $PSScriptRoot
$toolchain = Join-Path $projectRoot 'target\rustup-local\toolchains\stable-x86_64-pc-windows-gnullvm'
$names = @('RUSTC', 'RUSTDOC', 'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CARGO_TARGET_DIR')
$saved = @{}
foreach ($name in $names) { $saved[$name] = [Environment]::GetEnvironmentVariable($name, 'Process') }
Push-Location $projectRoot
try {
    if (Test-Path -LiteralPath (Join-Path $toolchain 'bin\cargo.exe')) {
        $env:RUSTC = Join-Path $toolchain 'bin\rustc.exe'
        $env:RUSTDOC = Join-Path $toolchain 'bin\rustdoc.exe'
        $linker = Join-Path $toolchain 'lib\rustlib\x86_64-pc-windows-gnullvm\bin\rust-lld.exe'
        $env:CARGO_ENCODED_RUSTFLAGS = @('-C', 'target-feature=+crt-static', '-C', "linker=$linker") -join [char]31
        $env:CARGO_TARGET_DIR = Join-Path $projectRoot 'target\native'
        & (Join-Path $toolchain 'bin\cargo.exe') @CargoArguments
    } else {
        & cargo @CargoArguments
    }
    $result = $LASTEXITCODE
} finally {
    foreach ($name in $names) { [Environment]::SetEnvironmentVariable($name, $saved[$name], 'Process') }
    Pop-Location
}
exit $result
