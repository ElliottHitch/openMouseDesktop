$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true
Set-Location (Split-Path -Parent $PSScriptRoot)
if (!$IsWindows) { throw 'OpenMouse Desktop release packaging requires Windows x64.' }
$version = $env:LOCAL_RELEASE_VERSION
if (!$version -or !$env:LOCAL_RELEASE_REPOSITORY) { throw 'The local release version and repository are required.' }
foreach ($path in @('gpui/Cargo.toml', 'gpui/installer/Cargo.toml')) {
    $manifest = Get-Content -LiteralPath $path -Raw
    if ($manifest -notmatch '(?m)^version\s*=\s*"([^"]+)"' -or $Matches[1] -ne $version) {
        throw "LOCAL_RELEASE_VERSION must match the package version in $path."
    }
}
npm ci
if ($LASTEXITCODE) { throw 'OpenMouse JavaScript dependency installation failed.' }
$env:CARGO_TARGET_DIR = Join-Path $PWD 'gpui/target'
cargo build --manifest-path gpui/Cargo.toml --release --locked
if ($LASTEXITCODE) { throw 'OpenMouse release build failed.' }
$env:CARGO_TARGET_DIR = Join-Path $PWD 'gpui/installer/target'
cargo build --manifest-path gpui/installer/Cargo.toml --release --locked
if ($LASTEXITCODE) { throw 'OpenMouse installer build failed.' }
New-Item -ItemType Directory -Path release -Force | Out-Null
Copy-Item -LiteralPath gpui/installer/target/release/openmouse-installer.exe -Destination release/OpenMouseDesktopSetup.exe -Force
node .github/scripts/make-release-files.mjs "v$version" release/OpenMouseDesktopSetup.exe $env:LOCAL_RELEASE_REPOSITORY release
if ($LASTEXITCODE) { throw 'OpenMouse release metadata generation failed.' }
