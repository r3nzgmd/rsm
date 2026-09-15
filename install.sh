set -e

URL="https://github.com/r3nzgmd/rsm/releases/download/v1.0.0/rsm-linux"
NAME="rsm"
INSTALL_DIR="/usr/local/bin"

if [ "$EUID" -ne 0 ]; then
	echo "Error: Run this script with sudo."
	exit 1
fi

if ! command -v curl &> /dev/null; then
	echo "Error: curl isn't installed."
	exit 1
fi

echo "Downloading binary from: $URL..."
curl -L "$URL" -o "$INSTALL_DIR/rsm"
chmod +x "$INSTALL_DIR/$NAME"

echo "Done. You can run the program by typing: $NAME."
