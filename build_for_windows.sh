echo "Building"
cargo build --release --target x86_64-pc-windows-gnu

echo "Copying files"
rm orbits_editor_win/levels/ -r
cp levels orbits_editor_win/ -r
cp target/x86_64-pc-windows-gnu/release/orbits_editor.exe orbits_editor_win/

echo "Zipping up"
rm orbits_editor.zip
zip orbits_editor.zip orbits_editor_win -r
