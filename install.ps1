# install.ps1
<#
.SYNOPSIS
TreeClip Installer for Windows

.DESCRIPTION
Downloads and installs TreeClip from GitHub Releases
#>

param(
    [string]$InstallPath = "$env:USERPROFILE\bin",
    [switch]$Force,
    [switch]$Help
)

# -------------------------------------------- Public Functions --------------------------------------------

function Show-Help {
    @"
TreeClip Installer for Windows 🌳✨

Usage: .\install.ps1 [OPTIONS]

Options:
  -InstallPath PATH    Installation directory (default: ~\bin)
  -Force              Force overwrite existing installation
  -Help               Show this help message

Examples:
  .\install.ps1                         # Install to default location
  .\install.ps1 -InstallPath C:\Tools   # Install to custom location
  .\install.ps1 -Force                  # Force reinstall
"@
}

function Install-TreeClip {
    [CmdletBinding()]
    param(
        [string]$InstallPath,
        [bool]$Force
    )
    
    Write-Host "🌳 Installing TreeClip..." -ForegroundColor Cyan
    
    # Create installation directory
    if (-not (Test-Path $InstallPath)) {
        New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
        Write-Host "📁 Created directory: $InstallPath"
    }
    
    # Download latest release
    $DownloadUrl = "https://github.com/seyallius/treeclip.v2/releases/latest/download/treeclip-windows-x86_64.exe.zip"
    $TempFile = Join-Path $env:TEMP "treeclip-windows.zip"
    $TargetPath = Join-Path $InstallPath "treeclip.exe"
    
    # Check if already exists
    if ((Test-Path $TargetPath) -and (-not $Force)) {
        Write-Host "⚠️  TreeClip already exists at: $TargetPath" -ForegroundColor Yellow
        Write-Host "   Use -Force to overwrite"
        return
    }
    
    try {
        Write-Host "📦 Downloading TreeClip..."
        Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempFile -UseBasicParsing
        
        Write-Host "📂 Extracting..."
        Expand-Archive -Path $TempFile -DestinationPath $env:TEMP -Force
        $ExtractedPath = Join-Path $env:TEMP "treeclip-windows-x86_64.exe"
        
        if (Test-Path $ExtractedPath) {
            Copy-Item -Path $ExtractedPath -Destination $TargetPath -Force
            Write-Host "✅ Installed to: $TargetPath" -ForegroundColor Green
            
            # Add to PATH if not already
            Add-ToPath $InstallPath
        } else {
            Write-Host "❌ Extraction failed" -ForegroundColor Red
        }
    } catch {
        Write-Host "❌ Download failed: $_" -ForegroundColor Red
    } finally {
        # Cleanup
        if (Test-Path $TempFile) {
            Remove-Item $TempFile -Force -ErrorAction SilentlyContinue
        }
    }
}

# -------------------------------------------- Private Helper Functions --------------------------------------------

function Add-ToPath {
    param([string]$Path)
    
    $CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($CurrentPath -notmatch [regex]::Escape($Path)) {
        [Environment]::SetEnvironmentVariable("Path", "$Path;$CurrentPath", "User")
        Write-Host "📝 Added $Path to user PATH (requires restart)" -ForegroundColor Yellow
    }
}

# Main execution
if ($Help) {
    Show-Help
    exit 0
}

Install-TreeClip -InstallPath $InstallPath -Force:$Force
Write-Host "✨ Installation complete! Try: treeclip --help" -ForegroundColor Cyan