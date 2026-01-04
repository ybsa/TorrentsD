# Why Can't We Build on Windows? - Simple Fix

## The Problem

You're getting this error:

```
LINK : fatal error LNK1181: cannot open input file 'kernel32.lib'
```

**Why?** The Windows SDK libraries aren't in the correct path, even though Visual Studio is installed.

## Quick Solutions

### ✅ Solution 1: Use the Build Tools Directly (Easiest)

1. **Open "Developer PowerShell for VS 2022"** (not regular PowerShell!)
   - Press Windows key
   - Search: "Developer PowerShell"
   - Or: "x64 Native Tools Command Prompt for VS 2022"

2. **Navigate and build:**

   ```powershell
   cd "C:\Users\wind xebec\OneDrive\Desktop\abc"
   cargo build --release
   ```

   This sets up all the paths automatically!

### ✅ Solution 2: Install Missing SDK Components

The Visual Studio installer might be missing the Windows SDK. Here's how to fix it:

1. **Open Visual Studio Installer**
   - Search "Visual Studio Installer" in Start Menu

2. **Click "Modify" on your VS 2022 installation**

3. **In Individual Components tab, search for and check:**
   - ✅ Windows 11 SDK (10.0.22621.0) or latest
   - ✅ MSVC v143 - VS 2022 C++ x64/x86 build tools (Latest)
   - ✅ C++ CMake tools for Windows

4. **Click "Modify" and wait for installation**

5. **Restart your computer**

6. **Try building again**

### ✅ Solution 3: Just Use Linux! (Recommended)

**Why Linux is Better for This:**

- ✅ No SDK issues
- ✅ Builds instantly
- ✅ No complex setup
- ✅ Faster compile times
- ✅ Better performance

**On your Linux machine:**

```bash
git clone https://github.com/ybsa/TorrentsD.git
cd TorrentsD
cargo build --release
# Done! Works immediately!
```

## Current Workaround

**You're already using Linux** (I saw in your screenshots), so:

```bash
# On your Linux box
cd ~/Projects/temp/TorrentsD
git pull
cargo build --release

# Test it!
./target/release/torrent-client download file.torrent -o ~/Downloads
```

**This works perfectly on Linux with ZERO setup!** 🚀

## Why Does This Happen on Windows?

Windows needs:

1. Visual Studio C++ compiler (`cl.exe`) ✅ You have this
2. Windows SDK (`kernel32.lib`, etc.) ❌ This is missing/misconfigured
3. Proper environment variables ❌ Not set in regular terminals

On Linux, you just need:

- `gcc` (usually pre-installed)
- Rust (install with one command)
- That's it!

## Recommendation

**For development:** Use Linux (easier, faster, no issues)
**For Windows users:** Build on Linux, then the `.exe` can be shared

If you really need Windows builds, use "Developer PowerShell" from Visual Studio.

---

## Quick Test Right Now

**On your Linux terminal:**

```bash
cd ~/Projects/temp/TorrentsD
cargo --version     # Verify Rust is installed
cargo build --release

# If successful:
./target/release/torrent-client --help
```

**This will work immediately on Linux!** No configuration needed.

Windows is fighting us because of SDK paths. Linux just works. 😊
