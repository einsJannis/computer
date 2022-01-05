# Computer Specification

## Architecture
8-bit arithmetic
16-bit address

## OP Codes
(0): NOP
### Memory OP Codes
(1): MOV reg, reg/lit8
(2): LDW reg, [HL/lit16]
(3): STW [HL/lit16], reg
(4): LDA [HL/lit16]
(5): PSH lit8/reg
(6): POP reg
### CONTROL FLOW
(7): JMP lit3, [HL/lit16]
### Arithmetic
(8): ADD reg, lit8/reg
(9): SUB reg, lit8/reg
(A): AND reg, lit8/reg
(B): OR  reg, lit8/reg
(C): INV reg
(D): CMP reg, lit8/reg
(E): SHL reg, lit8/reg
(F): SHR reg, lit8/reg

## OP Format
|OPC |F|reg|            |pddng|reg|            |lit8    |                            |lit16            |
|    | |lit| when F = 0 |     |   | when F = 1 |        | when OPC = LDW&F|STW&F|LDA |lit8    |lit8    |
|XXXX|X|XXX|            |     |XXX|            |XXXXXXXX|                            |XXXXXXXX|XXXXXXXX|

## Register
reg0 DATA
reg1 DATA
reg2 DATA
H HIGH_BYTE
L LOW_BYTE
pc_h HIGH_BYTE
pc_l HIGH_BYTE
flag FLAG
  CARRY
  BORROW
  OVERFLOW
  LESS
  EQUAL
  
