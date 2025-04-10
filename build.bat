cargo rustc --bin clicksbotgui --release -- -C link-args="/ENTRY:startup /SUBSYSTEM:console /DYNAMICBASE:NO /FIXED /NXCOMPAT:NO /SECTION:.code,ERW /SECTION:.stub,ERW"
cargo run --release -- target/release/clicksbotgui.exe`
@echo off
timeout 60