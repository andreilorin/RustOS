// compile the project run this inside ~/Projects/Rust/rust_os
RUST_TARGET_PATH=$(pwd) xargo build --target x86_64-rust_os

// create a boot image
bootimage --target x86_64-rust_os

// run boot image inside emulator
qemu-system-x86_64 -drive format=raw,file=bootimage.bin

// dependencies to install in linux before starting the tutorial
// more info in bookmarks: OS/RustOS