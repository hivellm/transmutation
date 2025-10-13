# WiX Installer Configuration

This directory contains the WiX Toolset configuration for building Windows MSI installers.

## Files

- **`main.wxs`** - Main WiX source file (installer definition)
- **`License.rtf`** - License text displayed during installation
- **`banner.bmp`** *(optional)* - Top banner image (493×58 pixels)
- **`dialog.bmp`** *(optional)* - Welcome dialog background (493×312 pixels)

## Quick Build

```powershell
# From project root
.\build-msi.ps1
```

## Output

The MSI installer will be created at:
```
../target/wix/transmutation-0.1.0-x86_64.msi
```

## Documentation

See [`../docs/MSI_BUILD.md`](../docs/MSI_BUILD.md) for complete documentation.

## Customization

Edit `main.wxs` to:
- Change product name, version, manufacturer
- Add/remove files
- Modify installation directory
- Add registry keys
- Create custom shortcuts

## Testing

```powershell
# Install
msiexec /i ..\target\wix\transmutation-0.1.0-x86_64.msi

# Verify
transmutation --version

# Uninstall
msiexec /x ..\target\wix\transmutation-0.1.0-x86_64.msi
```

