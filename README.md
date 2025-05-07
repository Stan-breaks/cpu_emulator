# CHIP‑8 CPU Emulator

A minimal CHIP‑8–style CPU emulator written in Rust. This emulator features:

- 16 general‑purpose 8‑bit registers (V0 through VF, with VF as a carry flag)
- 4 KB (4096 bytes) of addressable memory
- A 16‑level call stack for subroutine management
- Core instructions: HALT, RETURN, CALL, and ADD (with carry)

This README covers:

- [Overview](#overview)
- [Table of Contents](#table-of-contents)
- [CPU Structure](#cpu-structure)
- [Instruction Fetch & Decoding](#instruction-fetch--decoding)
- [Core Instruction Set](#core-instruction-set)
- [Example Program & `main()`](#example-program--main)
- [Running & Testing](#running--testing)
- [Extending the CPU](#extending-the-cpu)
- [License](#license)

---

## Overview

This project implements a simplified virtual CPU inspired by the CHIP‑8 architecture. It demonstrates:

1. How to organize registers, memory, and a stack in Rust.
2. A basic fetch‑decode‑execute loop.
3. Stack‑based subroutine calls and returns.
4. Arithmetic with carry flag handling.

Use it as a learning tool or foundation for a more complete CHIP‑8 emulator.

---

## Table of Contents

- [CPU Structure](#cpu-structure)
- [Instruction Fetch & Decoding](#instruction-fetch--decoding)
- [Core Instruction Set](#core-instruction-set)
- [Example Program & `main()`](#example-program--main)
- [Running & Testing](#running--testing)
- [Extending the CPU](#extending-the-cpu)
- [License](#license)

---

## CPU Structure

```rust
struct CPU {
    /// 16 general‑purpose 8‑bit registers (V0–VF; VF = carry flag)
    registers: [u8; 16],

    /// Program counter (index into memory)
    position_in_memory: usize,

    /// 4 KB (4096 bytes) of memory
    memory: [u8; 0x1000],

    /// Call stack (stores return addresses)
    stack: [u16; 16],

    /// Stack pointer (next free slot in `stack`)
    stack_pointer: usize,
}
```

- **`registers`**: V0–V14 are general‑purpose; VF is used for carry/overflow flags.
- **`memory`**: 4 096 bytes for instructions and data.
- **`stack`** & **`stack_pointer`**: manage subroutine calls/returns.

---

## Instruction Fetch & Decoding

### `read_opcode()`

Reads two bytes from `memory` at `position_in_memory` and combines them into a single 16‑bit opcode.

```rust
fn read_opcode(&self) -> u16 {
    let p = self.position_in_memory;
    let hi = self.memory[p]   as u16;
    let lo = self.memory[p+1] as u16;
    (hi << 8) | lo
}
```

### `run()` Loop

```rust
fn run(&mut self) {
    loop {
        let opcode = self.read_opcode();
        self.position_in_memory += 2;

        // Decode into nibbles
        let c   = ((opcode & 0xF000) >> 12) as u8;
        let x   = ((opcode & 0x0F00) >>  8) as u8;
        let y   = ((opcode & 0x00F0) >>  4) as u8;
        let d   = ( opcode & 0x000F)        as u8;
        let nnn = opcode & 0x0FFF;

        match (c, x, y, d) {
            (0, 0, 0, 0)    => return,               // HALT
            (0, 0, 0xE, 0xE) => self.ret(),          // RETURN
            (0x2, _, _, _)  => self.call(nnn as u16), // CALL addr
            (0x8, _, _, 0x4) => self.add_xy(x, y),    // ADD VY to VX
            _               => todo!("opcode {:04x}", opcode),
        }
    }
}
```

---

## Core Instruction Set

### 0x0000 — HALT

Stops execution and exits `run()`.

```rust
// (c, x, y, d) == (0,0,0,0)
```

### 0x00EE — RETURN

Pop return address from `stack` and jump back.

```rust
fn ret(&mut self) {
    if self.stack_pointer == 0 { panic!("Stack underflow"); }
    self.stack_pointer -= 1;
    self.position_in_memory = self.stack[self.stack_pointer] as usize;
}
```

### 0x2NNN — CALL NNN

Push current `position_in_memory` to `stack`, then jump to `NNN`.

```rust
fn call(&mut self, addr: u16) {
    if self.stack_pointer >= self.stack.len() {
        panic!("Stack overflow!");
    }
    self.stack[self.stack_pointer] = self.position_in_memory as u16;
    self.stack_pointer += 1;
    self.position_in_memory = addr as usize;
}
```

### 0x8XY4 — ADD VY → VX

Add register Y into X, set VF = carry flag.

```rust
fn add_xy(&mut self, x: u8, y: u8) {
    let (sum, carry) = self.registers[x as usize]
        .overflowing_add(self.registers[y as usize]);
    self.registers[x as usize] = sum;
    self.registers[0xF] = if carry { 1 } else { 0 };
}
```

---

## Example Program & `main()`

Demonstrates two subroutine calls to add V1 to V0 twice, starting from V0=5, V1=10.

```rust
fn main() {
    let mut cpu = CPU {
        registers: [0;16], memory: [0;4096],
        position_in_memory: 0, stack: [0;16], stack_pointer: 0
    };

    // Initialize registers
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // At 0x000: CALL 0x100 twice, then HALT
    cpu.memory[0x000..0x006].copy_from_slice(&[0x21,0x00, 0x21,0x00, 0x00,0x00]);

    // Subroutine at 0x100: ADD V1→V0 twice, then RETURN
    cpu.memory[0x100..0x106].copy_from_slice(&[0x80,0x14, 0x80,0x14, 0x00,0xEE]);

    cpu.run();
    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
```

---

## Running & Testing

1. **Build:** `cargo build --release`
2. **Run:** `cargo run`
3. **Validate:** The assertion in `main()` will panic if V0 != 45.

---

## Extending the CPU

To expand toward a full CHIP‑8 emulator, consider:

- **Graphics & Input:** Implement draw ops (0xDXYN) and keypad (EX9E/EXA1).
- **Timers:** Add delay & sound timers.
- **Memory Ops:** Support FX55 (dump registers) and FX65 (load registers).
- **Random:** Implement CXNN for random numbers.
- **ROM Loading:** Load binary ROM files into memory.
- **Cycle Counting:** Track CPU cycles per instruction.

---

## License

This project is released under the MIT License. See [LICENSE](LICENSE) for details.
