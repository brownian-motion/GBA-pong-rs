This is a project to see how viable it is to prepare GBA games using Rust sources.

Please see this excellent guide, on which I heavily depended to get started: https://rust-console.github.io/gba/development-setup.html

# Building

You can build this project using the [cargo make plugin](https://github.com/sagiegurari/cargo-make) using the command `cargo make`

You will need to download the `devkitARM` and GBA build tools from the `devkitPro` group, and put `devkitARM/bin` and `tools/bin` on your `PATH`.

# Running

The build script produces a GBA ROM file in `target/rusty-pong.gba`.

You can run this with an emulator of your choice; I use `mGBA` and run that with `cargo make run`.