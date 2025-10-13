@echo off
REM Install Transmutation dependencies on Windows
REM Uses winget (pre-installed on Windows 10/11)

echo ╔════════════════════════════════════════╗
echo ║  📦 Transmutation Dependencies        ║
echo ║     Windows (winget)                  ║
echo ╚════════════════════════════════════════╝
echo.

REM Check if running as Administrator
net session >nul 2>&1
if %errorLevel% == 0 (
    echo ✓ Running as Administrator
) else (
    echo ❌ This script must be run as Administrator!
    echo.
    echo Right-click this file and select "Run as Administrator"
    pause
    exit /b 1
)

echo.
echo 📥 Installing ALL dependencies for ALL features...
echo.

REM Check if winget is available
where winget >nul 2>&1
if %errorLevel% neq 0 (
    echo ❌ winget not found!
    echo.
    echo winget is included in Windows 10 1809+ and Windows 11
    echo Install "App Installer" from Microsoft Store or use install-deps-windows.ps1 with Chocolatey
    echo.
    pause
    exit /b 1
)

echo [1/6] Installing Visual Studio Build Tools...
winget install --id Microsoft.VisualStudio.2022.BuildTools --silent --accept-package-agreements --accept-source-agreements
if %errorLevel% neq 0 echo   ⚠️ Build Tools installation may require manual confirmation

echo.
echo [2/6] Installing CMake and Git...
winget install --id Kitware.CMake --silent --accept-package-agreements --accept-source-agreements
winget install --id Git.Git --silent --accept-package-agreements --accept-source-agreements

echo.
echo [3/6] Installing Poppler (PDF → Image)...
REM Poppler não tem pacote oficial no winget, baixar manualmente
echo   ⚠️ Poppler must be installed manually:
echo   1. Download: https://github.com/oschwartz10612/poppler-windows/releases/latest
echo   2. Extract to C:\Program Files\poppler\
echo   3. Add C:\Program Files\poppler\Library\bin\ to PATH
echo   Or use: choco install poppler

echo.
echo [4/6] Installing LibreOffice (Office formats)...
winget install --id TheDocumentFoundation.LibreOffice --silent --accept-package-agreements --accept-source-agreements

echo.
echo [5/6] Installing Tesseract (OCR)...
winget install --id UB-Mannheim.TesseractOCR --silent --accept-package-agreements --accept-source-agreements

echo.
echo [6/6] Installing FFmpeg (Audio/Video)...
winget install --id Gyan.FFmpeg --silent --accept-package-agreements --accept-source-agreements

echo.
echo ╔════════════════════════════════════════╗
echo ║  ✅ Installation Complete!            ║
echo ╚════════════════════════════════════════╝
echo.
echo 📊 Installed tools:
echo   ✓ Visual Studio Build Tools
echo   ✓ CMake ^& Git
echo   ⚠️ Poppler (manual installation required)
echo   ✓ LibreOffice
echo   ✓ Tesseract OCR
echo   ✓ FFmpeg
echo.
echo ⚠️  IMPORTANT: Restart your terminal/PowerShell to apply PATH changes
echo.
echo 🚀 After restart, you can run:
echo    transmutation convert document.pdf --format png
echo    transmutation convert document.docx -o output.md
echo.
echo 📝 For Poppler installation, see: transmutation\install\README.md
echo.
pause

