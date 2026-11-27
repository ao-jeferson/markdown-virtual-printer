# Android print service

The Android app registers **Markdown Virtual Printer** as a native Android `PrintService`. It accepts the PDF created by Android's print spooler, converts it with the shared Rust core through a small JNI bridge, and uses a notification to let the user open Android's system save picker for the `.md` output.

## User flow

1. Install the APK and open the app once.
2. Allow notifications so a completed print job can open the save picker.
3. Tap **Open print settings** and enable Markdown Virtual Printer.
4. In another app, choose **Print** and select Markdown Virtual Printer.
5. Tap the notification and choose the destination and filename in the system picker.

The **Keep formatting** setting is on by default. Turn it off to keep extracted Markdown source without report formatting. Documents are processed locally. Android grants access only to the destination selected in the system picker; the app does not request broad storage access. Print jobs over 64 MiB are rejected to bound memory use on phones.

## Build

The GitHub Actions workflow builds Rust shared libraries for `arm64-v8a` and `x86_64`, packages the installable debug APK, and uploads it as the `markdown-virtual-printer-android` workflow artifact. The APK is experimental and is not signed as a Play Store release.

To build locally, install JDK 17, Android SDK platform 36, Android NDK 27.0.12077973, CMake 3.22.1, Rust targets `aarch64-linux-android` and `x86_64-linux-android`, `cargo-ndk`, and Gradle 8.13. Then run:

```sh
cd platforms/android
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/27.0.12077973"
cargo ndk -t arm64-v8a -t x86_64 -o app/src/main/jniLibs build \
  --manifest-path ../../Cargo.toml --release -p printer-core --lib
gradle --no-daemon assembleDebug
```

Android's system print service API is available from API 19; this prototype sets Android 8.0 (API 26) as its minimum because it uses notification channels. A device-level print and save flow has not yet been validated. The APK is debug-signed for development and is not a store release.
