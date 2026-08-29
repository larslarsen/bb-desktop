$ErrorActionPreference = 'Stop'

$ProjectRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
Set-Location $ProjectRoot

$ElectronPackage = Join-Path $ProjectRoot 'node_modules/electron'
$ElectronDist = Join-Path $ElectronPackage 'dist'
$ElectronExecutable = Join-Path $ElectronDist 'electron.exe'
if (-not (Test-Path (Join-Path $ElectronPackage 'install.js'))) {
  throw 'Electron is not installed. Run npm ci first.'
}
if (-not (Test-Path $ElectronExecutable)) {
  & node (Join-Path $ElectronPackage 'install.js')
  if ($LASTEXITCODE -ne 0) { throw 'Downloading Electron failed.' }
}

$Version = (& node -p "require('./package.json').version").Trim()
$NodeArch = (& node -p 'process.arch').Trim()
if ($NodeArch -notin @('x64', 'arm64')) {
  throw "Unsupported Windows architecture: $NodeArch"
}

$DistDir = Join-Path $ProjectRoot 'dist'
$BundleDir = Join-Path $DistDir "BitBook-windows-$NodeArch"
$ZipPath = Join-Path $DistDir "BitBook-$Version-windows-$NodeArch-unsigned.zip"
if (Test-Path $BundleDir) { Remove-Item -LiteralPath $BundleDir -Recurse -Force }
if (Test-Path $ZipPath) { Remove-Item -LiteralPath $ZipPath -Force }
New-Item -ItemType Directory -Force -Path $BundleDir | Out-Null

Copy-Item -Path (Join-Path $ElectronDist '*') -Destination $BundleDir -Recurse -Force
Rename-Item -LiteralPath (Join-Path $BundleDir 'electron.exe') -NewName 'BitBook.exe'
$DefaultApp = Join-Path $BundleDir 'resources/default_app.asar'
if (Test-Path $DefaultApp) { Remove-Item -LiteralPath $DefaultApp -Force }

$AppSource = Join-Path $BundleDir 'resources/app'
New-Item -ItemType Directory -Force -Path (Join-Path $AppSource 'imgs') | Out-Null
$RuntimePackage = (Get-Content -Raw packaging/runtime-package.json.in).Replace('@VERSION@', $Version)
$Utf8NoBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText((Join-Path $AppSource 'package.json'), $RuntimePackage, $Utf8NoBom)
Copy-Item social-main.js (Join-Path $AppSource 'social-main.js')
Copy-Item social (Join-Path $AppSource 'social') -Recurse
Copy-Item imgs/icon.png (Join-Path $AppSource 'imgs/icon.png')

Compress-Archive -LiteralPath $BundleDir -DestinationPath $ZipPath -CompressionLevel Optimal
Write-Output "Built $ZipPath"
