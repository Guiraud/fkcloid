# FkCloud Share Code Signing and Notarization (Tauri 2)

French: [SIGNING.fr.md](SIGNING.fr.md).

This document describes the code signing and notarization procedure for distributing FkCloud Share in production for macOS and Windows without security warnings (SmartScreen, Gatekeeper).

---

## 1. macOS Distribution (Gatekeeper & Notarization)

For macOS, the application must be signed with an Apple Developer certificate and notarized (submitted to Apple's servers for security analysis) in order to run without a blocking warning message.

### Prerequisites
- A paid **Apple Developer** account ($99/year).
- A macOS machine or macOS runner with Xcode installed.
- A **Developer ID Application** certificate installed in the runner's keychain.
- An Apple ID with a dedicated app-specific password for notarization.

### Build environment variables
When launching the Tauri build, set the following variables in your CI environment:

```bash
# Exact certificate name in your Keychain
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name/Company (TEAMID)"

# Your Apple Developer account identifier
export APPLE_ID="your.email@dev-account.apple.com"

# App-specific password generated at appleid.apple.com
export APPLE_PASSWORD="abcd-efgh-ijkl-mnop"

# Your Apple Developer Team ID
export APPLE_TEAM_ID="TEAMID1234"
```

### Entitlements
Tauri automatically handles notarization via `notarytool` when calling `tauri build` if the variables above are present. If you use notarization, configure network entitlements if App Sandbox is enabled.

---

## 2. Windows Distribution (SmartScreen)

For Windows, the MSI installer must be signed with a code signing certificate (ideally EV — Extended Validation — to instantly remove the Microsoft SmartScreen filter).

### Prerequisites
- A Windows code signing certificate (`.pfx` file or via HSM/Cloud module).
- The `signtool.exe` tool (included in the Windows SDK).

### Tauri configuration for Windows
In `tauri.conf.json`, you can configure the timestamp server URL:

```json
{
  "bundle": {
    "windows": {
      "digestAlgorithm": "sha256",
      "timestampUrl": "http://timestamp.digicert.com"
    }
  }
}
```

### Build variables (SignTool)
Set these variables on your Windows runner before the build:

```powershell
# Local path to the PFX certificate file
$env:TAURI_SIGNING_IDENTITY = "C:\Certificates\my-code-signing.pfx"

# Password protecting the PFX certificate
$env:TAURI_SIGNING_IDENTITY_PASSWORD = "MySecretPassword"
```

---

## 3. Publishing and CI Integration (GitLab CI/CD)

The production build script will automatically generate the final binaries and compute SHA-256 hashes for traceability and security.

### Build command
```bash
npm run tauri build
```

### Automatic checksum computation (SHA-256)
After the build, generate hashes in the artifacts directory:

```bash
# macOS
shasum -a 256 src-tauri/target/release/bundle/dmg/*.dmg > SHA256SUMS.txt

# Windows
certutil -hashfile src-tauri/target/release/bundle/msi/*.msi SHA256 > SHA256SUMS.txt
```
These checksums must be published on the Release page of your GitLab repository or website.
