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

# Keep MSVC output paths short when the release checkout is deeply nested.
$buildDirectory = [System.IO.Directory]::CreateTempSubdirectory('om-')
$targetDir = $buildDirectory.FullName
$tempRoot = $buildDirectory.Parent.FullName
$previousAppExe = $env:OPENMOUSE_GPUI_EXE
try {
    cargo build --manifest-path gpui/Cargo.toml --release --locked --target-dir $targetDir
    if ($LASTEXITCODE) { throw 'OpenMouse release build failed.' }
    $env:OPENMOUSE_GPUI_EXE = Join-Path $targetDir 'release/openmouse-gpui.exe'
    cargo build --manifest-path gpui/installer/Cargo.toml --release --locked --target-dir $targetDir
    if ($LASTEXITCODE) { throw 'OpenMouse installer build failed.' }
    New-Item -ItemType Directory -Path release -Force | Out-Null
    Copy-Item -LiteralPath (Join-Path $targetDir 'release/openmouse-installer.exe') -Destination release/OpenMouseDesktopSetup.exe -Force
}
finally {
    $env:OPENMOUSE_GPUI_EXE = $previousAppExe
    $cleanupTarget = Get-Item -LiteralPath $targetDir -Force -ErrorAction SilentlyContinue
    if ($cleanupTarget -is [System.IO.DirectoryInfo] -and
        $cleanupTarget.FullName -ceq $targetDir -and
        $cleanupTarget.Parent.FullName -ceq $tempRoot -and
        -not ($cleanupTarget.Attributes -band [System.IO.FileAttributes]::ReparsePoint)) {
        Remove-Item -LiteralPath $cleanupTarget.FullName -Recurse -Force -ErrorAction Continue
    }
}

node .github/scripts/make-release-files.mjs "v$version" release/OpenMouseDesktopSetup.exe $env:LOCAL_RELEASE_REPOSITORY release
if ($LASTEXITCODE) { throw 'OpenMouse release metadata generation failed.' }
