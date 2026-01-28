$ErrorActionPreference = "Stop"

$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Definition
$JS_SCRIPT = Join-Path $SCRIPT_DIR "..\..\assets\font\subset_font.js"
# Resolve project root (two levels up from build/scripts)
$PROJECT_ROOT = Resolve-Path (Join-Path $SCRIPT_DIR "..\..")

Write-Host "Starting font subsetting process..." -ForegroundColor Cyan

# 1. Check for Node.js and npm
if ((-not (Get-Command node -ErrorAction SilentlyContinue)) -or (-not (Get-Command npm -ErrorAction SilentlyContinue))) {
    Write-Host "Error: Node.js and npm are required but not found." -ForegroundColor Red
    Write-Host "Please install Node.js from https://nodejs.org/"
    exit 1
}

# 2. Check for fontmin dependency
Write-Host "Checking for fontmin dependency..."
# 2. Check for fontmin dependency
Write-Host "Checking for fontmin dependency..."
$fontminInstalled = $false

Push-Location $PROJECT_ROOT
try {
    # Direct execution avoids Start-Process argument parsing headaches
    node -e "try { require('fontmin') } catch (e) { process.exit(1) }"
    if ($LASTEXITCODE -eq 0) {
        $fontminInstalled = $true
    }
}
finally {
    Pop-Location
}

if (-not $fontminInstalled) {
    Write-Host "Error: 'fontmin' dependency is missing." -ForegroundColor Red
    Write-Host "To fix this:" -ForegroundColor Yellow
    Write-Host "1. Open PowerShell or CMD as Administrator"
    Write-Host "2. Navigate to the project root: $PROJECT_ROOT"
    Write-Host "3. Run command: npm install fontmin --save-dev" -ForegroundColor White
    exit 1
}

Write-Host "Dependency check passed. Running subset_font.js..."
node $JS_SCRIPT

if ($LASTEXITCODE -eq 0) {
    Write-Host "Font subsetting finished successfully." -ForegroundColor Green
}
else {
    Write-Host "Font subsetting failed." -ForegroundColor Red
    exit 1
}
