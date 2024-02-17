//note: did not write the contents of this file,
//using it as a reference while writing this up

// Opcodes ======================================================
// There are 56 "legitimate" opcodes provided by the 6502 CPU. I
// have not modelled "unofficial" opcodes. As each opcode is
// defined by 1 byte, there are potentially 256 possible codes.
// Codes are not used in a "switch case" style on a processor,
// instead they are repsonisble for switching individual parts of
// CPU circuits on and off. The opcodes listed here are official,
// meaning that the functionality of the chip when provided with
// these codes is as the developers intended it to be. Unofficial
// codes will of course also influence the CPU circuitry in
// interesting ways, and can be exploited to gain additional
// functionality!
//
// These functions return 0 normally, but some are capable of
// requiring more clock cycles when executed under certain
// conditions combined with certain addressing modes. If that is
// the case, they return 1.
//
// I have included detailed explanations of each function in
// the class implementation file. Note they are listed in
// alphabetical order here for ease of finding.
//
// Opcode source: obelisk, OLC's NES video #2
// has relatively basic instructions, many doing similar shit,
// all listed below in groups for bookkeeping
// info from below group taken from obelisk

enum Opcode {
    //Load and Store Instructions
    //These instructions transfer a single byte between memory and one of the
    //registers. Load operations set the negative (N) and zero (Z) flags
    //depending on the value of transferred.
    //Store operations do not affect the flag settings.
    LDA, //Load Accumulator, N,Z
    LDX, //Load X Register, N,Z
    LDY, //Load Y Register, N,Z
    STA, //Store Accumulator
    STX, //Store X Register
    STY, //Store Y Register
    //Register Transfers
    //The contents of the X and Y registers can be moved to or from the
    //accumulator, setting the negative (N) and zero (Z) flags as appropriate.
    TAX, //Transfer Accumulator to X, N,Z
    TAY, //Transfer Accumulator to Y, N,Z
    TXA, //Transfer X to Accumulator, N,Z
    TYA, //Transfer Y to Accumulator, N,Z
    //Stack Operations
    //The 6502 microprocessor supports a 256 byte stack fixed between memory
    //locations $0100 and $01FF. A special 8-bit register, S, is used to keep
    //track of the next free byte of stack space. Pushing a byte on to the
    //stack causes the value to be stored at the current free location
    //(e.g. $0100,S) and then the stack pointer is post decremented.
    //Pull operations reverse this procedure.
    //
    //The stack register can only be accessed by transferring its value to or
    //from the X register. Its value is automatically modified by push/pull
    //instructions, subroutine calls and returns, interrupts
    //and returns from interrupts.
    TSX, //Transfer Stack Pointer to X, N,Z
    TXS, //Transfer X to Stack Pointer
    PHA, //Push Accumulator to Stack
    PHP, //Push Processor Status to Stack
    PLA, //Pull Accumulator from Stack, N,Z
    PLP, //Pull Processor Status from Stack, All
    //Logical
    //The following instructions perform logical operations on the contents of
    //the accumulator and another value held in memory. The BIT instruction
    //performs a logical AND to test the presence of bits in the memory value
    //to set the flags but does not keep the result.
    AND, //logical AND, N,Z
    EOR, //XOR, N,Z
    ORA, //inclusive OR, N,Z
    BIT, //Bit test, N,V,Z
    //Arithmetic
    //The arithmetic operations perform addition and subtraction on the
    //contents of the accumulator. The compare operations allow the comparison
    //of the accumulator and X or Y with memory values.
    ADC, //Add with Carry, N,V,Z,C
    SBC, //Subtract with Carry, N,V,Z,C
    CMP, //Compare accumulator, N,Z,C
    CPX, //Compare X, N,Z,C
    CPY, //Compare Y, N,Z,C
    //Increments and Decrements
    //Increment or decrement a memory location or one of the X or Y registers
    //by one setting the negative (N) and zero (Z) flags as appropriate
    INC, //Increment memory location, N,Z
    INX, //Increment X Register, N,Z
    INY, //Increment Y Register, N,Z
    DEC, //Decrement memory location, N,Z
    DEX, //Decrement X Register, N,Z
    DEY, //Decrement Y Register, N,Z
    //Shifts
    //Shift instructions cause the bits within either a memory location or the
    //accumulator to be shifted by one bit position. The rotate instructions
    //use the contents if the carry flag (C) to fill the vacant position
    //generated by the shift and to catch the overflowing bit.
    //The arithmetic and logical shifts shift in an appropriate 0 or 1 bit as
    //appropriate but catch the overflow bit in the carry flag (C).
    ASL, //Arithmetic Shift Left, N,Z,C
    LSR, //Logical Shift Right, N,Z,C
    ROL, //Rotate Left, N,Z,C
    ROR, //Rotate Right, N,Z,C
    //Jumps and Calls
    //The following instructions modify the program counter causing a break to
    //normal sequential execution. The JSR instruction pushes the old PC onto
    //the stack before changing it to the new location allowing a subsequent
    //RTS to return execution to the instruction after the call.
    JMP, //Jump to location
    JSR, //Jump to Subroutine
    RTS, //Return from Subroutine
    //Branches
    //Branched instructions break the normal sequential flow of execution by
    //changing the program counter if a specified condition is met.
    //All the conditions are based on examining a single bit within the
    //processor status.
    //
    //Branch instructions use relative address to identify the target
    //instruction if they are executed. As relative addresses are stored using
    //a signed 8 bit byte the target instruction must be within 126 bytes
    //before the branch or 128 bytes after the branch.
    BCC, //Branch if Carry Clear
    BCS, //Branch if Carry Set
    BEQ, //Branch if Zero Set
    BMI, //Branch if Negative Set
    BNE, //Branch if Zero Clear
    BPL, //Branch if Negative Clear
    BVC, //Branch if Overflow Clear
    BVS, //Branch if Overflow Set
    //Status Flag Changes
    //The following instructions change the values of specific status flags.
    CLC, //Clear Carry, C
    CLD, //Clear Decimal Mode, D
    CLI, //Clear Interrupt Disable, I
    CLV, //Clear Overflow, V
    SEC, //Set Carry, C
    SED, //Set Decimal Mode, D
    SEI, //Set Interrupt Disable, I
    //System Functions
    BRK, //Force Interrupt, B
    NOP, //No Operation
    RTI, //Return from Interrupt, All
}
