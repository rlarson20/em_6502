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
//Basic Architecture
//
//The 6502 microprocessor is a relatively simple 8 bit CPU with only a few
//internal registers capable of addressing at most 64Kb of memory via its 16
//bit address bus. The processor is little endian and expects addresses to be
//stored in memory least significant byte first.
//
//The first 256 byte page of memory ($0000-$00FF) is referred to as 'Zero Page'
//and is the focus of a number of special addressing modes that result in
//shorter (and quicker) instructions or allow indirect access to the memory.
//The second page of memory ($0100-$01FF) is reserved for the system stack and
//which cannot be relocated.
//The only other reserved locations in the memory map are the
//very last 6 bytes of memory $FFFA to $FFFF which must be programmed
//with the addresses of the non-maskable interrupt handler ($FFFA/B), the
//power on reset location ($FFFC/D) and the BRK/interrupt request handler ($FFFE/F) respectively.
//The 6502 does not have any special support of hardware devices
//so they must be mapped to regions of memory in order to exchange data with the hardware latches.
//
//END PRELUDE

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

//6502 has 3 primary 8-bit registers, A, X, and Y
//A: accumulator
//X: Register
//Y: Register
//and contains
//stkp: stack pointer
//pc: program counter
//status: status register
struct CPU {
    a: u8,      //accumulator
    x: u8,      //x register
    y: u8,      //y register
    stkp: u8,   //stack pointer
    pc: u16,    //program counter
    status: u8, //status register
}

impl RAM {
    fn read(&self, addr: u16, data: u8) -> u8 {
        if addr >= 0x0000 && addr <= 0xFFFF {
            return self.mem[addr as usize];
        }
        0
    }
    fn write(&mut self, data: u8, addr: u16, read_only: bool) {
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.mem[addr as usize] = data;
        }
    }
    fn new() -> RAM {
        RAM { mem: [0; 65536] }
    }
}
struct RAM {
    mem: [u8; 65536],
}

fn main() {}
