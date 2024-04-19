echo "Building"
cargo build --release

echo "Copying files"
mkdir orbits_editor_linux

cp levels orbits_editor_linux/ -r
cp target/release/orbits_editor orbits_editor_linux/orbits_editor.x86_64
cp README.md orbits_editor_linux/

echo "Zipping up"
rm orbits_editor_linux.zip
zip orbits_editor_linux.zip orbits_editor_linux -r

echo "Cleaning up"
rm orbits_editor_linux/ -r
