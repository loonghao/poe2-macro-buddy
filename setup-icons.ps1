# Setup POE2 Icons for Mouse Macro
Write-Host "==================================" -ForegroundColor Cyan
Write-Host "POE2 Icon Setup for Mouse Macro" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# Create icons directory
$iconsDir = "src-tauri\icons"
if (-not (Test-Path $iconsDir)) {
    New-Item -ItemType Directory -Path $iconsDir -Force | Out-Null
    Write-Host "✓ Created icons directory" -ForegroundColor Green
} else {
    Write-Host "✓ Icons directory exists" -ForegroundColor Green
}

Write-Host ""
Write-Host "Icon Download Options:" -ForegroundColor Yellow
Write-Host ""
Write-Host "Option 1: SteamGridDB (Recommended)" -ForegroundColor Cyan
Write-Host "  URL: https://www.steamgriddb.com/icon/69137" -ForegroundColor White
Write-Host "  - High quality POE2 icon" -ForegroundColor Gray
Write-Host "  - 256x256 ICO format" -ForegroundColor Gray
Write-Host "  - Free to download" -ForegroundColor Gray
Write-Host ""
Write-Host "Option 2: DeviantArt" -ForegroundColor Cyan
Write-Host "  Search: 'Path of Exile 2 ICO'" -ForegroundColor White
Write-Host "  - Various community-made icons" -ForegroundColor Gray
Write-Host ""

Write-Host "Manual Download Steps:" -ForegroundColor Yellow
Write-Host "1. Visit: https://www.steamgriddb.com/icon/69137" -ForegroundColor White
Write-Host "2. Click the 'DOWNLOAD' button" -ForegroundColor White
Write-Host "3. Save the file as 'icon.ico'" -ForegroundColor White
Write-Host "4. Move it to: $iconsDir\" -ForegroundColor White
Write-Host ""

# Try to download using PowerShell (may not work due to website protection)
Write-Host "Attempting automatic download..." -ForegroundColor Yellow

try {
    # Direct download URL (this is a guess, may need to be updated)
    $downloadUrl = "https://cdn2.steamgriddb.com/icon/c8f8e8e8e8e8e8e8e8e8e8e8e8e8e8e8.ico"
    $outputPath = "$iconsDir\icon.ico"
    
    # Note: This URL is a placeholder and likely won't work
    # The actual download requires clicking through the website
    Write-Host "Note: Automatic download may not work due to website protection" -ForegroundColor Yellow
    Write-Host "Please download manually from the link above" -ForegroundColor Yellow
} catch {
    Write-Host "Automatic download failed (expected)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "After downloading icon.ico:" -ForegroundColor Cyan
Write-Host "1. Place it in: $iconsDir\" -ForegroundColor White
Write-Host "2. Run: just build-gui" -ForegroundColor White
Write-Host ""

# Check if icon exists
if (Test-Path "$iconsDir\icon.ico") {
    Write-Host "✓ icon.ico found!" -ForegroundColor Green
    Write-Host ""
    Write-Host "You can now build the GUI installer:" -ForegroundColor Green
    Write-Host "  just build-gui" -ForegroundColor White
    Write-Host ""
    
    # Update tauri.conf.json to enable bundling
    Write-Host "Updating tauri.conf.json..." -ForegroundColor Yellow
    $configPath = "src-tauri\tauri.conf.json"
    if (Test-Path $configPath) {
        $config = Get-Content $configPath -Raw | ConvertFrom-Json
        $config.bundle.active = $true
        $config | ConvertTo-Json -Depth 100 | Set-Content $configPath
        Write-Host "✓ Enabled bundle in tauri.conf.json" -ForegroundColor Green
    }
} else {
    Write-Host "⚠ icon.ico not found" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "1. Download icon from: https://www.steamgriddb.com/icon/69137" -ForegroundColor White
    Write-Host "2. Save as: $iconsDir\icon.ico" -ForegroundColor White
    Write-Host "3. Run this script again" -ForegroundColor White
}

Write-Host ""
Write-Host "==================================" -ForegroundColor Cyan
Write-Host "Opening download page in browser..." -ForegroundColor Yellow
Start-Process "https://www.steamgriddb.com/icon/69137"

