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

//moving the flags declaration here since that's the only place it's used

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct StatusFlag : u8 {
        const C = 1 << 0; //Carry Bit
        const Z = 1 << 1; //Zero
        const I = 1 << 2; //Disable Interrupts
        const D = 1 << 3; //Decimal
        const B = 1 << 4; //Break
        const U = 1 << 5; //Unused
        const V = 1 << 6; //Overflow
        const N = 1 << 7; //Negative
    }
}

impl StatusFlag {
    fn as_u8(&self) -> u8 {
        self.bits() as u8
    }
}

impl CPU {
    //initialize the CPU
    fn new() -> CPU {
        CPU {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            stkp: 0x00,
            pc: 0x0000,
            status: 0x00,

            fetched: 0x00,
            temp: 0x0000,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00, //opcode byte, probably need it but idk if i actually do
            cycles: 0,
            clock_count: 0,

            memory: RAM::new(), //TODO: make bus
                                //TODO: make a blog post about why I did this project and what i learned from it
        }
    }
    //RAM connectivity (can/prob will expand to a bus later)
    fn read(&self, addr: u16) -> u8 {
        return self.memory.read(addr);
    }
    fn write(&mut self, addr: u16, data: u8) {
        self.memory.write(addr, data)
    }

    //FLAG FUNCTIONS
    fn set_flag(&mut self, flag: StatusFlag, val: bool) {
        if val {
            //if value is true, bitwise or it with the flag, otherwise, bitwise and it with the complement of the flag
            self.status = self.status | flag.as_u8()
        } else {
            self.status = self.status & !flag.as_u8()
        }
    }
    fn get_flag(&self, flag: StatusFlag) -> u8 {
        if (self.status & flag.as_u8()) > 0 {
            return 1;
        }
        return 0;
    }
    //EXTERNAL INPUTS
    fn reset(&mut self) {
        //get addr to set pc to
        self.addr_abs = 0xFFFC;
        let lo: u16 = self.read(self.addr_abs).into();
        let hi: u16 = self.read(self.addr_abs + 1).into();

        //set pc
        self.pc = (hi << 8) | lo;

        //reset internal registers
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = 0x00 | StatusFlag::U.as_u8();

        //clear helpers
        self.addr_abs = 0x0000;
        self.addr_rel = 0x0000;
        self.fetched = 0x00;

        //reseting takes time
        self.cycles = 8
    }
    fn irq(&mut self) {
        //check if interrupts are allowed
        if self.get_flag(StatusFlag::I) == 0 {
            //push pc to stack, have to do twice because it's 16-bit
            self.write(0x0100 + self.stkp, (self.pc >> 8) & 0x00FF);
            self.stkp = self.stkp - 1;
            self.write(0x0100 + self.stkp, self.pc & 0x00FF);
            self.stkp = self.stkp - 1;

            //push status register to stack
            self.set_flag(StatusFlag::B, false);
            self.set_flag(StatusFlag::U, true);
            self.set_flag(StatusFlag::I, true);
            self.write(0x0100 + self.stkp, self.status);
            self.stkp = self.stkp - 1;

            //read new pc location from
            self.addr_abs = 0xFFFE;
            let lo: u16 = self.read(self.addr_abs).into();
            let hi: u16 = self.read(self.addr_abs + 1).into();
            self.pc = (hi << 8) | lo;

            //these take time:
            self.cycles = 7;
        }
    }
    fn nmi(&mut self) {
        //pretty much same as above but doesn't check,
        //also uses addr 0xFFFA
        self.write(0x0100 + self.stkp, (self.pc >> 8) & 0x00FF);
        self.stkp = self.stkp - 1;
        self.write(0x0100 + self.stkp, self.pc & 0x00FF);
        self.stkp = self.stkp - 1;

        self.set_flag(StatusFlag::B, false);
        self.set_flag(StatusFlag::U, true);
        self.set_flag(StatusFlag::I, true);
        self.write(0x0100 + self.stkp, self.status);
        self.stkp = self.stkp - 1;

        self.addr_abs = 0xFFFA;
        let lo: u16 = self.read(self.addr_abs).into();
        let hi: u16 = self.read(self.addr_abs + 1).into();
        self.pc = (hi << 8) | lo;

        self.cycles = 8;
    }

    fn clock(&mut self) {
        if self.cycles == 0 {
            //read next instruction
            self.opcode = self.read(self.pc);

            //always set unused for some reason
            self.set_flag(StatusFlag::U, true);
            //increment pc
            self.pc = self.pc + 1;
            //Get number of cycles: should be from opcode struct
            //perform fetch of intermediate data using required addr mode
            //perform op
            //add any additional clock cycles that the instruction needs
            self.set_flag(StatusFlag::U, true);
        }
        self.clock_count = self.clock_count + 1;
        self.cycles = self.cycles - 1;
    }
}

struct CPU {
    a: u8,      //accumulator
    x: u8,      //x register
    y: u8,      //y register
    stkp: u8,   //stack pointer
    pc: u16,    //program counter
    status: u8, //status register

    fetched: u8,
    temp: u16,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8,
    clock_count: u32,

    memory: RAM, //TODO: change to bus that's connected to ram
}

impl RAM {
    fn read(&self, addr: u16) -> u8 {
        if addr >= 0x0000 && addr <= 0xFFFF {
            return self.mem[addr as usize];
        }
        0
    }
    fn write(&mut self, addr: u16, data: u8) {
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
