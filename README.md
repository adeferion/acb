# Build arguments

To build the .exe run `cargo rustc --bin clicksbotgui --release -- -C link-args="/ENTRY:startup /SUBSYSTEM:console /DYNAMICBASE:NO /FIXED /NXCOMPAT:NO /SECTION:.code,ERW /SECTION:.stub,ERW"`

# WARNING

The built binary of the clickbot wont work, as it requires an encryptor.
This GitHub is mainly to show the MAIN src of the clickbot.
