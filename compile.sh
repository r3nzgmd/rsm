# script used to clone git repository into home directory and build the app using Cargo
set -e

if ! command -v git &> /dev/null; then
	echo "Error: Git isn't installed." >&2
	exit 1
fi

if ! command -v cargo &> /dev/null; then
	echo "Error: Cargo isn't installed." >&2
	exit 1
fi

echo "Cloning repository..."
if [ -d "$HOME/rsm" ]; then
	echo "Directory $HOME/rsm already exists." >&2
	exit 1
else
	git clone "https://github.com/r3nzgmd/rsm" "$HOME/rsm"
fi

echo "Done."

cd "$HOME/rsm"

echo "Compiling..."
cargo build --release

echo "Done. Compilation finished without errors."
