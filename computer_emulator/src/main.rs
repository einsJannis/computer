use std::intrinsics::unreachable;
use crate::register::{HIGH, PC_H};

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
    pub const BORROW: u8 = 3;
    pub const EQUAL: u8 = 4;
    pub const LESS: u8 = 5;
    pub const MORE: u8 = 6;
}

struct Computer {
    registers: [u8; 2**3],
    ram: [u8; 2**16],
    stack: [u8; 2**8]
}

// static utils
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
    fn opc(&self) -> u8 { self.op_lit8() >> 4 }
    fn address(&self) -> u16 {
        self.reg16(register::H)
    }
    fn set_ram8(&self, address: u16, value: u8) {
        self.ram[address] = value
    }
    fn ram8(&self, address: u16) -> u8 {
        self.ram[address]
    }
    fn ram16(&self, address: u16) -> u16 {
        (self.ram8(address) as u16) << 8 | (self.ram8(address+1) as u16)
    }
    fn set_flag(&self, index: u8, value: bool) {
        self.set_reg8(register::FLAG, if value {
            self.reg8(register::FLAG) | (1 << index)
        } else {
            self.reg8(register::FLAG) & !(1 << index)
        })
    }
    fn flag(&self, index: u8) -> bool {
        self.reg8(register::FLAG) >> index & 1 != 0
    }
    fn stack_ptr(&mut self) -> u8 { self.reg8(register::SCTR) }
    fn set_stack_ptr(&mut self, value: u8) { self.set_reg8(register::SCTR, value) }
    fn inc_stack_ptr(&mut self) { self.set_stack_ptr(self.stack_ptr()+1) }
    fn dec_stack_ptr(&mut self) { self.set_stack_ptr(self.stack_ptr()-1) }
}

// current operation related utils
impl Computer {
    fn pc(&self) -> u16 {
        self.reg16(register::PC_H)
    }
    fn pc_inc(&self) { self.set_reg16(register::PC_H, self.pc()+1)}
    fn op_lit8(&self) -> u8 {
        self.ram8(self.pc())
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
    fn op_value8(&mut self) -> u8 {
        if self.op_flag() {
            self.pc_inc();
            self.op_lit8()
        } else {
            self.pc_inc();
            self.reg8(self.op_reg())
        }
    }
    fn op_value16(&mut self) -> u16 {
        if self.op_flag() {
            self.reg16(register::HIGH)
        } else {
            self.pc_inc();
            let ret = self.op_lit16();
            self.pc_inc();
            ret
        }
    }
}

// Execution Manager
impl Computer {
    fn run(&mut self) {
        while !self.flag(flag::HALT) {
            self.step()
        }
    }
    fn step(&mut self) {
        match self.opc() {
            0x0 => self.run_nop(),
            0x1 => self.run_mov(),
            0x2 => self.run_ldw(),
            0x3 => self.run_stw(),
            0x4 => self.run_lda(),
            0x5 => self.run_psh(),
            0x6 => self.run_pop(),
            0x7 => self.run_jmp(),
            0x8 => self.run_add(),
            0x9 => self.run_sub(),
            0xA => self.run_and(),
            0xB => self.run_or(),
            0xC => self.run_inv(),
            0xD => self.run_cmp(),
            0xE => self.run_shl(),
            0xF => self.run_shr(),
            _ => unreachable!()
        }
    }
}

// OP Implementations
impl Computer {
    fn run_nop(self) { }
    fn run_mov(&mut self) {
        self.set_reg8(
            self.op_reg(),
            self.op_value8()
        );
        self.pc_inc()
    }
    fn run_ldw(&mut self) {
        self.set_reg8(
            self.op_reg(),
            self.ram8(self.op_value16())
        );
        self.pc_inc();
    }
    fn run_stw(&mut self) {
        let value = self.op_reg();
        let dest = self.op_value16();
        self.set_ram8(
            dest,
            value
        );
        self.pc_inc();
    }
    fn run_lda(&mut self) {
        self.set_reg16(register::HIGH, self.ram16(self.op_value16()));
        self.pc_inc();
    }
    fn run_psh(&mut self) {
        self.stack[self.stack_ptr()] = self.reg8(self.op_reg());
        self.inc_stack_ptr();
        self.pc_inc();
    }
    fn run_pop(&mut self) {
        self.set_reg8(self.op_reg(), self.stack[self.stack_ptr()]);
        self.dec_stack_ptr();
        self.pc_inc();
    }
    fn run_jmp(&mut self) {
        if self.flag(self.op_reg()) {
            self.set_reg16(register::PC_H, self.op_value16())
        }
        self.pc_inc();
    }
    fn run_add(&mut self) {
        let result_reg = self.op_reg();
        let value = self.op_value8();
        let result = (self.reg8(result_reg) as i8).overflowing_add(value as i8) as (u8, bool);
        self.set_reg8(result_reg, result.0);
        self.set_flag(flag::CARRY, result.1);
        self.pc_inc();
    }
    fn run_sub(&mut self) {
        let result_reg = self.op_reg();
        let value = self.op_value8();
        let result = (self.reg8(result_reg) as i8).overflowing_sub(value as i8) as (u8, bool);
        self.set_reg8(result_reg, result.0);
        self.set_flag(flag::BORROW, result.1);
        self.pc_inc();
    }
    fn run_and(&mut self) {
        let result_reg = self.op_reg();
        let value = self.op_value8();
        self.set_reg8(result_reg, self.reg8(result_reg)&value);
        self.pc_inc();
    }
    fn run_or(&mut self) {
        let result_reg = self.op_reg();
        let value = self.op_value8();
        self.set_reg8(result_reg, self.reg8(result_reg)|value);
        self.pc_inc();
    }
    fn run_inv(&mut self) {
        let result_reg = self.op_reg();
        self.set_reg8(result_reg, !self.reg8(result_reg));
        self.pc_inc();
    }
    fn run_cmp(&mut self) {
        let a = self.op_reg();
        let b = self.op_value8();
        self.set_flag(flag::LESS, a < b);
        self.set_flag(flag::EQUAL, a == b);
        self.pc_inc();
    }
    fn run_shl(&mut self) {
        let result_reg = self.op_reg();
        let value = self.op_value8();
        self.set_reg8(result_reg, self.reg8(result_reg) << value);
        self.pc_inc();
    }
    fn run_shr(&mut self) {
        let result_reg = self.op_reg();
        let value = self.op_value8();
        self.set_reg8(result_reg, self.reg8(result_reg) << value);
        self.pc_inc();
    }
}
