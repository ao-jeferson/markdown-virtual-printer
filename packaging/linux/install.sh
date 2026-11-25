#!/usr/bin/env bash
set -euo pipefail

if [[ $EUID -eq 0 ]]; then
  echo "Run this installer as your desktop user; it uses sudo only for the CUPS queue." >&2
  exit 2
fi
command -v lpadmin >/dev/null || { echo "CUPS is required (install cups first)." >&2; exit 1; }
root_dir="$(cd "$(dirname "$0")/../.." && pwd)"
bin_dir="${HOME}/.local/bin"
mkdir -p "$bin_dir"
install -m 0755 "$root_dir/target/release/mdvp-agent" "$bin_dir/mdvp-agent"
install -m 0755 "$root_dir/target/release/mdvp-cups-backend" /tmp/mdvp-cups-backend
server_bin="$(cups-config --serverbin)/backend"
sudo install -m 0755 /tmp/mdvp-cups-backend "${server_bin}/mdvp"
rm -f /tmp/mdvp-cups-backend
sudo install -m 0644 "$root_dir/packaging/cups/MarkdownVirtualPrinter.ppd" /usr/share/ppd/MarkdownVirtualPrinter.ppd
sudo lpadmin -p MarkdownVirtualPrinter -E -v mdvp:/ -P /usr/share/ppd/MarkdownVirtualPrinter.ppd
sudo cupsenable MarkdownVirtualPrinter

service_dir="${HOME}/.config/systemd/user"
mkdir -p "$service_dir"
cat >"${service_dir}/mdvp-agent.service" <<EOF
[Unit]
Description=Markdown Virtual Printer save dialog
After=graphical-session.target

[Service]
ExecStart=${bin_dir}/mdvp-agent
Restart=on-failure

[Install]
WantedBy=default.target
EOF
systemctl --user daemon-reload
systemctl --user enable --now mdvp-agent.service
echo "Installed the Markdown Virtual Printer CUPS queue. Select it from the print dialog."
