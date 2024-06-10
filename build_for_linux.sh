echo "Building"
cargo build --release

echo "Copying files"
mkdir spaceshipment_editor_linux

cp levels spaceshipment_editor_linux/ -r
cp target/release/spaceshipment_editor spaceshipment_editor_linux/spaceshipment_editor.x86_64
cp README.md spaceshipment_editor_linux/

echo "Zipping up"
rm spaceshipment_editor_linux.zip
zip spaceshipment_editor_linux.zip spaceshipment_editor_linux -r

echo "Cleaning up"
rm spaceshipment_editor_linux/ -r
