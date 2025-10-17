@echo off
REM Install Transmutation dependencies on Windows (Manual Download Method)
REM Works on any Windows version without package managers

setlocal enabledelayedexpansion

echo ╔════════════════════════════════════════╗
echo ║  📦 Transmutation Dependencies        ║
echo ║     Windows (Manual Download)         ║
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
echo 📥 Downloading and installing dependencies...
echo.

REM Create temporary download directory
set TEMP_DIR=%TEMP%\transmutation_install
if not exist "%TEMP_DIR%" mkdir "%TEMP_DIR%"
cd /d "%TEMP_DIR%"

echo [1/6] Visual Studio Build Tools
echo   Please install manually from:
echo   https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
echo   Select "Desktop development with C++"
start https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
timeout /t 3 >nul

echo.
echo [2/6] CMake
echo   Downloading CMake...
set CMAKE_URL=https://github.com/Kitware/CMake/releases/download/v3.28.1/cmake-3.28.1-windows-x86_64.msi
curl -L -o cmake-installer.msi "%CMAKE_URL%"
if exist cmake-installer.msi (
    echo   Installing CMake...
    msiexec /i cmake-installer.msi /quiet /norestart ADD_CMAKE_TO_PATH=System
    timeout /t 5 >nul
) else (
    echo   ⚠️ Download failed, please install manually from:
    echo   https://cmake.org/download/
)

echo.
echo [3/6] Git
echo   Downloading Git...
set GIT_URL=https://github.com/git-for-windows/git/releases/download/v2.43.0.windows.1/Git-2.43.0-64-bit.exe
curl -L -o git-installer.exe "%GIT_URL%"
if exist git-installer.exe (
    echo   Installing Git...
    git-installer.exe /VERYSILENT /NORESTART /NOCANCEL /SP- /CLOSEAPPLICATIONS /RESTARTAPPLICATIONS /COMPONENTS="icons,ext\reg\shellhere,assoc,assoc_sh"
    timeout /t 5 >nul
) else (
    echo   ⚠️ Download failed, please install manually from:
    echo   https://git-scm.com/download/win
)

echo.
echo [4/6] Poppler (PDF tools)
echo   Downloading Poppler...
set POPPLER_URL=https://github.com/oschwartz10612/poppler-windows/releases/latest/download/Release-23.08.0-0.zip
curl -L -o poppler.zip "%POPPLER_URL%"
if exist poppler.zip (
    echo   Extracting Poppler...
    powershell -Command "Expand-Archive -Path poppler.zip -DestinationPath 'C:\Program Files\poppler\' -Force"
    echo   Adding to PATH...
    setx /M PATH "%PATH%;C:\Program Files\poppler\Library\bin"
) else (
    echo   ⚠️ Download failed, please install manually from:
    echo   https://github.com/oschwartz10612/poppler-windows/releases
)

echo.
echo [5/6] LibreOffice
echo   Downloading LibreOffice...
set LIBREOFFICE_URL=https://download.documentfoundation.org/libreoffice/stable/7.6.4/win/x86_64/LibreOffice_7.6.4_Win_x86-64.msi
curl -L -o libreoffice-installer.msi "%LIBREOFFICE_URL%"
if exist libreoffice-installer.msi (
    echo   Installing LibreOffice...
    msiexec /i libreoffice-installer.msi /quiet /norestart
    timeout /t 10 >nul
) else (
    echo   ⚠️ Download failed, please install manually from:
    echo   https://www.libreoffice.org/download/download/
)

echo.
echo [6/6] Tesseract OCR
echo   Downloading Tesseract...
set TESSERACT_URL=https://digi.bib.uni-mannheim.de/tesseract/tesseract-ocr-w64-setup-5.3.3.20231005.exe
curl -L -o tesseract-installer.exe "%TESSERACT_URL%"
if exist tesseract-installer.exe (
    echo   Installing Tesseract...
    tesseract-installer.exe /S /D=C:\Program Files\Tesseract-OCR
    timeout /t 5 >nul
    setx /M PATH "%PATH%;C:\Program Files\Tesseract-OCR"
) else (
    echo   ⚠️ Download failed, please install manually from:
    echo   https://github.com/UB-Mannheim/tesseract/wiki
)

echo.
echo [7/6] FFmpeg
echo   Downloading FFmpeg...
set FFMPEG_URL=https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip
curl -L -o ffmpeg.zip "%FFMPEG_URL%"
if exist ffmpeg.zip (
    echo   Extracting FFmpeg...
    powershell -Command "Expand-Archive -Path ffmpeg.zip -DestinationPath 'C:\Program Files\ffmpeg\' -Force"
    echo   Adding to PATH...
    setx /M PATH "%PATH%;C:\Program Files\ffmpeg\bin"
) else (
    echo   ⚠️ Download failed, please install manually from:
    echo   https://www.ffmpeg.org/download.html
)

echo.
echo.
echo ╔════════════════════════════════════════╗
echo ║  ✅ Installation Complete!            ║
echo ╚════════════════════════════════════════╝
echo.
echo 📊 Installed tools:
echo   ✓ Visual Studio Build Tools (manual)
echo   ✓ CMake
echo   ✓ Git
echo   ✓ Poppler (pdftoppm)
echo   ✓ LibreOffice
echo   ✓ Tesseract OCR
echo   ✓ FFmpeg
echo.
echo ⚠️  IMPORTANT: 
echo   1. RESTART your computer to apply PATH changes
echo   2. Complete Visual Studio Build Tools installation if prompted
echo.
echo 🚀 After restart, you can run:
echo    transmutation convert document.pdf --format png
echo    transmutation convert document.docx -o output.md
echo.
echo 🗑️ Cleaning up temporary files...
cd /d %TEMP%
rmdir /s /q "%TEMP_DIR%" 2>nul
echo.
pause

