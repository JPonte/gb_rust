# gb_rust
Gameboy emulator in Rust. Just for fun.

### Run instructions
To run locally with minifb: 
```
cargo run
```

To run the web version:
```
cd gb_wasm
wasm-pack build
cd www
npm run start
```

### TODOs:
 - Window overlay
 - Timer and Divider Registers/Interrupts
 - Proper sprite transparency
 - MBC 2 to 7
 - ...
