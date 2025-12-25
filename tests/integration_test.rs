use std::fs;
use std::path::Path;

// Assuming your emulator exposes a CPU or System struct
// use em_6502::{System, Cpu};

#[ignore = "have not really started on this project"]
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
