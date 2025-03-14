# gameboy-emulator
A simple GameBoy emulator written in Rust just for fun

## Progress:

- [ ] MVP
    - [x] Cartridge header reading
    - [ ] Cartridge reading
    - [x] Instruction decoding
    - [ ] Instruction execution
    - [ ] Execute Boot ROM
    - [ ] Execute all cpu_instrs.gb tests
    - [ ] Execute dmg-acid2.gb
    - [ ] Play Tetris

- [x] Instruction decoding
    - [x] All instruction opcodes
    - [x] All instruction arguments
    - [x] All instruction lengths
- [ ] Instruction execution
    - [ ] Block 0
        - [x] nop
        - [x] ld r16, imm16
        - [x] ld \[r16mem\], a
        - [x] ld a, \[r16mem\]
        - [x] ld \[imm16\], sp
        - [x] inc r16
        - [ ] dec r16
        - [x] add hl, r16
        - [x] inc r8
        - [ ] dec r8
        - [x] ld r8, imm8
        - [x] rlca
        - [ ] rrca
        - [ ] rla
        - [ ] rra
        - [ ] daa
        - [ ] cpl
        - [ ] scf
        - [ ] ccf
    - [ ] Block 1: 8-Bit Register-to-Register Loads
        - [ ] ld r8, r8
        - [ ] halt
    - [ ] Block 2: 8-Bit Arithmetic
        - [ ] add a, r8
        - [ ] adc a, r8
        - [ ] sub a, r8
        - [ ] sbc a, r8
        - [ ] and a, r8
        - [x] xor a, r8
        - [ ] or a, r8
        - [ ] cp a, r8
    - [ ] Block 3
        - [ ] add a, imm8
        - [ ] adc a, imm8
        - [ ] sub a, imm8
        - [ ] sbc a, imm8
        - [ ] and a, imm8
        - [ ] xor a, imm8
        - [ ] or a, imm8
        - [ ] cp a, imm8
        - [ ] ret cond
        - [ ] ret
        - [ ] reti
        - [x] jp cond, imm16
        - [x] jp imm16
        - [ ] jp hl
        - [ ] call cond, imm16
        - [ ] call imm16
        - [ ] rst tgt3
        - [ ] pop r16stk
        - [ ] push r16stk
        - [ ] ldh \[c\], a
        - [ ] ldh \[imm8\], a
        - [ ] ld \[imm16\], a
        - [ ] ldh a, \[c\]
        - [ ] ldh a, \[imm8\]
        - [ ] ld a, \[imm16\]
        - [ ] add sp, imm8
        - [ ] ld hl, sp + imm8
        - [ ] ld sp, hl
        - [ ] di
        - [ ] ei
    - 0xCB Prefixed Instructions
        - [ ] rlc r8
        - [ ] rrc r8
        - [ ] rl r8
        - [ ] rr r8
        - [ ] sla r8
        - [ ] sra r8
        - [ ] swap r8
        - [ ] srl r8
        - [x] bit b3, r8
        - [ ] res b3, r8
        - [ ] set b3, r8
- [ ] Memory Map
    - [x] Boot ROM
    - [x] Cartrige ROM Bank 0
    - [x] Cartrige ROM Bank 1
    - [ ] Cartridge ROM Bank Swapping
    - [x] VRAM
    - [ ] Cartridge RAM
    - [ ] Cartridge RAM Bank Swapping
    - [ ] Work RAM Bank 0
    - [ ] Work RAM Bank 1
    - [ ] Echo RAM
    - [ ] OAM RAM
    - [ ] HRAM
    - [ ] Interrupt Enable Register
- [ ] I/O Registers
    - [x]  Joypad Input
    - [x]  Serial transfer
    - [ ]  Timer and Divider
    - [ ]  Interrupts
    - [ ]  Audio
    - [ ]  Wave Pattern
    - [ ]  LCD / Display
    - [ ]  ~~VRAM Bank Select~~ (CGB)
    - [ ]  Boot ROM Enable
    - [ ]  ~~VRAM DMA~~ (CGB)
    - [ ]  ~~LCD Color Palettes~~ (CGB)
    - [ ]  ~~WRAM Bank Select~~ (CGB)
- [x] Cartridge Header Reading
    - [x] Entry Point
    - [x] Nintendo Logo
    - [x] Title
    - [x] Manufacturer Code
    - [x] ~~Color GameBoy Flag~~
    - [x] New Licensee Code
    - [x] ~~Super GameBoy Flag~~
    - [x] Cartridge Type
    - [x] ROM Size
    - [x] RAM Size
    - [x] Destination Code
    - [x] Old Licensee Code
    - [x] Mask ROM Version Number
    - [x] Header Checksum
    - [x] Global Checksum
    - [x] Header Checksum Calculation
    - [ ] Global Checksum Calculation