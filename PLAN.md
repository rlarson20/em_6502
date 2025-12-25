# Integration Plan for Klaus Dolmann's 6502 Tests

## Overview

This plan details how to integrate the Klaus Dolmann 6502 functional tests into the `em_6502` Rust project. The tests are provided as binary memory images in the `tests/external_tests` submodule.

## Test Suite Details

- **Source**: `tests/external_tests` (submodule)
- **Primary Test Binary**: `tests/external_tests/bin_files/6502_functional_test.bin`
- **File Size**: 65,536 bytes (64KB full memory image)
- **Start Address (PC)**: `0x0400`
- **Success Address (PC)**: `0x3469` (Infinite loop at this address indicates success)
- **Failure Condition**: Any infinite loop at a different address.

## Integration Strategy

### 1. Test Runner (`tests/integration_test.rs`)

Create an integration test file `tests/integration_test.rs` that acts as a harness for the emulator.

**Pseudo-code Implementation:**

```rust
// tests/integration_test.rs
use std::fs;
use std::path::Path;

// Assuming your emulator exposes a CPU or System struct
// use em_6502::{System, Cpu}; 

#[test]
fn test_klaus_functional_test() {
    let bin_path = Path::new("tests/external_tests/bin_files/6502_functional_test.bin");
    let bin_data = fs::read(bin_path).expect("Failed to read test binary");

    // Initialize Emulator Memory
    // let mut system = System::new();
    // system.load_memory(0x0000, &bin_data);

    // Initialize CPU
    // let mut cpu = Cpu::new(&mut system);
    
    // Set Program Counter to Start Address
    // cpu.set_pc(0x0400);

    // Run Loop
    // Limit cycles to prevent infinite loops in case of errors (e.g., 100 million cycles)
    // const MAX_CYCLES: u64 = 100_000_000;
    // let mut cycles_executed = 0;

    // while cycles_executed < MAX_CYCLES {
    //     cpu.step(); 
    //     cycles_executed += 1; // Or add actual instruction cycles
    //
    //     if cpu.pc() == 0x3469 {
    //         // Success!
    //         return;
    //     }
    //
    //     // Optional: Check for known trap addresses or previous PC == current PC (loop)
    //     // if cpu.pc() == previous_pc {
    //     //      panic!("Test trapped at address {:#06x}", cpu.pc());
    //     // }
    // }

    // panic!("Test timed out after {} cycles. Last PC: {:#06x}", cycles_executed, cpu.pc());
}
```

### 2. Development Workflow

#### Continuous Integration (GitHub Actions)
A workflow has been created at `.github/workflows/rust.yml` which:
- Checks out the repository (including submodules).
- Runs `cargo build`.
- Runs `cargo test`.
- Checks formatting (`cargo fmt`).
- Runs linter (`cargo clippy`).

#### Local Development (`bacon`)
A `bacon.toml` file has been added. Run `bacon` to start the background checker.
- `bacon`: Runs `cargo check`.
- `bacon test`: Runs `cargo test` (will include the integration test once enabled).
- `bacon clippy`: Runs clippy.

#### Pre-commit Hooks
To ensure code quality before committing, use a git hook.
Create `.git/hooks/pre-commit` with:

```bash
#!/bin/sh
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

Make it executable: `chmod +x .git/hooks/pre-commit`.

## Next Steps

1.  Implement the core `Cpu` and `Memory` structs in `src/`.
2.  Uncomment and adapt the code in `tests/integration_test.rs`.
3.  Run `cargo test` to verify the integration.
