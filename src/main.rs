//START PRELUDE
//
//needed 'signals' for cpu:
//clock, address, data, r/w
//addr output by 6502 is 16 bits (0x0000 - 0xffff)
//data output by 6502 is 8 bits
//r/w is a signal in real life, in code not a signal
//
//also need a bus, which gets a connection from the cpu's data lines,
//and a back and forth data connection
//
//following the OneLoneCoder Nes Emulator tutorial
//but doing it in rust instead of c++
//so an assumption being made is that there's only one device
//connected to the bus, can be read and written to, and covers
//the entire address space, AKA it's ram
//
//64KB of ram specifically
//
//END PRELUDE

//6502 has 3 primary 8-bit registers, A, X, and Y
//A: accumulator
//X: Register
//Y: Register
//and contains
//stkp: stack pointer
//pc: program counter
//status: status register
//
//because of variable byte size of instruction, we need to know
//size and duration of instruction (56 total legal instructions)
//
//more specifically, an instruction composes of a function, an address mode,
//and a number of cycles
//
//basic strat:
//1) read byte at pc
//2) get opcode, addressing mode and cycles from byte at pc
//3) read 0,1,2 more byte
//4) execute
//5) wait, count clocks, complete

//Bus has devices, like CPU, and just for now, has RAM, 64KB
//

enum Flags_6502 {
    C, //Carry Bit
    Z, //Zero
    I, //Disable Interrupts
    D, //Decimal (unused in implementation followed)
    B, //Break
    U, //Unused
    V, //Overflow
    N, //Negative
}

let status = 0x00u8; //status register
let x = 0x00u8; //x register
let y = 0x00u8; //register
let a = 0x00u8; //accumulator
let stkp = 0x00u8; //stack pointer
let pc = 0x0000u16; //program counter

impl Bus {
    fn read(&self, addr: u16, data: u8) -> u8 {
        if addr >= 0x0000 && addr <= 0xFFFF {
            return self.ram.mem[addr as usize];
        }
        0
    }
    fn write(&mut self, data: u8, addr: u16, readOnly: bool) {
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram.mem[addr as usize] = data;
        }
    }
}

struct Bus {
    cpu: CPU,
    ram: RAM,
}

struct CPU {}

impl RAM {
    fn new() -> RAM {
        RAM { mem: [0; 65536] }
    }
}
struct RAM {
    mem: [u8; 65536],
}

fn main() {}
