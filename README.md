# Chip 8 Emulator
## Instruction list
```
V - 0nnn - SYS addr
V - 00E0 - CLS
V - 00EE - RET
V - 1nnn - JP addr
V - 2nnn - CALL addr
V - 3xkk - SE Vx, byte
V - 4xkk - SNE Vx, byte
V - 5xy0 - SE Vx, Vy
V - 6xkk - LD Vx, byte
V - 7xkk - ADD Vx, byte
V - 8xy0 - LD Vx, Vy
V - 8xy1 - OR Vx, Vy
V - 8xy2 - AND Vx, Vy
V - 8xy3 - XOR Vx, Vy
V - 8xy4 - ADD Vx, Vy
V - 8xy5 - SUB Vx, Vy
V - 8xy6 - SHR Vx {, Vy}
V - 8xy7 - SUBN Vx, Vy
V - 8xyE - SHL Vx {, Vy}
V - 9xy0 - SNE Vx, Vy
V - Cxkk - RND Vx, byte
V - Dxyn - DRW Vx, Vy, nibble
V - Ex9E - SKP Vx
V - ExA1 - SKNP Vx
X - Fx07 - LD Vx, DT
X - Fx0A - LD Vx, K
X - Fx15 - LD DT, Vx
X - Fx18 - LD ST, Vx
X - Fx1E - ADD I, Vx
X - Fx29 - LD F, Vx
X - Fx33 - LD B, Vx
X - Fx55 - LD [I], Vx
X - Fx65 - LD Vx, [I]
```