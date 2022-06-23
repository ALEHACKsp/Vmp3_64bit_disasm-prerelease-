use iced_x86::{Code, Instruction, OpKind, Register};

use crate::vm_handler::VmRegisterAllocation;

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
