<#
.SYNOPSIS
    Consolidated i18n cleanup script.
    1. Removes unused keys from assets/i18n/en.toml based on code analysis.
    2. Synchronizes all other *.toml files with en.toml by removing extra keys.

.USAGE
    .\build\scripts\i18n_clean.ps1
#>

$ErrorActionPreference = "Stop"

# -----------------------------------------------------------------------------
# 1. Path Configuration
# -----------------------------------------------------------------------------
$ScriptDir = $PSScriptRoot
# Go up two levels from build/scripts/ to project root
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
$I18nDir = Join-Path $ProjectRoot "assets\i18n"
$BaseFile = Join-Path $I18nDir "en.toml"
$SrcDir = Join-Path $ProjectRoot "src"

Write-Host "`n=== i18n Consolidated Cleanup & Sync Tool ===" -ForegroundColor Cyan
Write-Host "Project Root: $ProjectRoot"
Write-Host "Base File   : $BaseFile"
Write-Host "Source Dir  : $SrcDir"
Write-Host "=============================================`n"

if (-not (Test-Path $BaseFile)) {
    Write-Error "Error: Base file 'en.toml' not found at $BaseFile"
}

# -----------------------------------------------------------------------------
# 2. Helper Functions
# -----------------------------------------------------------------------------
function Get-TomlKeys {
    param (
        [string]$FilePath
    )
    $Keys = @()
    if (-not (Test-Path $FilePath)) { return $Keys }
    
    $Lines = Get-Content $FilePath -Encoding UTF8
    $CurrentSection = ""

    foreach ($Line in $Lines) {
        $Trimmed = $Line.Trim()
        if ([string]::IsNullOrWhiteSpace($Trimmed) -or $Trimmed.StartsWith("#")) { continue }

        # Match Section headers: [section_name]
        if ($Trimmed -match '^\[(.+)\]$') {
            $CurrentSection = $Matches[1]
        } 
        # Match Keys: key_name = "value"
        elseif ($Trimmed -match '^([a-zA-Z0-9_\-]+)\s*=') {
            $KeyName = $Matches[1]
            $FullKey = if ($CurrentSection) { "$CurrentSection.$KeyName" } else { $KeyName }
            $Keys += $FullKey
        }
    }
    return $Keys
}

# -----------------------------------------------------------------------------
# PHASE 1: Clean Unused Keys from en.toml
# -----------------------------------------------------------------------------
Write-Host "--- Phase 1: Cleaning unused keys from en.toml ---" -ForegroundColor Magenta

# 1.1 Parse Keys from en.toml
$EnKeys = Get-TomlKeys -FilePath $BaseFile
Write-Host "Found $($EnKeys.Count) translation keys in en.toml."

# 1.2 Scan Source Code
Write-Host "Scanning source files in '$SrcDir'..."
$SourceFiles = Get-ChildItem -Path $SrcDir -Recurse -File
$CombinedContentBuilder = [System.Text.StringBuilder]::new()
foreach ($File in $SourceFiles) {
    try {
        [void]$CombinedContentBuilder.Append([System.IO.File]::ReadAllText($File.FullName))
    }
    catch {
        Write-Warning "Could not read file: $($File.FullName)"
    }
}
$CombinedContent = $CombinedContentBuilder.ToString()

# 1.3 Identify Unused Keys
$UnusedKeys = @()
foreach ($Key in $EnKeys) {
    if (-not $CombinedContent.Contains($Key)) {
        $UnusedKeys += $Key
    }
}

if ($UnusedKeys.Count -gt 0) {
    Write-Host "Found $($UnusedKeys.Count) unused keys in en.toml:" -ForegroundColor Yellow
    $UnusedKeys | ForEach-Object { Write-Host "  [-] $_" -ForegroundColor Red }

    # 1.4 Remove from en.toml
    $RawLines = Get-Content $BaseFile -Encoding UTF8
    $NewContent = @()
    $CurrentSection = ""
    $DeletedCount = 0

    foreach ($Line in $RawLines) {
        $Trimmed = $Line.Trim()
        $Keep = $true

        if ($Trimmed -match '^\[(.+)\]$') {
            $CurrentSection = $Matches[1]
        }
        elseif ($Trimmed -match '^([a-zA-Z0-9_\-]+)\s*=') {
            $KeyName = $Matches[1]
            $FullKey = if ($CurrentSection) { "$CurrentSection.$KeyName" } else { $KeyName }
            if ($FullKey -in $UnusedKeys) {
                $Keep = $false
                $DeletedCount++
            }
        }
        if ($Keep) { $NewContent += $Line }
    }
    [System.IO.File]::WriteAllLines($BaseFile, $NewContent, [System.Text.Encoding]::UTF8)
    Write-Host "Successfully removed $DeletedCount unused keys from en.toml." -ForegroundColor Green
}
else {
    Write-Host "All keys in en.toml are currently in use." -ForegroundColor Green
}

# -----------------------------------------------------------------------------
# PHASE 2: Synchronize Other Language Files with en.toml
# -----------------------------------------------------------------------------
Write-Host "`n--- Phase 2: Synchronizing other language files ---" -ForegroundColor Magenta

# 2.1 Get Fresh Base Keys (after cleanup)
$EnKeysAfter = Get-TomlKeys -FilePath $BaseFile
$BaseKeySet = [System.Collections.Generic.HashSet[string]]::new()
foreach ($k in $EnKeysAfter) { [void]$BaseKeySet.Add($k) }
Write-Host "Base keys count: $($BaseKeySet.Count)"

# 2.2 Process Other TOML files
$TargetFiles = Get-ChildItem -Path $I18nDir -Filter "*.toml"
$TotalRemoved = 0

foreach ($File in $TargetFiles) {
    if ($File.Name -eq "en.toml") { continue }

    Write-Host "Checking $($File.Name)..." -NoNewline
    $FileKeys = Get-TomlKeys -FilePath $File.FullName
    $ExtraKeys = @()
    foreach ($K in $FileKeys) {
        if (-not $BaseKeySet.Contains($K)) { $ExtraKeys += $K }
    }

    if ($ExtraKeys.Count -eq 0) {
        Write-Host " OK" -ForegroundColor Green
        continue
    }

    Write-Host " Found $($ExtraKeys.Count) extra keys." -ForegroundColor Yellow
    
    # 2.3 Remove extra keys
    $Lines = Get-Content $File.FullName -Encoding UTF8
    $NewFileContent = [System.Collections.Generic.List[string]]::new()
    $CurrentSection = ""
    $FileDeletedCount = 0

    foreach ($Line in $Lines) {
        $Trimmed = $Line.Trim()
        $Keep = $true

        if ($Trimmed -match '^\[(.+)\]$') {
            $CurrentSection = $Matches[1]
        }
        elseif ($Trimmed -match '^([a-zA-Z0-9_\-]+)\s*=') {
            $KeyName = $Matches[1]
            $FullKey = if ($CurrentSection) { "$CurrentSection.$KeyName" } else { $KeyName }
            if ($FullKey -notin $EnKeysAfter) {
                $Keep = $false
                Write-Host "  [-] Removes: $($File.Name):$FullKey" -ForegroundColor Red
                $FileDeletedCount++
            }
        }
        if ($Keep) { $NewFileContent.Add($Line) }
    }
    [System.IO.File]::WriteAllLines($File.FullName, $NewFileContent, [System.Text.Encoding]::UTF8)
    $TotalRemoved += $FileDeletedCount
}

Write-Host "`n============================================="
Write-Host "[Complete] i18n Cleanup and Sync Finished." -ForegroundColor Cyan
Write-Host "Total Phase 2 labels removed: $TotalRemoved" -ForegroundColor Green
