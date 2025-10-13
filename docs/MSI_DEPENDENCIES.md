# MSI Installer with Dependency Management

## Overview

Transmutation MSI installer can be configured to help users install external dependencies (Poppler, LibreOffice, Tesseract, FFmpeg).

---

## Three Approaches

### 1. **Basic MSI** (Default)
```powershell
.\build-msi.ps1
```

**Pros:**
- ✅ Small size (~5 MB)
- ✅ Fast installation
- ✅ No elevated permissions needed

**Cons:**
- ❌ Users must install dependencies manually
- ❌ No guidance provided

**Best for:** Advanced users who know how to install dependencies

---

### 2. **MSI with Dependency Installer** (Recommended)
```powershell
.\build-msi.ps1 -IncludeDepsInstaller
```

**What it does:**
- Installs Transmutation
- Includes a PowerShell script (`install-deps.ps1`)
- Creates Start Menu shortcut: "Install Dependencies"

**After installation, users can:**
```powershell
# Option A: Use Start Menu shortcut
Start Menu → Transmutation → Install Dependencies

# Option B: Run PowerShell command
cd "C:\Program Files\Transmutation"
.\install-deps.ps1
```

**The script automatically:**
- ✅ Detects Chocolatey or winget
- ✅ Installs all dependencies
- ✅ Shows progress and errors
- ✅ Provides manual installation instructions if needed

**Pros:**
- ✅ User-friendly
- ✅ One-click dependency installation
- ✅ Automatic package manager detection
- ✅ Fallback to manual instructions

**Cons:**
- ⚠️ Requires Chocolatey or winget
- ⚠️ Requires Administrator privileges (for dependencies, not MSI)

**Best for:** Distribution to end-users

---

### 3. **Custom MSI with Auto-Install** (Advanced)
Use `wix/main-with-deps.wxs` for fully automated installation.

**What it does:**
- Installs Transmutation
- **Automatically** installs dependencies during MSI installation
- Uses Chocolatey if available

**Build:**
```powershell
.\build-msi.ps1 -Method wix
# Then manually configure to use main-with-deps.wxs
```

**Pros:**
- ✅ Fully automated
- ✅ One-step installation
- ✅ No user interaction needed

**Cons:**
- ❌ Requires Chocolatey pre-installed
- ❌ Much larger MSI if embedding installers
- ❌ Longer installation time
- ❌ More complex build process
- ⚠️ May fail if package manager unavailable

**Best for:** Enterprise deployments with controlled environments

---

## Comparison Table

| Feature | Basic | With Installer Script | Auto-Install |
|---------|-------|----------------------|--------------|
| **MSI Size** | ~5 MB | ~5 MB | ~5 MB |
| **User Interaction** | Manual | One-click after install | None |
| **Requires Admin** | No | Yes (for deps) | Yes |
| **Requires Pkg Manager** | No | Yes | Yes (pre-installed) |
| **Installation Time** | <1 min | <1 min + deps (~5 min) | ~5-10 min |
| **Flexibility** | Low | High | Low |
| **Failure Handling** | N/A | Fallback instructions | May fail silently |
| **Best For** | Advanced users | End-users | Enterprise |

---

## Recommendation

For **public distribution**, use:
```powershell
.\build-msi.ps1 -IncludeDepsInstaller
```

This provides the best balance of:
- ✅ User experience (easy to install dependencies)
- ✅ Reliability (fallback to manual instructions)
- ✅ Flexibility (users can skip dependencies)
- ✅ Size (no embedded installers)

---

## How It Works: Dependency Installer Script

The `install-deps-embedded.ps1` script:

1. **Checks for Administrator privileges**
   - Warns if not running as Admin
   - Allows to continue (for portable installation)

2. **Detects package managers**
   ```
   Chocolatey → Use choco install
   winget → Use winget install
   None → Show manual instructions
   ```

3. **Installs dependencies**
   ```
   Poppler → PDF to Image
   LibreOffice → DOCX to PDF/Image
   Tesseract → OCR
   FFmpeg → Audio/Video
   ```

4. **Provides feedback**
   - Progress messages
   - Success/failure notifications
   - Manual installation links if needed

5. **Reminds to restart terminal**
   - PATH changes require restart
   - Tests to verify installation

---

## Security Considerations

### Why Not Auto-Install During MSI?

1. **Security:** MSI Custom Actions run with SYSTEM privileges
   - Running package managers as SYSTEM is risky
   - May expose system to vulnerabilities

2. **Reliability:** Package managers may not be installed
   - Chocolatey is not pre-installed on Windows
   - winget is only on Windows 10 1809+ and Windows 11

3. **User Control:** Users should choose what to install
   - Some users may not want all dependencies
   - Corporate environments may have restrictions

4. **Installation Time:** Dependencies are large
   - LibreOffice alone is ~300 MB
   - Total download: ~500 MB
   - MSI would take 10+ minutes

### Why Post-Install Script is Better

1. **User Control:** Runs with user privileges
2. **Transparency:** User sees what's being installed
3. **Flexibility:** Can skip dependencies
4. **Reliability:** Fallback to manual instructions
5. **Fast MSI:** Installation remains fast

---

## Alternative: Bundle Installer

If you need a **true "all-in-one" installer**, consider:

### Option A: NSIS/Inno Setup Wrapper
Create a wrapper installer that:
1. Installs Transmutation MSI
2. Installs Chocolatey (if missing)
3. Runs dependency installation

### Option B: Docker Container
Distribute as Docker image with all dependencies:
```bash
docker pull hivellm/transmutation:latest
docker run -v $(pwd):/data transmutation convert /data/document.pdf
```

### Option C: Portable ZIP
Create a portable package:
```
transmutation-portable.zip
├── transmutation.exe
├── poppler/
├── libreoffice-portable/
├── tesseract/
└── ffmpeg/
```

All dependencies as portable versions (~1 GB).

---

## Testing

### Test MSI with Dependency Installer

```powershell
# Build
.\build-msi.ps1 -IncludeDepsInstaller

# Install
msiexec /i target\wix\transmutation-*.msi

# Test dependency installer
cd "C:\Program Files\Transmutation"
.\install-deps.ps1

# Verify
transmutation --version
pdftoppm -v
tesseract --version
ffmpeg -version
```

### Test in Clean Environment

```powershell
# Use Windows Sandbox or VM
# 1. Install MSI
# 2. Run dependency installer
# 3. Test conversions
transmutation convert test.pdf --format png
transmutation convert test.docx -o output.md
```

---

## FAQ

### Q: Can dependencies be installed silently?

**A:** Yes, modify the script:
```powershell
.\install-deps.ps1 -Silent
```

### Q: What if Chocolatey is not installed?

**A:** The script detects this and:
1. Shows installation instructions
2. Offers winget as alternative
3. Provides manual download links

### Q: Can users opt-out of dependencies?

**A:** Yes, dependencies are completely optional. Transmutation works without them for basic features (PDF/DOCX → Markdown).

### Q: How much disk space is needed?

**A:**
- Transmutation: ~5 MB
- Dependencies: ~800 MB total
  - Poppler: ~50 MB
  - LibreOffice: ~300 MB
  - Tesseract: ~50 MB
  - FFmpeg: ~100 MB

### Q: Can I customize which dependencies are installed?

**A:** Yes, edit `install-deps-embedded.ps1` to remove unwanted dependencies.

---

**Last Updated:** October 13, 2025  
**See also:** [`MSI_BUILD.md`](MSI_BUILD.md), [`DEPENDENCIES.md`](DEPENDENCIES.md)

