#!/usr/bin/env bash
# Install this app on Linux for the person who runs it with sudo:
#
#   sudo ./install.sh              build, then install or update
#   sudo ./install.sh --no-deps    skip the system packages
#   sudo ./install.sh --uninstall  remove what an install put in place
#
# Root is used for one thing only: installing the build and runtime libraries
# (WebKitGTK and friends) with the system package manager. The app itself is
# built as you and installed into your home, owned by you:
#
#   ~/.local/bin/<binary>                     the app
#   ~/.local/share/icons/<identifier>.png     its icon
#   ~/.local/share/applications/<identifier>.desktop   the menu entry
#
# Names come from src-tauri/Cargo.toml (binary) and src-tauri/tauri.conf.json
# (productName, identifier), so renaming the app needs no change here.
# Set COMMENT to change the menu entry's description.
set -euo pipefail

die() { echo "install.sh: $*" >&2; exit 1; }
say() { echo "==> $*"; }

DEPS=1
UNINSTALL=0
for arg in "$@"; do
  case $arg in
    --no-deps) DEPS=0 ;;
    --uninstall) UNINSTALL=1 ;;
    -h|--help) sed -n '2,19p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    *) die "unknown option $arg (try --help)" ;;
  esac
done

[[ $EUID -eq 0 ]] || die "run it with sudo: sudo $0 $*"
USER_NAME=${SUDO_USER:-}
[[ -n $USER_NAME && $USER_NAME != root ]] ||
  die "run it with sudo from your own account, so it knows whose home to install into"
USER_HOME=$(getent passwd "$USER_NAME" | cut -d: -f6)
USER_GROUP=$(id -gn "$USER_NAME")
[[ -d $USER_HOME ]] || die "home directory of $USER_NAME not found"

# Run a command as the user, with their HOME (so rustup and cargo find their files).
as_user() { runuser -u "$USER_NAME" -- env HOME="$USER_HOME" PATH="$USER_PATH" "$@"; }
USER_PATH="$USER_HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin"

HERE=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
SRC="$HERE/src-tauri"
BIN=$(sed -n 's/^name *= *"\(.*\)"/\1/p' "$SRC/Cargo.toml" | head -n1)
NAME=$(sed -n 's/.*"productName": *"\([^"]*\)".*/\1/p' "$SRC/tauri.conf.json" | head -n1)
ID=$(sed -n 's/.*"identifier": *"\([^"]*\)".*/\1/p' "$SRC/tauri.conf.json" | head -n1)
COMMENT=${COMMENT:-"$NAME, a text-first native client"}
[[ -n $BIN && -n $NAME && -n $ID ]] || die "could not read the app's names from src-tauri/"

BIN_PATH="$USER_HOME/.local/bin/$BIN"
ICON_PATH="$USER_HOME/.local/share/icons/$ID.png"
DESKTOP_PATH="$USER_HOME/.local/share/applications/$ID.desktop"

refresh_menu() {
  if command -v update-desktop-database >/dev/null; then
    as_user update-desktop-database "$USER_HOME/.local/share/applications" || true
  fi
}

if [[ $UNINSTALL -eq 1 ]]; then
  rm -fv "$BIN_PATH" "$ICON_PATH" "$DESKTOP_PATH"
  refresh_menu
  say "Uninstalled $NAME. Its data (login, settings) stays in $USER_HOME/.local/share/$ID; delete that folder to remove it too."
  exit 0
fi

# 1. System packages: the Tauri prerequisites for building, which include the
#    runtime libraries. Lists from https://tauri.app/start/prerequisites/
if [[ $DEPS -eq 1 ]]; then
  say "Installing build and runtime libraries"
  if command -v pacman >/dev/null; then
    pacman -S --needed --noconfirm webkit2gtk-4.1 base-devel curl wget file openssl \
      appmenu-gtk-module libappindicator-gtk3 librsvg xdotool
  elif command -v apt-get >/dev/null; then
    apt-get update
    apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev \
      libssl-dev libayatana-appindicator3-dev librsvg2-dev
  elif command -v dnf >/dev/null; then
    dnf install -y webkit2gtk4.1-devel openssl-devel curl wget file libappindicator-gtk3-devel \
      librsvg2-devel libxdo-devel
    dnf group install -y c-development || dnf group install -y "C Development Tools and Libraries"
  elif command -v zypper >/dev/null; then
    zypper --non-interactive install webkit2gtk3-devel libopenssl-devel curl wget file \
      libappindicator3-1 librsvg-devel
    zypper --non-interactive install -t pattern devel_basis
  else
    echo "Unknown package manager: install the Tauri prerequisites yourself (https://tauri.app/start/prerequisites/)." >&2
  fi
fi

# 2. Build as the user, so target/ stays theirs. custom-protocol is the
#    production mode that `cargo tauri build` would turn on.
as_user sh -c 'command -v cargo' >/dev/null ||
  die "cargo not found for $USER_NAME. Install Rust as $USER_NAME first: https://rustup.rs"
say "Building $NAME (release; the first build takes a few minutes)"
as_user cargo build --release --manifest-path "$SRC/Cargo.toml" --features tauri/custom-protocol

# 3. Install into the user's home, owned by the user.
say "Installing into $USER_HOME/.local"
as_user mkdir -p "$(dirname "$BIN_PATH")" "$(dirname "$ICON_PATH")" "$(dirname "$DESKTOP_PATH")"
install -m 755 -o "$USER_NAME" -g "$USER_GROUP" "$SRC/target/release/$BIN" "$BIN_PATH"
install -m 644 -o "$USER_NAME" -g "$USER_GROUP" "$SRC/icons/icon.png" "$ICON_PATH"
cat > "$DESKTOP_PATH" <<EOF
[Desktop Entry]
Type=Application
Name=$NAME
Comment=$COMMENT
Exec=$BIN_PATH
Icon=$ICON_PATH
Terminal=false
Categories=Network;
EOF
chown "$USER_NAME:$USER_GROUP" "$DESKTOP_PATH"
chmod 644 "$DESKTOP_PATH"
refresh_menu

say "Installed. Find \"$NAME\" in your app menu, or run $BIN_PATH"
case ":$(runuser -l "$USER_NAME" -c 'echo $PATH' 2>/dev/null):" in
  *":$USER_HOME/.local/bin:"*) ;;
  *) echo "    (~/.local/bin isn't on your PATH, so type the full path to run it from a terminal)" ;;
esac
