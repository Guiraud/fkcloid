# FkCloud Share (Android)

Android share-sheet client for [rmfakecloud](https://github.com/ddvk/rmfakecloud),
the self-hosted reMarkable® cloud replacement. Share a PDF or EPUB from any
app, pick a destination folder, and it lands in your reMarkable library.

License: [GPL-3.0-or-later](LICENSE). Not affiliated with reMarkable AS.

## Features

- Android share target (`ACTION_SEND` / `ACTION_SEND_MULTIPLE`) for PDF and EPUB
- Destination folder picker: navigable tree (breadcrumb, tap into
  subfolders) with documents shown inline for context — not a flat list
- Upload confirmation auto-dismisses after a short delay; no extra tap needed
- Web file manager: opens the server's own rmfakecloud web UI in an
  in-app WebView (browse, rename, move, delete, download) instead of
  reimplementing a file manager natively
- HTTPS enforced by default; plain HTTP requires an explicit opt-in
- Password and session token encrypted with an AES-256-GCM key in the
  Android Keystore; `allowBackup` disabled so secrets never leave the device
- Visual identity: palette « Registre libre » (warm cream/dark surfaces, blue primary, thin-outline borders) — see app themes under `app/src/main/res/`
- Zero trackers, zero proprietary dependencies — F-Droid friendly
- Dependencies: AndroidX AppCompat, Material Components, Kotlin coroutines,
  OkHttp (all FOSS); WebView is part of the Android platform, no extra
  dependency

## Server API used

The app talks to the rmfakecloud **web UI API**, not the reMarkable sync
protocol, so no device pairing code is needed — just a web UI account:

| Call | Purpose |
|------|---------|
| `POST /ui/api/login` | JSON `{email, password}` → JWT (plain-text body, 24 h) |
| `GET /ui/api/documents` | Full document tree (folders + files), used by the folder picker |
| `POST /ui/api/documents/upload` | Multipart fields `parent` + `file` |

Authenticated calls send `Authorization: Bearer <token>`; the app caches the
token for 23 h and re-logs in automatically on expiry or `401`.

The web file manager instead loads the server's own web root directly in a
WebView and lets its own login form/session handle everything — it doesn't
go through the app's stored JWT.

## Build

Requirements: JDK 17, Android SDK (platform 35).

```bash
cd android
./gradlew assembleRelease   # unsigned APK in app/build/outputs/apk/release/
./gradlew assembleDebug     # debug-signed APK for local testing
```

## Usage

1. Open **FkCloud Share** once: enter the server URL (e.g.
   `https://rm-cloud.example.invalid`), your web UI
   username and password, then *Save & test connection*.
2. In any app, share a PDF/EPUB → choose **FkCloud Share** → pick a folder
   → *Upload*.
3. To browse/manage what's already on the server, tap **Open web file
   manager** on the main screen.

## F-Droid notes

- `versionCode`/`versionName` live in `app/build.gradle.kts`
- Fastlane metadata (en-US, fr-FR) is in `../../fastlane/metadata/android/`
- Release builds are unsigned (F-Droid signs its own builds);
  `dependenciesInfo` is stripped from the APK for reproducibility
- No `google-services`, no proprietary Maven artifacts
