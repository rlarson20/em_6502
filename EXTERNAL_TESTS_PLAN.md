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

## Next Steps

1.  Implement the core `Cpu` and `Memory` structs in `src/`.
2.  Uncomment and adapt the code in `tests/integration_test.rs`.
3.  Run `cargo test` to verify the integration.
