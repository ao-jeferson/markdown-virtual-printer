#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "Usage: $0 <version> <release-root>" >&2
  exit 2
fi

version="$1"
release_root="$2"
package_root="${TMPDIR:-/tmp}/mdvp-deb-root"
rm -rf "$package_root"
mkdir -p "$package_root/DEBIAN" \
  "$package_root/usr/lib/cups/backend" \
  "$package_root/usr/lib/mdvp" \
  "$package_root/usr/share/ppd" \
  "$package_root/usr/lib/systemd/user"

install -m 0755 "$release_root/mdvp-agent" "$package_root/usr/lib/mdvp/mdvp-agent"
install -m 0755 "$release_root/mdvp-cups-backend" "$package_root/usr/lib/cups/backend/mdvp"
install -m 0644 "$release_root/MarkdownVirtualPrinter.ppd" "$package_root/usr/share/ppd/MarkdownVirtualPrinter.ppd"
cat > "$package_root/usr/lib/systemd/user/mdvp-agent.service" <<'EOF'
[Unit]
Description=Markdown Virtual Printer save dialog
After=graphical-session.target

[Service]
ExecStart=/usr/lib/mdvp/mdvp-agent
Restart=on-failure

[Install]
WantedBy=default.target
EOF

cat > "$package_root/DEBIAN/control" <<EOF
Package: markdown-virtual-printer
Version: ${version#v}
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Markdown Virtual Printer contributors
Description: Native CUPS virtual printer that saves print jobs as Markdown
 Experimental x86_64 package. Desktop session integration is still being validated.
Depends: cups, systemd
EOF

cat > "$package_root/DEBIAN/postinst" <<'EOF'
#!/bin/sh
set -e
if command -v lpadmin >/dev/null 2>&1; then
  lpadmin -p MarkdownVirtualPrinter -E -v mdvp:/ -P /usr/share/ppd/MarkdownVirtualPrinter.ppd || true
  cupsenable MarkdownVirtualPrinter || true
fi
systemctl --global enable mdvp-agent.service || true
exit 0
EOF
chmod 0755 "$package_root/DEBIAN/postinst"

mkdir -p "$release_root/out"
dpkg-deb --root-owner-group --build "$package_root" "$release_root/out/markdown-virtual-printer_${version#v}_amd64.deb"
