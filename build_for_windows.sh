echo "Building"
cargo build --release --target x86_64-pc-windows-gnu

echo "Copying Files"
cp assets orbits_editor_win/ -r
cp target/x86_64-pc-windows-gnu/release/orbits_editor.exe orbits_editor_win/
