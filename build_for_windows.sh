echo "Building"
cargo build --release --target x86_64-pc-windows-gnu

echo "Copying files"
rm spaceshipment_editor_win/levels/ -r
cp levels spaceshipment_editor_win/ -r
cp target/x86_64-pc-windows-gnu/release/spaceshipment_editor.exe spaceshipment_editor_win/
cp README.md spaceshipment_editor_win/

echo "Zipping up"
rm spaceshipment_editor_win.zip
zip spaceshipment_editor_win.zip spaceshipment_editor_win -r
