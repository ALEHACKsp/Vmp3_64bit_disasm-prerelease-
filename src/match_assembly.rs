use iced_x86::{Code, Instruction, OpKind, Register};

use crate::vm_handler::VmRegisterAllocation;

pub fn match_pushfq(instruction: &Instruction) -> bool {
    if instruction.code() != Code::Pushfq {
        return false;
    }
    true
}

pub fn match_popfq(instruction: &Instruction) -> bool {
    if instruction.code() != Code::Popfq {
        return false;
    }
    true
}

pub fn match_ret(instruction: &Instruction) -> bool {
    if instruction.code() != Code::Retnq {
        return false;
    }
    true
}

pub fn match_not_reg(instruction: &Instruction,
                     register: Register)
                     -> bool {
    match instruction.code() {
        Code::Not_rm8 | Code::Not_rm16 | Code::Not_rm32 | Code::Not_rm64
            if instruction.op0_register().full_register() == register =>
        {
            return true;
        },
        _ => return false,
    }
}

pub fn match_mov_reg_source(instruction: &Instruction,
                            register: Register)
                            -> bool {
    match instruction.code() {
        Code::Mov_r64_rm64 |
        Code::Mov_r32_rm32 |
        Code::Mov_r16_rm16 |
        Code::Mov_r8_rm8 |
        Code::Mov_rm64_r64 |
        Code::Mov_rm32_r32 |
        Code::Mov_rm16_r16 |
        Code::Mov_rm8_r8 => {},
        _ => return false,
    }

    if instruction.op0_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_register() != register {
        return false;
    }

    true
}

/// Return size of the store in bytes
pub fn match_store_reg2_in_reg1(instruction: &Instruction,
                                reg1: Register,
                                reg2: Register)
                                -> Option<usize> {
    let instruction_size;
    match instruction.code() {
        Code::Mov_rm8_r8 => {
            instruction_size = 1;
        },
        Code::Mov_rm16_r16 => {
            instruction_size = 2;
        },
        Code::Mov_rm32_r32 => {
            instruction_size = 4;
        },
        Code::Mov_rm64_r64 => {
            instruction_size = 8;
        },
        _ => return None,
    }

    if instruction.op0_kind() != OpKind::Memory {
        return None;
    }

    if instruction.op1_kind() != OpKind::Register {
        return None;
    }

    if instruction.memory_base() != reg1 {
        return None;
    }

    if instruction.op1_register() != reg2 {
        return None;
    }

    Some(instruction_size)
}

pub fn match_shr_reg_reg(instruction: &Instruction,
                         reg: Register)
                         -> bool {
    match instruction.code() {
        Code::Shr_rm8_CL | Code::Shr_rm16_CL | Code::Shr_rm32_CL | Code::Shr_rm64_CL
            if (instruction.op0_register().full_register() == reg) =>
        {
            true
        },

        _ => false,
    }
}

pub fn match_or_reg_reg(instruction: &Instruction,
                        reg1: Register,
                        reg2: Register)
                        -> bool {
    match instruction.code() {
        Code::Or_rm8_r8 |
        Code::Or_rm16_r16 |
        Code::Or_rm32_r32 |
        Code::Or_rm64_r64 |
        Code::Or_r8_rm8 |
        Code::Or_r16_rm16 |
        Code::Or_r32_rm32 |
        Code::Or_r64_rm64
            if (instruction.op0_register().full_register() == reg1 &&
                instruction.op1_register().full_register() == reg2) ||
               (instruction.op0_register().full_register() == reg2 &&
                instruction.op1_register().full_register() == reg1) =>
        {
            true
        },

        _ => false,
    }
}

pub fn match_and_reg_reg(instruction: &Instruction,
                         reg1: Register,
                         reg2: Register)
                         -> bool {
    match instruction.code() {
        Code::And_rm8_r8 |
        Code::And_rm16_r16 |
        Code::And_rm32_r32 |
        Code::And_rm64_r64 |
        Code::And_r8_rm8 |
        Code::And_r16_rm16 |
        Code::And_r32_rm32 |
        Code::And_r64_rm64
            if (instruction.op0_register().full_register() == reg1 &&
                instruction.op1_register().full_register() == reg2) ||
               (instruction.op0_register().full_register() == reg2 &&
                instruction.op1_register().full_register() == reg1) =>
        {
            true
        },

        _ => false,
    }
}

pub fn match_add_reg_reg(instruction: &Instruction,
                         reg1: Register,
                         reg2: Register)
                         -> bool {
    match instruction.code() {
        Code::Add_rm8_r8 |
        Code::Add_rm16_r16 |
        Code::Add_rm32_r32 |
        Code::Add_rm64_r64 |
        Code::Add_r8_rm8 |
        Code::Add_r16_rm16 |
        Code::Add_r32_rm32 |
        Code::Add_r64_rm64
            if (instruction.op0_register().full_register() == reg1 &&
                instruction.op1_register().full_register() == reg2) ||
               (instruction.op0_register().full_register() == reg2 &&
                instruction.op1_register().full_register() == reg1) =>
        {
            true
        },

        _ => false,
    }
}

/// Returns the size of the match in bytes if there is one
pub fn match_fetch_reg_any_size(instruction: &Instruction,
                                register: Register)
                                -> Option<usize> {
    let mov_size = match instruction.code() {
        Code::Mov_r64_rm64 if instruction.op1_kind() == OpKind::Memory => Some(8),
        Code::Mov_r32_rm32 if instruction.op1_kind() == OpKind::Memory => Some(4),
        Code::Mov_r16_rm16 if instruction.op1_kind() == OpKind::Memory => Some(2),
        Code::Mov_r8_rm8 if instruction.op1_kind() == OpKind::Memory => Some(1),
        _ => return None,
    };

    if mov_size.is_some() {
        if instruction.memory_base().full_register() == register {
            return mov_size;
        } else {
            return None;
        }
    } else {
        return None;
    }
}

pub fn match_fetch_zx_reg_any_size(instruction: &Instruction,
                                   register: Register)
                                   -> Option<usize> {
    let mov_size = match instruction.code() {
        Code::Movzx_r64_rm8 if instruction.op1_kind() == OpKind::Memory => Some(1),
        Code::Movzx_r64_rm16 if instruction.op1_kind() == OpKind::Memory => Some(2),
        Code::Movzx_r32_rm8 if instruction.op1_kind() == OpKind::Memory => Some(1),
        Code::Movzx_r32_rm16 if instruction.op1_kind() == OpKind::Memory => Some(2),
        Code::Movzx_r16_rm8 if instruction.op1_kind() == OpKind::Memory => Some(1),
        Code::Movzx_r16_rm16 if instruction.op1_kind() == OpKind::Memory => Some(2),
        _ => return None,
    };

    if mov_size.is_some() {
        if instruction.memory_base().full_register() == register {
            return mov_size;
        } else {
            return None;
        }
    } else {
        return None;
    }
}
pub fn match_store_reg_any_size(instruction: &Instruction,
                                register: Register)
                                -> Option<usize> {
    let mov_size = match instruction.code() {
        Code::Mov_rm64_r64 if instruction.op0_kind() == OpKind::Memory => Some(8),
        Code::Mov_rm32_r32 if instruction.op0_kind() == OpKind::Memory => Some(4),
        Code::Mov_rm16_r16 if instruction.op0_kind() == OpKind::Memory => Some(2),
        Code::Mov_rm8_r8 if instruction.op0_kind() == OpKind::Memory => Some(1),
        _ => return None,
    };

    if mov_size.is_some() {
        if instruction.memory_base().full_register() == register {
            return mov_size;
        } else {
            return None;
        }
    } else {
        return None;
    }
}

pub fn match_fetch_encrypted_vip(instruction: &Instruction,
                                 vm_register_allocation: &VmRegisterAllocation)
                                 -> bool {
    // Check the instruction opcode
    if instruction.code() != Code::Mov_r64_rm64 {
        return false;
    }

    // Check that the second operand is a memory type operand
    if instruction.op1_kind() != OpKind::Memory {
        return false;
    }

    // Check that the displacement is 90
    if instruction.memory_displacement64() != 0x90 {
        return false;
    }

    // Check that the index register is rsp
    if instruction.memory_base() != Register::RSP {
        return false;
    }

    // Check that the write is to vip
    if instruction.op0_register() != vm_register_allocation.vip.into() {
        return false;
    }

    true
}

pub fn match_fetch_vip(instruction: &Instruction,
                       vm_register_allocation: &VmRegisterAllocation)
                       -> bool {
    if instruction.code() != Code::Mov_r32_rm32 {
        return false;
    }

    if instruction.op1_kind() != OpKind::Memory {
        return false;
    }

    if instruction.memory_base() != vm_register_allocation.vip.into() {
        return false;
    }

    true
}

pub fn match_push_rolling_key(instruction: &Instruction,
                              vm_register_allocation: &VmRegisterAllocation)
                              -> bool {
    if instruction.code() != Code::Push_r64 {
        return false;
    }

    if instruction.op0_kind() != OpKind::Register {
        return false;
    }

    if instruction.op0_register().full_register() != vm_register_allocation.key.into() {
        return false;
    }

    true
}

pub fn match_add_vsp_by_amount(instruction: &Instruction,
                               vm_register_allocation: &VmRegisterAllocation,
                               amount: u32)
                               -> bool {
    if instruction.code() != Code::Add_rm64_imm32 {
        return false;
    }

    if instruction.immediate32() != amount {
        return false;
    }

    if instruction.op0_kind() != OpKind::Register {
        return false;
    }

    if instruction.op0_register().full_register() != vm_register_allocation.vsp.into() {
        return false;
    }

    true
}

pub fn match_sub_vsp_by_amount(instruction: &Instruction,
                               vm_register_allocation: &VmRegisterAllocation,
                               amount: u32)
                               -> bool {
    if instruction.code() != Code::Sub_rm64_imm32 {
        return false;
    }

    if instruction.immediate32() != amount {
        return false;
    }

    if instruction.op0_kind() != OpKind::Register {
        return false;
    }

    if instruction.op0_register().full_register() != vm_register_allocation.vsp.into() {
        return false;
    }

    true
}

pub fn match_sub_vsp_get_amount(instruction: &Instruction,
                                vm_register_allocation: &VmRegisterAllocation)
                                -> Option<u32> {
    if instruction.code() != Code::Sub_rm64_imm32 {
        return None;
    }

    if instruction.op0_kind() != OpKind::Register {
        return None;
    }

    if instruction.op0_register().full_register() != vm_register_allocation.vsp.into() {
        return None;
    }

    Some(instruction.immediate32())
}

pub fn match_add_vsp_get_amount(instruction: &Instruction,
                                vm_register_allocation: &VmRegisterAllocation)
                                -> Option<u32> {
    if instruction.code() != Code::Add_rm64_imm32 {
        return None;
    }

    if instruction.op0_kind() != OpKind::Register {
        return None;
    }

    if instruction.op0_register().full_register() != vm_register_allocation.vsp.into() {
        return None;
    }

    Some(instruction.immediate32())
}

pub fn match_xor_64_rolling_key_source(instruction: &Instruction,
                                       vm_register_allocation: &VmRegisterAllocation)
                                       -> bool {
    if instruction.code() != Code::Xor_r64_rm64 {
        return false;
    }

    if instruction.op0_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_register().full_register() != vm_register_allocation.key.into() {
        return false;
    }

    true
}

pub fn match_xor_64_rolling_key_dest(instruction: &Instruction,
                                     vm_register_allocation: &VmRegisterAllocation)
                                     -> bool {
    if instruction.code() != Code::Xor_r64_rm64 {
        return false;
    }

    if instruction.op0_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_kind() != OpKind::Register {
        return false;
    }

    if instruction.op0_register().full_register() != vm_register_allocation.key.into() {
        return false;
    }

    true
}

pub fn match_xor_16_rolling_key_source(instruction: &Instruction,
                                       vm_register_allocation: &VmRegisterAllocation)
                                       -> bool {
    if instruction.code() != Code::Xor_r16_rm16 {
        return false;
    }

    if instruction.op0_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_register().full_register() != vm_register_allocation.key.into() {
        return false;
    }

    true
}

pub fn match_xor_16_rolling_key_dest(instruction: &Instruction,
                                     vm_register_allocation: &VmRegisterAllocation)
                                     -> bool {
    if instruction.code() != Code::Xor_r16_rm16 {
        return false;
    }

    if instruction.op0_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_kind() != OpKind::Register {
        return false;
    }

    if instruction.op0_register().full_register() != vm_register_allocation.key.into() {
        return false;
    }

    true
}

pub fn match_xor_32_rolling_key_source(instruction: &Instruction,
                                       vm_register_allocation: &VmRegisterAllocation)
                                       -> bool {
    if instruction.code() != Code::Xor_r32_rm32 {
        return false;
    }

    if instruction.op0_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_register().full_register() != vm_register_allocation.key.into() {
        return false;
    }

    true
}
pub fn match_xor_8_rolling_key_source(instruction: &Instruction,
                                      vm_register_allocation: &VmRegisterAllocation)
                                      -> bool {
    if instruction.code() != Code::Xor_r8_rm8 {
        return false;
    }

    if instruction.op0_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_register().full_register() != vm_register_allocation.key.into() {
        return false;
    }

    true
}

pub fn match_xor_8_rolling_key_dest(instruction: &Instruction,
                                    vm_register_allocation: &VmRegisterAllocation)
                                    -> bool {
    if instruction.code() != Code::Xor_r8_rm8 {
        return false;
    }

    if instruction.op0_kind() != OpKind::Register {
        return false;
    }

    if instruction.op1_kind() != OpKind::Register {
        return false;
    }

    if instruction.op0_register().full_register() != vm_register_allocation.key.into() {
        return false;
    }

    true
}
