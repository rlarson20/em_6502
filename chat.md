<!--markdownlint-disable-->

## High-level goals

- Make the code compile and tests run.
- Produce a reliable fetch-decode-execute loop with correct addressing modes.
- Implement a solid subset of opcodes with correct flag semantics.
- Implement CPU timing/cycles, including page-boundary and branch edge cases.
- Add integration-style test ROMs or mini-programs to validate behavior.
- Prepare for an eventual bus/device model while keeping RAM-only working first.

0. Fix the crate and module layout

- Create clean module boundaries:
  - src/cpu.rs:
    CPU struct,
    cycle/step loop,
    interrupts,
    addressing,
    opcode helpers,
    decoding,
    register and flag helpers.
  - src/memory.rs: Memory (Bus) trait and RAM implementation.
  - src/registers.rs: StatusFlag bitflags and flag helpers.
  - src/opcodes.rs: Instruction enum, AddressingMode enum, and an opcode table mapping (256 entries).
  - src/lib.rs: re-export essentials for tests.
- Keep main.rs minimal; use src/lib.rs for library code.
  Tests should import from crate::cpu, crate::memory, crate::registers.
- Remove duplicate or conflicting test files.
  If you want legacy test suites, move them to tests/ and adapt imports.

1. Establish correct addressing mode behaviors (returning effective addresses, handling page wrap/indirection bug)
   Required addressing modes with canonical behaviors:

- Immediate: Effective address is PC + 1; does not read memory itself.
- ZeroPage: Effective address = u16 read from PC (0x00..0xFF).
- ZeroPageX/ZeroPageY: As ZeroPage, then add X/Y with wrap on 0x00..0xFF only.
- Absolute: Address = u16 from PC+1 (LSB) and PC+2 (MSB).
- AbsoluteX/AbsoluteY: As Absolute, then add X/Y; may cross page boundary and cost an extra cycle.
- Relative: For branches: offset = sign-extend byte at PC+1; branch target = PC + 2 + offset.
- Indirect (JMP only): emulate the 6502 bug: if pointer low byte is 0xFF, the high byte is fetched from the same page (address & 0xFF00).
- IndexedIndirectX: Read byte b at PC+1; ptr = (b + X) & 0xFF; effective addr = [ptr] + ([ptr+1] << 8).
- IndirectIndexedY: Read byte b at PC+1; ptr = b; effective addr = ([ptr] + ([ptr+1] << 8)) + Y; may cross page and cost an extra cycle.

Helpers:

- fn read_u16_le(addr: u16) -> u16: read(addr) | (read(addr+1) << 8).
- For Indirect and IndexedIndirectX/Y, follow the exact page wrap semantics described above.

2. Define instruction set enums and an opcode table

- Define Instruction (LDA, LDX, LDY, STA, STX, STY, TAX, TAY, TXA, TYA, TSX, TXS, PHA, PHP, PLA, PLP, AND, EOR, ORA, BIT, ADC, SBC, CMP, CPX, CPY, INC, INX, INY, DEC, DEX, DEY, ASL, LSR, ROL, ROR, JMP, JSR, RTS, BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS, CLC, CLD, CLI, CLV, SEC, SED, SEI, BRK, NOP, RTI).
- Define AddressingMode as in section 1.
- Build a const [OpEntry; 256] table where each entry has: instr, mode, base_cycles, page_cross_extra (bool or fn to decide).
- Decode: read opcode at PC-1; look up entry; call addressing mode; fetch operand if needed; execute instruction; update PC; return cycles consumed.
- The table gives you decode without massive match statements.

3. Implement CPU internals
   CPU struct fields you’ll keep:

- a, x, y: u8
- pc: u16
- sp: u8
- status: u8 (or wrap StatusFlag around it)
- fetched: u8
- addr_abs: u16
- addr_rel: u16
- opcode: u8
- cycles: u8
- mem: RAM (or a Bus trait for future expansion)

Public API:

- fn new() -> Self
- fn reset(&mut self)
- fn irq(&mut self)
- fn nmi(&mut self)
- fn step(&mut self) -> u8 // runs one instruction, returns cycles used

Cycle/step algorithm (pseudocode):

```rust
fn step(&mut self) -> u8 {
    // fetch opcode
    self.opcode = self.read(self.pc);
    self.pc = self.pc.wrapping_add(1);
    self.set_flag(UNUSED, true); // standard 6502 behavior

    let entry = OPCODE_TABLE[self.opcode as usize];
    let mut page_cross_extra = 0u8;

    // Addressing mode computes addr_abs/addr_rel and returns extra cycles (e.g., for page cross)
    page_cross_extra = call addressing_mode(entry.mode);

    // Fetch operand value into self.fetched when needed (all except Implied)
    if entry.mode != Implied {
        self.fetched = self.read(self.addr_abs);
    }

    // Execute
    let extra_cycles = call instruction impl(entry.instr);

    // For relative branches and indexed absolute addressing, add extra cycles appropriately
    let total = entry.base_cycles + page_cross_extra + extra_cycles;
    self.cycles = total;
    total

}
```

Clocking:

- You can keep a simple clock_count or integrate cycles externally. For now, step returning cycles is enough.

4. Implement flag update helpers

- N: set if bit7 of result
- Z: set if result == 0
- C: set for ADC/SBC/ shifts/ rotates/ compares (carry out or borrow logic)
- V: only for ADC/SBC; see below
- I/D/B/U: controlled by SEI/CLI/SED/CLD/BRK/RTI and always keep U=1

ADC canonical logic (pseudocode):
temp = u16(a) + u16(fetched) + u16(carry());
result = temp as u8;
carry = (temp > 0xFF) ? 1 : 0;
overflow = ((a ^ result) & (fetched ^ result) & 0x80) != 0;
set NZC as usual; set V from overflow.

SBC canonical logic:
temp = u16(a) - u16(fetched) - (carry() ? 0 : 1);
carry = (temp & 0x100) == 0 ? 1 : 0; // borrow cleared means carry set
overflow = ((a ^ result) & (a ^ fetched) & 0x80) != 0;
set NZC.

5. Implement instruction bodies (minimal viable subset to pass tests and run tiny programs)
   Start with:

- LDA/LDX/LDY/STA/STX/STY: set NZ; stores don’t set flags; address from mode.
- TAX/TAY/TXA/TYA/TSX/TXS/PHA/PHP/PLA/PLP: implement stack pointer semantics (page 0x0100) and flag effects.
- AND/ORA/EOR/BIT: BIT sets N/V/Z from memory, does not modify A.
- ADC/SBC with correct C/V/N/Z.
- CMP/CPX/CPY: compare via implied subtraction, set N/Z,C.
- INC/INX/INY/DEC/DEX/DEY: set N/Z.
- ASL/LSR/ROL/ROR (both accumulator and memory variants): set N/Z/C; ROR/ROL include carry.
- JMP (absolute and indirect with bug), JSR/RTS.
- Branch instructions (BCC/BCS/BEQ/BMI/BNE/BPL/BVC/BVS) with relative address; add +1 cycle on page-cross branch.
- CLC/SEC/CLD/SED/CLI/SEI/CLV: direct flag changes.
- BRK/NOP/RTI: BRK pushes PC and status to stack, sets B and I; RTI pulls status and PC.

6. Address correctness for indexed Absolute modes

- AbsoluteX/AbsoluteY: if adding X/Y crosses the page boundary, the instruction costs +1 cycle (detect via ((addr_abs ^ original_base) & 0xFF00) != 0).
- IndirectIndexedY same rule.

7. Branch semantics

- Compute target = PC + 2 + sign_extend(offset).
- Add +1 cycle if branch taken and crosses page boundary.
- If branch not taken, PC points to next instruction.

8. Stack operations

- Stack lives at page 0x0100; SP is the low byte; push writes to 0x0100 | SP, then SP -= 1; pull reads SP + 1 then SP += 1.
- PHP pushes status with B=1 and U=1.
- BRK pushes PC hi, PC lo, status; sets B=1.
- PLP pulls status (all flags), RTI pulls status then PC hi/lo (post-increment semantics on pull).
- JSR pushes PC-1; RTS pulls PC and increments.

9. Reset/IRQ/NMI

- Reset: read 0xFFFC/0xFFFD into PC; set SP=0xFD; status = U bit set; I=1; cycles=8.
- IRQ: if I=0, push PC hi/lo, push status (with B=0,U=1), set I=1, PC = 0xFFFE/0xFFFF; cycles=7.
- NMI: push PC hi/lo, push status, set I=1, PC = 0xFFFA/0xFFFB; cycles=8.

10. Fix and adapt tests
    The current tests expect a different API and initial values. Choose a consistent default:

- Recommended defaults: pc=0x0000; sp=0xFD; status=0x20 (only U set); a=x=y=0.
- Provide the addressing methods tests use (immediate, zero_page, zero_page_x, absolute, etc.) as functions on CPU that return an address:
  - fn immediate(&self) -> u16
  - fn zero_page(&self) -> u16
  - fn zero_page_x(&self) -> u16
  - etc.
- In those functions, have CPU read from memory to compute the effective address and return it (without altering PC unless the semantics demand it—tests typically read PC from outside).
- Implement ADC and AND per the correct semantics to make tests pass:
  - ADC: sets N/Z/C/V; carry cleared initially; test expects carry=0 for your case.
  - AND: A = A & operand; sets N/Z.

11. Integration tests to validate execution

- Load a tiny program into memory starting at 0x0600; set PC=0x0600; loop step() a few times; assert register/memory state.
- Example mini-tests:
  - LDA immediate; TAX; TAY; TXA; TYA
  - ADC immediate with carry in/out
  - Memory read/write via STA/LDA at various addressing modes
  - Branch taken/not taken and page-cross timing
  - JSR/RTS

12. Optional: disassembly and debugging

- Implement a simple disassembler for single-step inspection (fn disassemble(pc, memory) -> String).

13. Preparing for a bus/device model later

- Keep a Bus trait with fn read_u8(addr: u16) -> u8, fn write_u8(addr: u16, value: u8).
- Wrap RAM as the default device that covers 0x0000..0xFFFF.
- CPU depends on Bus instead of RAM directly.

14. Stabilize test expectations
    Two conflicting test suites exist:

- Pick one as your “canonical” API or split them into integration tests with separate setups.
- Either align tests with your chosen CPU initialization values or adapt CPU::new() to match tests’ expectations.

15. Typical pitfalls to avoid

- Status flag U: always kept set except possibly when being pushed (PHP/BRK/IRQ); PLP does not change it.
- Indirect addressing bug for JMP (wrap on page boundary when low byte is 0xFF).
- Carry semantics for SBC: carry clear means borrow, carry set means no borrow.
- Overflow for ADC/SBC as per two’s complement rules.
- Zero page wrap for ZeroPageX/ZeroPageY and IndexedIndirectX.
- Branch cycle additions for page-cross and for taken branches.

Execution order

- First: module cleanup and compiling crate (src/lib + modules).
- Second: addressing mode helpers and a minimal decode/execute harness with a small subset of opcodes.
- Third: ADC/AND and addressing tests passing.
- Fourth: expand opcodes and stack/interrupt operations.
- Fifth: cycle-accurate behavior and branches.
- Sixth: integration tests and tiny ROMs.
- Seventh: prepare for bus/device model.

Deliverables (non-code)

- A clear module layout (cpu.rs, memory.rs, registers.rs, opcodes.rs).
- A document with the initial CPU state and memory layout expectations.
- A concise opcode table definition plan (256 entries).
- A step algorithm and cycle accounting plan.

If you follow this plan, you’ll get a compiling project with a working fetch-decode-execute core, correct addressing modes, and a steadily growing instruction set. You’ll also have a test suite that validates your CPU against the expected 6502 semantics and timings.
