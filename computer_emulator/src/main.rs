use std::intrinsics::unreachable;
use crate::register::PC_H;

fn main() {
    println!("Hello, world!");
}

mod register {
    pub const REG0: u8 = 0;
    pub const REG1: u8 = 1;
    pub const HIGH: u8 = 2;
    pub const LOW : u8 = 3;
    pub const PC_H: u8 = 4;
    pub const PC_L: u8 = 5;
    pub const SCTR: u8 = 6;
    pub const FLAG: u8 = 7;
}

mod flag {
    pub const HALT: u8 = 0;
    pub const OVERFLOW: u8 = 1;
    pub const CARRY: u8 = 2;
    pub const EQUAL: u8 = 3;
    pub const LESS: u8 = 4;
    pub const MORE: u8 = 5;
}

struct Computer {
    registers: [u8; 2**3],
    ram: [u8; 2**16],
    stack: [u8; 2**8]
}

fn opc(word: u8) -> u8 {
    word >> 4
}

fn op_flag(word: u8) -> bool {
    word >> 3 & 1 != 0
}

fn op_reg(word: u8) -> u8 {
    word & 0b111
}

impl Computer {
    fn set_reg8(&self, register: u8, value: u8) {
        self.registers[register] = value
    }
    fn reg8(&self, register: u8) -> u8 {
        self.registers[register]
    }
    fn set_reg16(&self, register: u8, value: u16) {
        self.registers[register] = value >> 8;
        self.registers[register+1] = value;
    }
    fn reg16(&self, register: u8) -> u16 {
        ((self.registers[register] as u16) << 8) | (self.registers[register+1] as u16)
    }
    fn pc(&self) -> u16 {
        self.reg16(register::PC_H)
    }
    fn pc_inc(&self) { self.set_reg16(register::PC_H, self.pc()+1)}
    fn address(&self) -> u16 {
        self.reg16(register::H)
    }
    fn set_ram8(&self, address: u16, value: u8) {
        self.ram[address] = value
    }
    fn ram8(&self, address: u16) -> u8 {
        self.ram[address]
    }
    fn op_lit8(&self) -> u8 {
        self.ram8(self.pc())
    }
    fn ram16(&self, address: u16) -> u16 {
        (self.ram8(address) as u16) << 8 | (self.ram8(address+1) as u16)
    }
    fn op_lit16(&self) -> u16 {
        self.ram16(self.pc())
    }
    fn op_reg(&self) -> u8 {
        self.op_lit8() & 0b111
    }
    fn op_flag(&self) -> bool {
        self.ram8(self.pc()) >> 3 & 1 != 0
    }
    fn flag(&self, index: u8) -> bool {
        self.reg8(register::FLAG) >> index & 1 != 0
    }
    fn stack_ptr(&mut self) -> u8 { self.reg8(register::SCTR) }
    fn set_stack_ptr(&mut self, value: u8) { self.set_reg8(register::SCTR, value) }
    fn stack_ptr_inc(&mut self) { self.set_stack_ptr(self.stack_ptr()+1) }
    fn stack_ptr_dec(&mut self) { self.set_stack_ptr(self.stack_ptr()-1) }
    fn run(&mut self) {
        while !self.flag(flag::HALT) {
            self.step()
        }
    }
    fn step(&mut self) {
        match opc(self.op_lit8()) {
            0x0 => self.run_nop(),
            0x1 => self.run_mov(),
            0x2 => self.run_ldw(),
            0x3 => self.run_stw(),
            0x4 => self.run_lda(),
            0x5
            _ => unreachable!()
        }
    }
    fn run_nop(self) { }
    fn run_mov(&mut self) {
        self.set_reg8(
            self.op_reg(),
            if self.op_flag() {
                self.pc_inc();
                self.op_lit8()
            } else {
                self.pc_inc();
                self.reg8(self.op_reg())
            }
        );
        self.pc_inc()
    }
    fn run_ldw(&mut self) {
        self.set_reg8(
            self.op_reg(),
            self.ram8(if self.op_flag() {
                self.reg16(register::HIGH)
            } else {
                self.pc_inc();
                self.op_lit16();
                self.pc_inc()
            })
        );
        self.pc_inc();
    }
    fn run_stw(&mut self) {
        let value = self.op_reg();
        let dest = if self.op_flag() {
            self.reg16(register::HIGH)
        } else {
            self.pc_inc();
            self.op_lit16();
            self.pc_inc();
        };
        self.set_ram8(
            dest,
            value
        );
    }
    fn run_lda(&mut self) {
        self.set_reg16(register::HIGH, self.ram16(if self.op_flag() {
            self.reg16(register::HIGH)
        } else {
            self.pc_inc();
            self.op_lit16();
            self.pc_inc();
        }));
        self.pc_inc();
    }
    fn run_psh(&mut self) {
        self.stack[self.stack_ptr()] = self.reg8(self.op_reg());
        self.stack_ptr_inc();
    }
    fn run_pop(&mut self) {
        self.set_reg8(self.op_reg(), self.stack[self.stack_ptr()]);
        self.stack_ptr_dec();
    }
}
