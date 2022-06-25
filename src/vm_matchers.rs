use iced_x86::{Code, Register};

use crate::{
    match_assembly::{
        match_add_reg_reg, match_add_vsp_by_amount, match_add_vsp_get_amount, match_and_reg_reg,
        match_fetch_reg_any_size, match_fetch_zx_reg_any_size, match_mov_reg_source, match_not_reg,
        match_or_reg_reg, match_popfq, match_pushfq, match_ret, match_shr_reg_reg,
        match_store_reg2_in_reg1, match_store_reg_any_size, match_sub_vsp_by_amount,
        match_sub_vsp_get_amount,
    },
    util::check_full_reg_written,
    vm_handler::{Registers, VmHandler, VmRegisterAllocation},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandlerClass {
    ByteOperand,
    WordOperand,
    DwordOperand,
    QwordOperand,
    NoOperand,
    UnconditionalBranch,
    NoVipChange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandlerVmInstruction {
    /// Size in bytes and reg offset in register file
    Pop(usize, u8),
    Push(usize, u8),
    PushImm64(u64),
    PushImm32(u32),
    PushImm16(u16),
    PushVsp(usize),
    PopVsp(usize),
    Add(usize),
    Shr(usize),
    Nand(usize),
    Nor(usize),
    Fetch(usize),
    Store(usize),
    VmExit,
    UnknownByteOperand,
    UnknownWordOperand,
    UnknownDwordOperand,
    UnknownQwordOperand,
    UnknownNoOperand,
    UnknownNoVipChange,
    Unknown,
}

impl VmHandler {
    pub fn match_handler_class(&self,
                               reg_allocation: &VmRegisterAllocation)
                               -> HandlerClass {
        let instruction_iter = self.instructions.iter();

        let vip_modification_vec =
            instruction_iter.clone()
                            .filter(|insn| check_full_reg_written(insn, reg_allocation.vip.into()))
                            .filter(|insn| insn.code() == Code::Mov_r64_rm64)
                            .collect::<Vec<_>>();

        if (reg_allocation.vip != Registers::Rsi &&
            reg_allocation.vip != Registers::Rdi &&
            !vip_modification_vec.is_empty()) ||
           (vip_modification_vec.len() >= 2)
        {
            return HandlerClass::UnconditionalBranch;
        }

        let vip_update_vec =
            instruction_iter.filter(|insn| check_full_reg_written(insn, reg_allocation.vip.into()))
                            .filter(|insn| {
                                insn.code() == Code::Add_rm64_imm32 ||
                                insn.code() == Code::Sub_rm64_imm32
                            })
                            .map(|insn| insn.immediate32())
                            .collect::<Vec<_>>();

        match vip_update_vec.as_slice() {
            &[] => HandlerClass::NoVipChange,
            &[4] => HandlerClass::NoOperand,
            &[8, 4] => HandlerClass::QwordOperand,
            &[4, 4] => HandlerClass::DwordOperand,
            &[2, 4] => HandlerClass::WordOperand,
            &[1, 4] => HandlerClass::ByteOperand,
            slice => {
                panic!("Unimplemented handler class with slice {:?}", slice)
            },
        }
    }

    pub fn match_no_vip_change_instructions(&self,
                                            reg_allocation: &VmRegisterAllocation)
                                            -> HandlerVmInstruction {
        if vm_match_vm_exit(self, reg_allocation) {
            return HandlerVmInstruction::VmExit;
        }

        HandlerVmInstruction::UnknownNoVipChange
    }

    pub fn match_byte_operand_instructions(&self,
                                           reg_allocation: &VmRegisterAllocation,
                                           byte_operand: u8)
                                           -> HandlerVmInstruction {
        if let Some(size) = vm_match_vm_reg_pop(self, reg_allocation) {
            return HandlerVmInstruction::Pop(size, byte_operand);
        }

        if let Some(size) = vm_match_vm_reg_push(self, reg_allocation) {
            return HandlerVmInstruction::Push(size, byte_operand);
        }

        HandlerVmInstruction::UnknownByteOperand
    }

    pub fn match_word_operand_instructions(&self,
                                           reg_allocation: &VmRegisterAllocation,
                                           word_operand: u16)
                                           -> HandlerVmInstruction {
        if vm_match_push_imm16(self, reg_allocation) {
            return HandlerVmInstruction::PushImm16(word_operand);
        }
        HandlerVmInstruction::UnknownWordOperand
    }

    pub fn match_dword_operand_instructions(&self,
                                            reg_allocation: &VmRegisterAllocation,
                                            dword_operand: u32)
                                            -> HandlerVmInstruction {
        if vm_match_push_imm32(self, reg_allocation) {
            return HandlerVmInstruction::PushImm32(dword_operand);
        }
        HandlerVmInstruction::UnknownDwordOperand
    }

    pub fn match_qword_operand_instructions(&self,
                                            reg_allocation: &VmRegisterAllocation,
                                            qword_operand: u64)
                                            -> HandlerVmInstruction {
        if vm_match_push_imm64(self, reg_allocation) {
            return HandlerVmInstruction::PushImm64(qword_operand);
        }
        HandlerVmInstruction::UnknownQwordOperand
    }

    pub fn match_no_operand_instructions(&self,
                                         reg_allocation: &VmRegisterAllocation)
                                         -> HandlerVmInstruction {
        if let Some(size) = vm_match_add(self, reg_allocation) {
            return HandlerVmInstruction::Add(size);
        }

        if let Some(size) = vm_match_add_byte(self, reg_allocation) {
            return HandlerVmInstruction::Add(size);
        }

        if let Some(size) = vm_match_shr(self, reg_allocation) {
            return HandlerVmInstruction::Shr(size);
        }

        if let Some(size) = vm_match_shr_byte(self, reg_allocation) {
            return HandlerVmInstruction::Shr(size);
        }

        if let Some(size) = vm_match_nand(self, reg_allocation) {
            return HandlerVmInstruction::Nand(size);
        }

        if let Some(size) = vm_match_nand_byte(self, reg_allocation) {
            return HandlerVmInstruction::Nand(size);
        }

        if let Some(size) = vm_match_nor(self, reg_allocation) {
            return HandlerVmInstruction::Nor(size);
        }

        if let Some(size) = vm_match_nor_byte(self, reg_allocation) {
            return HandlerVmInstruction::Nor(size);
        }

        if let Some(size) = vm_match_push_vsp(self, reg_allocation) {
            return HandlerVmInstruction::PushVsp(size);
        }

        if vm_match_pop_vsp_64(self, reg_allocation) {
            return HandlerVmInstruction::PopVsp(8);
        }

        if let Some(size) = vm_match_fetch(self, reg_allocation) {
            return HandlerVmInstruction::Fetch(size);
        }

        if let Some(size) = vm_match_fetch_byte(self, reg_allocation) {
            return HandlerVmInstruction::Fetch(size);
        }

        if let Some(size) = vm_match_store(self, reg_allocation) {
            return HandlerVmInstruction::Store(size);
        }
        HandlerVmInstruction::UnknownNoOperand
    }
}

fn vm_match_vm_reg_pop(vm_handler: &VmHandler,
                       reg_allocation: &VmRegisterAllocation)
                       -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();
    instruction_iter.find(|insn| {
                        match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                    });
    let add_vsp_instruction =
        instruction_iter.find(|insn| match_add_vsp_get_amount(insn, reg_allocation).is_some());
    add_vsp_instruction.map(|insn| insn.immediate32() as usize)
}

fn vm_match_vm_reg_push(vm_handler: &VmHandler,
                        reg_allocation: &VmRegisterAllocation)
                        -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();

    let sub_vsp_instruction =
        instruction_iter.find(|insn| match_sub_vsp_get_amount(insn, reg_allocation).is_some());

    instruction_iter.find(|insn| {
                        match_store_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                    });
    if instruction_iter.len() == 0 {
        return None;
    }

    sub_vsp_instruction.map(|insn| insn.immediate32() as usize)
}

fn vm_match_push_imm64(vm_handler: &VmHandler,
                       reg_allocation: &VmRegisterAllocation)
                       -> bool {
    let mut instruction_iter = vm_handler.instructions.iter();
    instruction_iter.find(|insn| match_sub_vsp_by_amount(insn, reg_allocation, 8));
    instruction_iter.any(|insn| match_store_reg_any_size(insn, reg_allocation.vsp.into()).is_some())
}

fn vm_match_push_imm32(vm_handler: &VmHandler,
                       reg_allocation: &VmRegisterAllocation)
                       -> bool {
    let mut instruction_iter = vm_handler.instructions.iter();
    instruction_iter.find(|insn| match_sub_vsp_by_amount(insn, reg_allocation, 4));
    instruction_iter.any(|insn| match_store_reg_any_size(insn, reg_allocation.vsp.into()).is_some())
}

fn vm_match_push_imm16(vm_handler: &VmHandler,
                       reg_allocation: &VmRegisterAllocation)
                       -> bool {
    let mut instruction_iter = vm_handler.instructions.iter();
    instruction_iter.find(|insn| match_sub_vsp_by_amount(insn, reg_allocation, 2));
    instruction_iter.any(|insn| match_store_reg_any_size(insn, reg_allocation.vsp.into()).is_some())
}

fn vm_match_pop_vsp_64(vm_handler: &VmHandler,
                       reg_allocation: &VmRegisterAllocation)
                       -> bool {
    let mut instruction_iter = vm_handler.instructions.iter();

    let fetch_vsp_instruction_1 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        });
    if fetch_vsp_instruction_1.is_none() {
        return false;
    }

    fetch_vsp_instruction_1.unwrap().op0_register() == reg_allocation.vsp.into()
}

fn vm_match_add(vm_handler: &VmHandler,
                reg_allocation: &VmRegisterAllocation)
                -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();

    let fetch_vsp_instruction_1 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg1 = fetch_vsp_instruction_1.op0_register().full_register();

    let fetch_vsp_instruction_2 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg2 = fetch_vsp_instruction_2.op0_register().full_register();

    let instruction_size = fetch_vsp_instruction_1.memory_size().size();

    instruction_iter.find(|insn| match_add_reg_reg(insn, reg1, reg2))?;

    instruction_iter.find(|insn| match_pushfq(insn))?;

    Some(instruction_size)
}

fn vm_match_add_byte(vm_handler: &VmHandler,
                     reg_allocation: &VmRegisterAllocation)
                     -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();

    let fetch_vsp_instruction_1 =
        instruction_iter.find(|insn| {
                            match_fetch_zx_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg1 = fetch_vsp_instruction_1.op0_register().full_register();

    let fetch_vsp_instruction_2 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg2 = fetch_vsp_instruction_2.op0_register().full_register();

    let instruction_size = fetch_vsp_instruction_1.memory_size().size();

    instruction_iter.find(|insn| match_add_reg_reg(insn, reg1, reg2))?;

    instruction_iter.find(|insn| match_pushfq(insn))?;

    Some(instruction_size)
}

fn vm_match_shr(vm_handler: &VmHandler,
                reg_allocation: &VmRegisterAllocation)
                -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();

    let fetch_vsp_instruction_1 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg = fetch_vsp_instruction_1.op0_register();

    let _fetch_vsp_instruction_2 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;

    let instruction_size = fetch_vsp_instruction_1.memory_size().size();

    instruction_iter.find(|insn| match_shr_reg_reg(insn, reg))?;

    instruction_iter.find(|insn| match_pushfq(insn))?;

    Some(instruction_size)
}

fn vm_match_shr_byte(vm_handler: &VmHandler,
                     reg_allocation: &VmRegisterAllocation)
                     -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();

    let fetch_vsp_instruction_1 =
        instruction_iter.find(|insn| {
                            match_fetch_zx_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg = fetch_vsp_instruction_1.op0_register().full_register();

    let _fetch_vsp_instruction_2 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;

    let instruction_size = fetch_vsp_instruction_1.memory_size().size();

    instruction_iter.find(|insn| match_shr_reg_reg(insn, reg))?;

    instruction_iter.find(|insn| match_pushfq(insn))?;

    Some(instruction_size)
}
fn vm_match_nand(vm_handler: &VmHandler,
                 reg_allocation: &VmRegisterAllocation)
                 -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();

    let fetch_vsp_instruction_1 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg1 = fetch_vsp_instruction_1.op0_register().full_register();

    let fetch_vsp_instruction_2 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg2 = fetch_vsp_instruction_2.op0_register().full_register();

    let instruction_size = fetch_vsp_instruction_1.memory_size().size();

    instruction_iter.find(|insn| match_not_reg(insn, reg1))?;
    instruction_iter.find(|insn| match_not_reg(insn, reg2))?;

    instruction_iter.find(|insn| match_or_reg_reg(insn, reg1, reg2))?;

    instruction_iter.find(|insn| match_pushfq(insn))?;

    Some(instruction_size)
}

fn vm_match_nand_byte(vm_handler: &VmHandler,
                      reg_allocation: &VmRegisterAllocation)
                      -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();

    let fetch_vsp_instruction_1 =
        instruction_iter.find(|insn| {
                            match_fetch_zx_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg1 = fetch_vsp_instruction_1.op0_register().full_register();

    let fetch_vsp_instruction_2 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg2 = fetch_vsp_instruction_2.op0_register().full_register();

    let instruction_size = fetch_vsp_instruction_1.memory_size().size();

    instruction_iter.find(|insn| match_not_reg(insn, reg1))?;
    instruction_iter.find(|insn| match_not_reg(insn, reg2))?;

    instruction_iter.find(|insn| match_or_reg_reg(insn, reg1, reg2))?;

    instruction_iter.find(|insn| match_pushfq(insn))?;

    Some(instruction_size)
}

fn vm_match_nor(vm_handler: &VmHandler,
                reg_allocation: &VmRegisterAllocation)
                -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();

    let fetch_vsp_instruction_1 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg1 = fetch_vsp_instruction_1.op0_register().full_register();

    let fetch_vsp_instruction_2 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg2 = fetch_vsp_instruction_2.op0_register().full_register();

    let instruction_size = fetch_vsp_instruction_1.memory_size().size();

    instruction_iter.find(|insn| match_not_reg(insn, reg1))?;
    instruction_iter.find(|insn| match_not_reg(insn, reg2))?;

    instruction_iter.find(|insn| match_and_reg_reg(insn, reg1, reg2))?;

    instruction_iter.find(|insn| match_pushfq(insn))?;

    Some(instruction_size)
}

fn vm_match_nor_byte(vm_handler: &VmHandler,
                     reg_allocation: &VmRegisterAllocation)
                     -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();

    let fetch_vsp_instruction_1 =
        instruction_iter.find(|insn| {
                            match_fetch_zx_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg1 = fetch_vsp_instruction_1.op0_register().full_register();

    let fetch_vsp_instruction_2 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg2 = fetch_vsp_instruction_2.op0_register().full_register();

    let instruction_size = fetch_vsp_instruction_1.memory_size().size();

    instruction_iter.find(|insn| match_not_reg(insn, reg1))?;
    instruction_iter.find(|insn| match_not_reg(insn, reg2))?;

    instruction_iter.find(|insn| match_and_reg_reg(insn, reg1, reg2))?;

    instruction_iter.find(|insn| match_pushfq(insn))?;

    Some(instruction_size)
}
fn vm_match_store(vm_handler: &VmHandler,
                  reg_allocation: &VmRegisterAllocation)
                  -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();

    let fetch_vsp_instruction_1 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg1 = fetch_vsp_instruction_1.op0_register();

    let fetch_vsp_instruction_2 =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;
    let reg2 = fetch_vsp_instruction_2.op0_register();

    instruction_iter.find(|insn| match_add_vsp_by_amount(insn, reg_allocation, 0x10))?;

    let mut instruction_size = 0;
    instruction_iter.find(|insn| match match_store_reg2_in_reg1(insn, reg1, reg2) {
                        Some(size) => {
                            instruction_size = size;
                            true
                        },
                        _ => false,
                    })?;
    Some(instruction_size)
}

fn vm_match_fetch(vm_handler: &VmHandler,
                  reg_allocation: &VmRegisterAllocation)
                  -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();
    let fetch_vsp_instruction =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;

    let fetch_register = fetch_vsp_instruction.op0_register();

    let mut fetch_size = 0;
    instruction_iter.find(|insn| {
                        if let Some(size) = match_fetch_reg_any_size(insn, fetch_register) {
                            fetch_size = size;
                            true
                        } else {
                            false
                        }
                    })?;

    Some(fetch_size)
}

fn vm_match_fetch_byte(vm_handler: &VmHandler,
                       reg_allocation: &VmRegisterAllocation)
                       -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();
    let fetch_vsp_instruction =
        instruction_iter.find(|insn| {
                            match_fetch_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                        })?;

    let fetch_register = fetch_vsp_instruction.op0_register().full_register();

    let mut fetch_size = 0;
    instruction_iter.find(|insn| {
                        if let Some(size) = match_fetch_zx_reg_any_size(insn, fetch_register) {
                            fetch_size = size;
                            true
                        } else {
                            false
                        }
                    })?;

    Some(fetch_size)
}
fn vm_match_push_vsp(vm_handler: &VmHandler,
                     reg_allocation: &VmRegisterAllocation)
                     -> Option<usize> {
    let mut instruction_iter = vm_handler.instructions.iter();
    instruction_iter.find(|insn| match_mov_reg_source(insn, reg_allocation.vsp.into()))?;

    let sub_vsp_instruction =
        instruction_iter.find(|insn| match_sub_vsp_get_amount(insn, reg_allocation).is_some())?;
    let instruction_size = sub_vsp_instruction.immediate32();
    instruction_iter.find(|insn| {
                        match_store_reg_any_size(insn, reg_allocation.vsp.into()).is_some()
                    })?;

    Some(instruction_size as usize)
}

fn vm_match_vm_exit(vm_handler: &VmHandler,
                    reg_allocation: &VmRegisterAllocation)
                    -> bool {
    let instruction_iter = vm_handler.instructions.iter();

    if !instruction_iter.clone().any(match_ret) {
        return false;
    }

    if !instruction_iter.clone().any(match_popfq) {
        return false;
    }

    if !instruction_iter.clone().any(|insn| {
                                    insn.code() == Code::Mov_r64_rm64 &&
                                    insn.op0_register() == Register::RSP &&
                                    insn.op1_register() == reg_allocation.vsp.into()
                                })
    {
        return false;
    }

    if instruction_iter.filter(|insn| insn.code() == Code::Pop_r64)
                       .count() !=
       15
    {
        return false;
    }

    true
}
