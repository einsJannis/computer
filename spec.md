# Computer Specification

## Architecture
8-bit arithmetic
16-bit address

## OP Codes
### Memory OP Codes
MOV reg, reg/lit8
LDW reg, [HL/lit16]
STW [HL/lit16], reg
LDA [HL/lit16]
PSH imm8/reg
POP reg
### CONTROL FLOW
JMP lit3, [HL/lit16]
### Arithmetic
ADD reg, lit8/reg
SUB reg, lit8/reg
AND reg, lit8/reg
OR  reg, lit8/reg
NOT reg
CMP reg, lit8/reg
SHL reg, lit8/reg
SHR reg, lit8/reg

## OP Format
|OPC |F|reg|            |pddng|reg|            |lit8    |                            |lit16            |
|    | |lit| when F = 0 |     |reg| when F = 1 |        | when OPC = LDW&F|STW&F|LDA |lit8    |lit8    |
|XXXX|X|XXX|            |XXXXX|reg|            |XXXXXXXX|                            |XXXXXXXX|XXXXXXXX|

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
  OVERFLOW
  LESS
  EQUAL
  
