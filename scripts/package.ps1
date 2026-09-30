param([switch]$SkipBuild, [string]$IsccPath)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$projectRoot = Split-Path $PSScriptRoot -Parent
Push-Location $projectRoot
try {
    $versionMatch = [regex]::Match((Get-Content Cargo.toml -Raw), '(?m)^version\s*=\s*"([0-9]+\.[0-9]+\.[0-9]+)"')
    if (-not $versionMatch.Success) { throw 'Invalid package version' }
    $version = $versionMatch.Groups[1].Value
    if (-not $SkipBuild) {
        cargo build --release --locked
        if ($LASTEXITCODE -ne 0) { throw 'Release build failed' }
    }
    $targetRoot = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $projectRoot 'target' }
    $binary = Join-Path $targetRoot 'release/pz-workshop-downloader.exe'
    if (-not (Test-Path -LiteralPath $binary)) { throw 'Release binary not found' }
    if ([Diagnostics.FileVersionInfo]::GetVersionInfo($binary).FileVersion -ne $version) {
        throw 'Release binary version does not match Cargo.toml; rebuild before packaging'
    }
    if (-not $IsccPath) {
        $candidates = @(
            "$env:LOCALAPPDATA/Programs/Inno Setup 6/ISCC.exe",
            "${env:ProgramFiles(x86)}/Inno Setup 6/ISCC.exe",
            "$env:ProgramFiles/Inno Setup 6/ISCC.exe"
        )
        $IsccPath = $candidates | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
    }
    if (-not $IsccPath) { throw 'Install Inno Setup 6 or supply -IsccPath' }
    $dist = Join-Path $projectRoot 'dist'
    # Unique staging keeps stale files out of a release and avoids deleting a configured directory.
    $stage = Join-Path $dist ("stage-" + [guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Path $stage -Force | Out-Null
    Copy-Item -LiteralPath $binary -Destination (Join-Path $stage 'Workshop-Device.exe')
    foreach ($name in @('LICENSE','README.md','CONTRIBUTING.md','THIRD-PARTY-NOTICES.html')) {
        Copy-Item -LiteralPath (Join-Path $projectRoot $name) -Destination $stage
    }
    New-Item -ItemType Directory -Path (Join-Path $stage 'docs'),(Join-Path $stage 'assets') | Out-Null
    foreach ($name in @('README.pt-BR.md','README.ja.md','CONTRIBUTING.pt-BR.md','CONTRIBUTING.ja.md','DEPENDENCIES.md')) {
        Copy-Item -LiteralPath (Join-Path $projectRoot "docs/$name") -Destination (Join-Path $stage 'docs')
    }
    Copy-Item -LiteralPath (Join-Path $projectRoot 'assets/app-icon.png'),(Join-Path $projectRoot 'assets/IMAGE.md') -Destination (Join-Path $stage 'assets')
    & $IsccPath /Qp "/DAppVersion=$version" "/DPackageDir=$stage" "/O$dist" (Join-Path $projectRoot 'installer/workshop-device.iss')
    if ($LASTEXITCODE -ne 0) { throw 'Installer compilation failed' }
    $zip = Join-Path $dist "Workshop-Device-$version-windows-x64.zip"
    Compress-Archive -Path (Join-Path $stage '*') -DestinationPath $zip -Force
    $portable = Join-Path $dist 'Workshop-Device.exe'
    Copy-Item -LiteralPath (Join-Path $stage 'Workshop-Device.exe') -Destination $portable -Force
    $outputs = @($portable, $zip, (Join-Path $dist "Workshop-Device-Setup-$version.exe"))
    foreach ($name in @('LICENSE', 'THIRD-PARTY-NOTICES.html')) {
        $notice = Join-Path $dist $name
        Copy-Item -LiteralPath (Join-Path $projectRoot $name) -Destination $notice -Force
        $outputs += $notice
    }
    $lines = foreach ($file in $outputs) {
        $hash = Get-FileHash -LiteralPath $file -Algorithm SHA256
        "$($hash.Hash.ToLowerInvariant())  $([IO.Path]::GetFileName($file))"
    }
    $lines | Set-Content -LiteralPath (Join-Path $dist 'SHA256SUMS.txt') -Encoding utf8
    Write-Output "Release $version ready in $dist"
} finally {
    Pop-Location
}
