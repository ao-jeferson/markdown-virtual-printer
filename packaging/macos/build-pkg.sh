#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "Usage: $0 <version> <release-root>" >&2
  exit 2
fi

version="$1"
release_root="$2"
pkg_root="${TMPDIR:-/tmp}/mdvp-pkg-root"
scripts="${TMPDIR:-/tmp}/mdvp-pkg-scripts"
rm -rf "$pkg_root" "$scripts"
mkdir -p "$pkg_root/usr/local/lib/mdvp" "$pkg_root/usr/local/share/ppd" "$scripts" "$release_root/out"
install -m 0755 "$release_root/mdvp-agent" "$pkg_root/usr/local/lib/mdvp/mdvp-agent"
install -m 0755 "$release_root/mdvp-cups-backend" "$pkg_root/usr/local/lib/mdvp/mdvp-cups-backend"
install -m 0644 "$release_root/libprinter_core.dylib" "$pkg_root/usr/local/lib/mdvp/libprinter_core.dylib"
install -m 0644 "$release_root/MarkdownVirtualPrinter.ppd" "$pkg_root/usr/local/share/ppd/MarkdownVirtualPrinter.ppd"

cat > "$scripts/postinstall" <<'EOF'
#!/bin/sh
set -u
agent=/usr/local/lib/mdvp/mdvp-agent
launch_agents=/Library/LaunchAgents
mkdir -p "$launch_agents"
cat > "$launch_agents/org.markdownvirtualprinter.agent.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>Label</key><string>org.markdownvirtualprinter.agent</string>
<key>ProgramArguments</key><array><string>/usr/local/lib/mdvp/mdvp-agent</string></array>
<key>RunAtLoad</key><true/>
<key>KeepAlive</key><true/>
</dict></plist>
PLIST
chmod 0644 "$launch_agents/org.markdownvirtualprinter.agent.plist"
if command -v cups-config >/dev/null 2>&1 && command -v lpadmin >/dev/null 2>&1; then
  backend_dir="$(cups-config --serverbin)/backend"
  if [ -d "$backend_dir" ] && install -m 0755 /usr/local/lib/mdvp/mdvp-cups-backend "$backend_dir/mdvp"; then
    lpadmin -p MarkdownVirtualPrinter -E -v mdvp:/ -P /usr/local/share/ppd/MarkdownVirtualPrinter.ppd || true
  else
    echo "CUPS backend directory is not writable by this installer; printer queue was not registered." >&2
  fi
fi
exit 0
EOF
chmod 0755 "$scripts/postinstall"

pkgbuild --root "$pkg_root" --scripts "$scripts" \
  --identifier org.markdownvirtualprinter.pkg --version "${version#v}" \
  --install-location / "${release_root}/out/markdown-virtual-printer-${version}-arm64.pkg"
