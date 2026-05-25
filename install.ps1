# install.ps1 — Windows installer for rSIDlet
param(
    [string]$Version = ""
)

$ErrorActionPreference = "Stop"

# --- Configuration ---
$Owner   = "SidneyLYZhang"
$Repo    = "rSIDlet"
$Name    = "rsidlet"
$Binary  = "sidlet.exe"
$InstallDir = if ($env:RSIDLET_INSTALL_DIR) {
    $env:RSIDLET_INSTALL_DIR
} else {
    "$env:LOCALAPPDATA\Programs\rsidlet"
}

# --- Resolve version ---
if (-not $Version) {
    Write-Host "Fetching latest release tag..."
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Owner/$Repo/releases/latest"
    $Version = $release.tag_name -replace '^v', ''
    if (-not $Version) {
        Write-Error "Failed to fetch latest version from GitHub API."
        exit 1
    }
}
$Version = $Version -replace '^v', ''

$Target = "win64"
$Ext    = "zip"

Write-Host "Installing $Name v$Version (Windows → $Target)..."

# --- Download ---
$Archive = "$Name-$Version-$Target.$Ext"
$Url     = "https://github.com/$Owner/$Repo/releases/download/v$Version/$Archive"

$TmpDir = Join-Path $env:TEMP "rsidlet-install-$PID"
New-Item -ItemType Directory -Force -Path $TmpDir | Out-Null

try {
    $ArchivePath = Join-Path $TmpDir $Archive
    Write-Host "Downloading $Url..."
    Invoke-WebRequest -Uri $Url -OutFile $ArchivePath

    # --- Extract ---
    Write-Host "Extracting..."
    Expand-Archive -Path $ArchivePath -DestinationPath $TmpDir -Force

    # --- Locate files in the extracted tree ---
    $srcBinary = Get-ChildItem -Path $TmpDir -Recurse -Name "$Binary" | Select-Object -First 1
    if (-not $srcBinary) {
        Write-Error "Binary '$Binary' not found in the archive."
        exit 1
    }
    $srcFonts = Get-ChildItem -Path $TmpDir -Recurse -Directory -Name "fonts" | Select-Object -First 1

    # --- Install ---
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

    Copy-Item -Path (Join-Path $TmpDir $srcBinary) -Destination (Join-Path $InstallDir $Binary) -Force

    if ($srcFonts) {
        $destFonts = Join-Path $InstallDir "fonts"
        if (Test-Path $destFonts) { Remove-Item -Recurse -Force $destFonts }
        Copy-Item -Recurse -Path (Join-Path $TmpDir $srcFonts) -Destination $destFonts
    }

    Write-Host "Installed to $InstallDir"

    # --- Add to user PATH ---
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if (-not $userPath) { $userPath = "" }
    if ($userPath -notlike "*$InstallDir*") {
        Write-Host ""
        Write-Host "Adding $InstallDir to user PATH..."
        [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallDir", "User")
        $env:Path = "$env:Path;$InstallDir"
        Write-Host "PATH updated. Restart your terminal for the change to take full effect."
    }

} finally {
    Remove-Item -Recurse -Force $TmpDir -ErrorAction SilentlyContinue
}

Write-Host "Done! Run 'sidlet --help' to get started."
