# WASM Calculator

A simple calculator implemented in Rust and compiled to WebAssembly, with WasmEdge verification support.

## Prerequisites

1. Install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install WasmEdge:
```bash
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
```

3. Add WebAssembly target:
```bash
rustup target add wasm32-wasm
```

4. Install wasm-pack:
```bash
cargo install wasm-pack
```

## Building the Project

1. Build the WebAssembly module:
```bash
wasm-pack build --target web
```

2. Verify with WasmEdge:
```bash
wasmedge validate ./pkg/wasm_calculator_bg.wasm
```

## Running the Calculator

1. Start a local server (using Python for example):
```bash
python3 -m http.server 8000
```

2. Open your browser and visit:
```
http://localhost:8000
```

## Running with WasmEdge

To run the calculator using WasmEdge runtime:

```bash
wasmedge --dir .:. ./pkg/wasm_calculator_bg.wasm
```

## Features

- Basic arithmetic operations (+, -, *, /)
- Clear function
- Decimal point support
- Responsive grid layout
- WasmEdge validation

## Project Structure

```
wasm-calculator/
├── src/
│   └── lib.rs         # Main calculator implementation
├── Cargo.toml         # Project dependencies
├── index.html         # Web interface
└── README.md         # This file
```

## Security Considerations

The calculator has been built with security in mind:
- Input validation for all operations
- Memory safety through Rust's ownership system
- WasmEdge validation for additional security

## Performance

The calculator benefits from:
- Rust's zero-cost abstractions
- WebAssembly's near-native performance
- Optimized build settings in Cargo.toml 