# Building MSI Installer for Windows

This guide explains how to create a professional Windows Installer (`.msi`) for Transmutation.

---

## Methods

There are **two methods** to build the MSI installer:

| Method | Tool | Complexity | Recommended |
|--------|------|------------|-------------|
| **1. cargo-wix** | Rust plugin | ⭐ Easy | ✅ Yes |
| **2. WiX Toolset** | Manual build | Medium | For customization |

---

## Method 1: cargo-wix (Recommended)

**cargo-wix** is a Cargo plugin that simplifies MSI creation.

### Prerequisites

1. **Rust**: 1.85+ installed
2. **Visual Studio Build Tools**: For linking
3. **WiX Toolset 3.11+**: Required by cargo-wix

### Installation

```powershell
# Quick install (automatic)
.\install-wix.ps1

# Or manual install
# Step 1: Install WiX Toolset
choco install wixtoolset

# Step 2: Install cargo-wix
cargo install cargo-wix

# Step 3: Restart PowerShell (important!)
```

**Important:** You must **restart PowerShell** after installing WiX Toolset for PATH changes to take effect.

#### Troubleshooting: "candle.exe not found"

If you see:
```
Error[2]: The compiler application (candle) could not be found in the PATH
```

**Solution:**
```powershell
# Option 1: Quick fix (automatic)
.\install-wix.ps1

# Option 2: Manual install via Chocolatey
choco install wixtoolset -y

# Option 3: Download installer
# https://wixtoolset.org/releases/
# Download and run: wix311.exe

# After installation: RESTART POWERSHELL
```

### Build MSI

```powershell
# Option 1: Basic installer (Transmutation only)
.\build-msi.ps1

# Option 2: With dependency installer (recommended for distribution)
.\build-msi.ps1 -IncludeDepsInstaller

# Option 3: Manual steps
# Step 1: Build release binary
cargo build --release --bin transmutation --features "cli,pdf,office,web"

# Step 2: Generate MSI
cargo wix --nocapture
```

**What's the difference?**

| Option | Size | Includes | Use Case |
|--------|------|----------|----------|
| **Basic** | ~5 MB | Transmutation CLI only | Users will install dependencies manually |
| **With Deps Installer** | ~5 MB + script | Transmutation + PowerShell script to install dependencies | Easy distribution - users can install dependencies after |

The **dependency installer** (`-IncludeDepsInstaller`) adds a PowerShell script that:
- ✅ Detects Chocolatey or winget
- ✅ Installs Poppler, LibreOffice, Tesseract, FFmpeg
- ✅ Creates Start Menu shortcut for easy access

### Output

The MSI will be created at:
```
target/wix/transmutation-0.1.1-x86_64.msi
```

---

## Method 2: WiX Toolset (Advanced)

For full control over the installer, use WiX Toolset directly.

### Prerequisites

1. **WiX Toolset 3.11+**: Download from https://wixtoolset.org/releases/
2. **Visual Studio Build Tools**
3. **Rust 1.85+**

### Installation

```powershell
# Install WiX Toolset
choco install wixtoolset

# Or download installer from:
# https://github.com/wixtoolset/wix3/releases
```

### Build Process

```powershell
# Use the provided script with WiX method
.\build-msi.ps1 -Method wix

# Or manual steps:
# 1. Build release binary
cargo build --release --features "cli,pdf,office,web"

# 2. Compile WiX source
"C:\Program Files (x86)\WiX Toolset v3.11\bin\candle.exe" `
    -dVersion="0.1.1" `
    -dCargoTargetBinDir="target\release" `
    -dSourceDir="." `
    -out "target\wix\" `
    "wix\main.wxs"

# 3. Link MSI
"C:\Program Files (x86)\WiX Toolset v3.11\bin\light.exe" `
    -ext WixUIExtension `
    -out "target\wix\transmutation-0.1.1-x86_64.msi" `
    "target\wix\main.wixobj"
```

---

## Installing Dependencies After MSI Installation

If you built the MSI with `-IncludeDepsInstaller`, users can install dependencies in 3 ways:

### Method 1: Start Menu Shortcut

1. Open Start Menu
2. Find "Transmutation" folder
3. Click "Install Dependencies"
4. Follow the prompts

### Method 2: PowerShell Command

```powershell
# Navigate to installation directory
cd "C:\Program Files\Transmutation"

# Run installer
.\install-deps.ps1
```

### Method 3: Manual Installation

Users can still use the standalone scripts:
```powershell
# Download and run
.\install\install-deps-windows.ps1

# Or with Chocolatey
choco install poppler libreoffice tesseract ffmpeg -y

# Or with winget
winget install TheDocumentFoundation.LibreOffice
winget install UB-Mannheim.TesseractOCR
winget install Gyan.FFmpeg
```

---

## MSI Features

The generated installer includes:

### ✅ Installation Features

- **Binary Installation**: Installs `transmutation.exe` to `C:\Program Files\Transmutation\`
- **PATH Addition**: Automatically adds to system PATH
- **Start Menu Shortcut**: Creates shortcut in Start Menu
- **License Agreement**: Shows MIT license during installation
- **Uninstaller**: Complete uninstall support
- **Upgrade Support**: Can upgrade over previous versions

### 📦 Included Files

- `transmutation.exe` - Main CLI binary
- `LICENSE.txt` - MIT license
- `README.md` - Project documentation

---

## Customization

### Modify Installer

Edit `wix/main.wxs` to customize:

1. **Product Information**:
   ```xml
   <Product
     Name='Transmutation'
     Manufacturer='HiveLLM Team'
     Version='0.1.1'>
   ```

2. **Installation Directory**:
   ```xml
   <Directory Id='APPLICATIONFOLDER' Name='Transmutation'>
   ```

3. **Additional Files**:
   ```xml
   <Component Id='MyComponent' Guid='*' Win64='yes'>
     <File Id='MyFile' Source='path\to\file' />
   </Component>
   ```

4. **Registry Keys**:
   ```xml
   <RegistryValue
     Root='HKLM'
     Key='Software\Transmutation'
     Name='InstallPath'
     Type='string'
     Value='[APPLICATIONFOLDER]' />
   ```

### Branding

Replace default images in `wix/`:
- `banner.bmp` - Top banner (493×58 pixels)
- `dialog.bmp` - Welcome dialog (493×312 pixels)

---

## Testing

### Silent Install (for CI/CD)

```powershell
# Install silently
msiexec /i target\wix\transmutation-0.1.1-x86_64.msi /qn /l*v install.log

# Verify installation
transmutation --version

# Uninstall silently
msiexec /x target\wix\transmutation-0.1.1-x86_64.msi /qn
```

### Interactive Install

```powershell
# Double-click the MSI file or:
msiexec /i target\wix\transmutation-0.1.1-x86_64.msi
```

### Installation Verification

```powershell
# Check if installed
Get-Command transmutation

# Check version
transmutation --version

# Check installation path
(Get-Command transmutation).Source
```

---

## Distribution

### Signing the MSI

For production releases, sign the MSI with a code signing certificate:

```powershell
# Sign MSI (requires certificate)
signtool sign /f "certificate.pfx" /p "password" /t "http://timestamp.digicert.com" `
    target\wix\transmutation-0.1.1-x86_64.msi

# Verify signature
signtool verify /pa target\wix\transmutation-0.1.1-x86_64.msi
```

### GitHub Releases

```yaml
# .github/workflows/release.yml
- name: Build MSI
  run: |
    cargo install cargo-wix
    cargo wix --nocapture
    
- name: Upload MSI
  uses: actions/upload-artifact@v3
  with:
    name: transmutation-windows-installer
    path: target/wix/*.msi
```

---

## Troubleshooting

### Error: "WiX Toolset not found"

**Solution**: Install WiX Toolset or use cargo-wix:
```powershell
cargo install cargo-wix
.\build-msi.ps1  # Uses cargo-wix by default
```

### Error: "candle.exe failed with error code 1"

**Solution**: Check `wix/main.wxs` syntax:
```powershell
# Validate XML
[xml]$xml = Get-Content wix\main.wxs
$xml.OuterXml  # Should not throw errors
```

### Error: "light.exe: Unresolved reference to symbol 'WixUI:WixUI_InstallDir'"

**Solution**: Add `-ext WixUIExtension` to light.exe:
```powershell
light.exe -ext WixUIExtension -out output.msi input.wixobj
```

### MSI doesn't add to PATH

**Solution**: Run installer as Administrator:
```powershell
# Right-click MSI → "Run as Administrator"
# Or:
Start-Process msiexec -ArgumentList "/i transmutation.msi /qn" -Verb RunAs
```

---

## File Structure

```
transmutation/
├── wix/
│   ├── main.wxs          # WiX source file (installer definition)
│   ├── License.rtf       # License text for installer UI
│   ├── banner.bmp        # Top banner (optional)
│   └── dialog.bmp        # Welcome dialog image (optional)
├── build-msi.ps1         # Automated build script
├── Cargo.toml            # Includes [package.metadata.wix]
└── target/
    └── wix/
        ├── *.wixobj      # Compiled WiX object
        └── *.msi         # Final installer ✅
```

---

## Advanced Configuration

### Include Dependencies

If your build requires external DLLs:

```xml
<Component Id='Dependencies' Guid='*' Win64='yes'>
  <File Id='MyDLL' Source='path\to\library.dll' />
</Component>
```

### Custom Actions

Run scripts during installation:

```xml
<CustomAction Id='MyAction'
  BinaryKey='WixCA'
  DllEntry='CAQuietExec'
  Execute='deferred'
  Return='check'
  Impersonate='no' />
```

### Multiple Features

Offer optional components:

```xml
<Feature Id='CoreFeature' Title='Core' Level='1'>
  <ComponentRef Id='MainExecutable' />
</Feature>

<Feature Id='DocsFeature' Title='Documentation' Level='2'>
  <ComponentRef Id='Documentation' />
</Feature>
```

---

## CI/CD Integration

### GitHub Actions

```yaml
name: Build MSI

on:
  push:
    tags:
      - 'v*'

jobs:
  build-msi:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Install cargo-wix
        run: cargo install cargo-wix
        
      - name: Build MSI
        run: cargo wix --nocapture
        
      - name: Upload Release Asset
        uses: actions/upload-release-asset@v1
        with:
          upload_url: ${{ github.event.release.upload_url }}
          asset_path: target/wix/transmutation-*.msi
          asset_name: transmutation-${{ github.ref_name }}-x86_64.msi
          asset_content_type: application/octet-stream
```

---

## References

- **cargo-wix**: https://github.com/volks73/cargo-wix
- **WiX Toolset**: https://wixtoolset.org/documentation/
- **WiX Tutorial**: https://www.firegiant.com/wix/tutorial/

---

**Last Updated**: October 13, 2025  
**Tested on**: Windows 11 with WiX Toolset 3.11

