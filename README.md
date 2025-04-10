# Build arguments

To build the .exe run `cargo rustc --bin clicksbotgui --release -- -C link-args="/ENTRY:startup /SUBSYSTEM:console /DYNAMICBASE:NO /FIXED /NXCOMPAT:NO /SECTION:.code,ERW /SECTION:.stub,ERW"`

# Clicksbot Encrypter

This is neccesary to build a functional version of adf's Clickbot.

To use this, run `cargo run --release -- target/release/clicksbotgui.exe` to build the encrypted version of `clicksbotgui.exe`

To run the clickbot put it in a folder with a `silent.wav` and a `click_types` folder with *at least* one clickpack.