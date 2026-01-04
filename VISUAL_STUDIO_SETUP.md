# Windows Build Setup (Visual Studio)

This guide helps you build this Rust project on Windows.

## Quick Start

### Option 1: Use Developer Command Prompt (Recommended)

1. Press **Windows Key**
2. Type: **"Developer Command Prompt for VS 2022"**
3. Open it and run:

```powershell
cd "c:\Users\wind xebec\OneDrive\Desktop\abc"
cargo build --release
```

Done! ✅

---

### Option 2: Fix Regular PowerShell (One-Time Setup)

If you want `cargo build` to work from ANY terminal:

#### Step 1: Install Visual Studio Build Tools

1. Open **Visual Studio Installer** (search in Start Menu)
2. Click **Modify** on your Visual Studio installation
3. Check ✅ **"Desktop development with C++"**
4. In **Individual components**, ensure these are checked:
   - ✅ MSVC v143 - VS 2022 C++ x64/x86 build tools
   - ✅ Windows 11 SDK (latest version)
5. Click **Modify** and wait for installation
6. **Restart your computer**

#### Step 2: Test

```powershell
cd "c:\Users\wind xebec\OneDrive\Desktop\abc"
cargo build --release
```

---

## Troubleshooting

| Error | Solution |
|-------|----------|
| `cannot open input file 'kernel32.lib'` | Install Windows SDK via Visual Studio Installer |
| `link.exe not found` | Install "MSVC build tools" in Visual Studio Installer |
| Works in Developer PowerShell but not regular | Use Developer PowerShell, or add SDK paths to system PATH |

---

## Verification

After setup, this should work in regular PowerShell:

```powershell
rustc --version
cargo --version
cargo build --release
```

If you still have issues, always use **"Developer Command Prompt for VS 2022"** - it works 100% of the time.
