use iced_x86::{Code, Instruction, Register};

use crate::util::check_full_reg_written;

pub fn get_transform_for_instruction(instruction: &Instruction) -> Option<Transform> {
    // Add the transform that represents this instruction to the transforms vec
    match instruction.code() {
        Code::Bswap_r16 => Some(Transform::ByteSwap16),
        Code::Bswap_r32 => Some(Transform::ByteSwap32),
        Code::Bswap_r64 => Some(Transform::ByteSwap64),

        Code::Sub_AL_imm8 => Some(Transform::SubtractConstant8(instruction.immediate8())),
        Code::Sub_rm8_imm8 => Some(Transform::SubtractConstant8(instruction.immediate8())),
        Code::Sub_AX_imm16 => Some(Transform::SubtractConstant16(instruction.immediate16())),
        Code::Sub_rm16_imm16 => Some(Transform::SubtractConstant16(instruction.immediate16())),
        Code::Sub_EAX_imm32 => Some(Transform::SubtractConstant32(instruction.immediate32())),
        Code::Sub_rm32_imm32 => Some(Transform::SubtractConstant32(instruction.immediate32())),
        Code::Sub_RAX_imm32 => Some(Transform::SubtractConstant64(instruction.immediate64())),
        Code::Sub_rm64_imm32 => Some(Transform::SubtractConstant64(instruction.immediate64())),

        Code::Add_AL_imm8 => Some(Transform::AddConstant8(instruction.immediate8())),
        Code::Add_rm8_imm8 => Some(Transform::AddConstant8(instruction.immediate8())),
        Code::Add_AX_imm16 => Some(Transform::AddConstant16(instruction.immediate16())),
        Code::Add_rm16_imm16 => Some(Transform::AddConstant16(instruction.immediate16())),
        Code::Add_EAX_imm32 => Some(Transform::AddConstant32(instruction.immediate32())),
        Code::Add_rm32_imm32 => Some(Transform::AddConstant32(instruction.immediate32())),
        Code::Add_RAX_imm32 => Some(Transform::AddConstant64(instruction.immediate64())),
        Code::Add_rm64_imm32 => Some(Transform::AddConstant64(instruction.immediate64())),

        Code::Neg_rm8 => Some(Transform::Negate8),
        Code::Neg_rm16 => Some(Transform::Negate16),
        Code::Neg_rm32 => Some(Transform::Negate32),
        Code::Neg_rm64 => Some(Transform::Negate64),

        Code::Not_rm8 => Some(Transform::Not8),
        Code::Not_rm16 => Some(Transform::Not16),
        Code::Not_rm32 => Some(Transform::Not32),
        Code::Not_rm64 => Some(Transform::Not64),

        Code::Rol_rm8_imm8 => Some(Transform::RotateLeft8(instruction.immediate8() as u32)),
        Code::Rol_rm16_imm8 => Some(Transform::RotateLeft16(instruction.immediate8() as u32)),
        Code::Rol_rm32_imm8 => Some(Transform::RotateLeft32(instruction.immediate8() as u32)),
        Code::Rol_rm64_imm8 => Some(Transform::RotateLeft64(instruction.immediate8() as u32)),

        Code::Ror_rm8_imm8 => Some(Transform::RotateRight8(instruction.immediate8() as u32)),
        Code::Ror_rm16_imm8 => Some(Transform::RotateRight16(instruction.immediate8() as u32)),
        Code::Ror_rm32_imm8 => Some(Transform::RotateRight32(instruction.immediate8() as u32)),
        Code::Ror_rm64_imm8 => Some(Transform::RotateRight64(instruction.immediate8() as u32)),

        Code::Rol_rm8_1 => Some(Transform::RotateLeft8(1u32)),
        Code::Rol_rm16_1 => Some(Transform::RotateLeft16(1u32)),
        Code::Rol_rm32_1 => Some(Transform::RotateLeft32(1u32)),
        Code::Rol_rm64_1 => Some(Transform::RotateLeft64(1u32)),

        Code::Ror_rm8_1 => Some(Transform::RotateRight8(1u32)),
        Code::Ror_rm16_1 => Some(Transform::RotateRight16(1u32)),
        Code::Ror_rm32_1 => Some(Transform::RotateRight32(1u32)),
        Code::Ror_rm64_1 => Some(Transform::RotateRight64(1u32)),

        Code::Inc_rm8 => Some(Transform::Increment8),
        Code::Inc_rm16 => Some(Transform::Increment16),
        Code::Inc_rm32 => Some(Transform::Increment32),
        Code::Inc_rm64 => Some(Transform::Increment64),

        Code::Dec_rm8 => Some(Transform::Decrement8),
        Code::Dec_rm16 => Some(Transform::Decrement16),
        Code::Dec_rm32 => Some(Transform::Decrement32),
        Code::Dec_rm64 => Some(Transform::Decrement64),

        Code::Xor_AL_imm8 => Some(Transform::XorConstant8(instruction.immediate8())),
        Code::Xor_rm8_imm8 => Some(Transform::XorConstant8(instruction.immediate8())),
        Code::Xor_AX_imm16 => Some(Transform::XorConstant16(instruction.immediate16())),
        Code::Xor_rm16_imm16 => Some(Transform::XorConstant16(instruction.immediate16())),
        Code::Xor_EAX_imm32 => Some(Transform::XorConstant32(instruction.immediate32())),
        Code::Xor_rm32_imm32 => Some(Transform::XorConstant32(instruction.immediate32())),
        Code::Xor_RAX_imm32 => Some(Transform::XorConstant64(instruction.immediate64())),
        Code::Xor_rm64_imm32 => Some(Transform::XorConstant64(instruction.immediate64())),
        _ => None,
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Transform {
    ByteSwap64,
    ByteSwap32,
    ByteSwap16,

    SubtractConstant64(u64),
    SubtractConstant32(u32),
    SubtractConstant16(u16),
    SubtractConstant8(u8),

    AddConstant64(u64),
    AddConstant32(u32),
    AddConstant16(u16),
    AddConstant8(u8),

    Negate64,
    Negate32,
    Negate16,
    Negate8,

    Not64,
    Not32,
    Not16,
    Not8,

    RotateLeft64(u32),
    RotateLeft32(u32),
    RotateLeft16(u32),
    RotateLeft8(u32),

    RotateRight64(u32),
    RotateRight32(u32),
    RotateRight16(u32),
    RotateRight8(u32),

    Increment64,
    Increment32,
    Increment16,
    Increment8,

    Decrement64,
    Decrement32,
    Decrement16,
    Decrement8,

    XorConstant64(u64),
    XorConstant32(u32),
    XorConstant16(u16),
    XorConstant8(u8),
}

impl Transform {
    pub fn get_size(&self) -> usize {
        match self {
            Self::ByteSwap64 => 64,
            Self::ByteSwap32 => 32,
            Self::ByteSwap16 => 16,

            Self::SubtractConstant64(_) => 64,
            Self::SubtractConstant32(_) => 32,
            Self::SubtractConstant16(_) => 16,
            Self::SubtractConstant8(_) => 8,

            Self::AddConstant64(_) => 64,
            Self::AddConstant32(_) => 32,
            Self::AddConstant16(_) => 16,
            Self::AddConstant8(_) => 8,

            Self::Negate64 => 64,
            Self::Negate32 => 32,
            Self::Negate16 => 16,
            Self::Negate8 => 8,

            Self::Not64 => 64,
            Self::Not32 => 32,
            Self::Not16 => 16,
            Self::Not8 => 8,

            Self::RotateLeft64(_) => 64,
            Self::RotateLeft32(_) => 32,
            Self::RotateLeft16(_) => 16,
            Self::RotateLeft8(_) => 8,

            Self::RotateRight64(_) => 64,
            Self::RotateRight32(_) => 32,
            Self::RotateRight16(_) => 16,
            Self::RotateRight8(_) => 8,

            Self::Increment64 => 64,
            Self::Increment32 => 32,
            Self::Increment16 => 16,
            Self::Increment8 => 8,

            Self::Decrement64 => 64,
            Self::Decrement32 => 32,
            Self::Decrement16 => 16,
            Self::Decrement8 => 8,

            Self::XorConstant64(_) => 64,
            Self::XorConstant32(_) => 32,
            Self::XorConstant16(_) => 16,
            Self::XorConstant8(_) => 8,
        }
    }
}

pub trait EmulateTransform {
    fn emulate_transform(self,
                         transform: Transform)
                         -> Self;
}

impl EmulateTransform for u8 {
    fn emulate_transform(self,
                         transform: Transform)
                         -> Self {
        emulate_transform8(transform, self)
    }
}

impl EmulateTransform for u16 {
    fn emulate_transform(self,
                         transform: Transform)
                         -> Self {
        emulate_transform16(transform, self)
    }
}

impl EmulateTransform for u32 {
    fn emulate_transform(self,
                         transform: Transform)
                         -> Self {
        emulate_transform32(transform, self)
    }
}

impl EmulateTransform for u64 {
    fn emulate_transform(self,
                         transform: Transform)
                         -> Self {
        emulate_transform64(transform, self)
    }
}

pub trait EmulateEncryption {
    fn emulate_encryption<'a, I>(self,
                                 instruction_iter: I,
                                 rolling_key: &mut u64,
                                 encrypted_reg: Register)
                                 -> Self
        where I: Iterator<Item = &'a Instruction>;
}

impl EmulateEncryption for u64 {
    fn emulate_encryption<'a, I>(mut self,
                                 instruction_iter: I,
                                 rolling_key: &mut u64,
                                 encrypted_reg: Register)
                                 -> Self
        where I: Iterator<Item = &'a Instruction>
    {
        self ^= *rolling_key as u64;

        for instruction in
            instruction_iter.filter(|&insn| check_full_reg_written(&insn, encrypted_reg))
        {
            let transform = get_transform_for_instruction(&instruction);

            if let Some(transform) = transform {
                self = self.emulate_transform(transform);
            }
        }

        *rolling_key ^= self as u64;

        self
    }
}

impl EmulateEncryption for u32 {
    fn emulate_encryption<'a, I>(mut self,
                                 instruction_iter: I,
                                 rolling_key: &mut u64,
                                 encrypted_reg: Register)
                                 -> Self
        where I: Iterator<Item = &'a Instruction>
    {
        self ^= *rolling_key as u32;

        for instruction in
            instruction_iter.filter(|&insn| check_full_reg_written(&insn, encrypted_reg))
        {

            let transform = get_transform_for_instruction(&instruction);

            if let Some(transform) = transform {
                self = self.emulate_transform(transform);
            }
        }

        *rolling_key ^= self as u64;

        self
    }
}

impl EmulateEncryption for u16 {
    fn emulate_encryption<'a, I>(mut self,
                                 instruction_iter: I,
                                 rolling_key: &mut u64,
                                 encrypted_reg: Register)
                                 -> Self
        where I: Iterator<Item = &'a Instruction>
    {
        self ^= *rolling_key as u16;

        for instruction in
            instruction_iter.filter(|&insn| check_full_reg_written(&insn, encrypted_reg))
        {
            let transform = get_transform_for_instruction(&instruction);

            if let Some(transform) = transform {
                self = self.emulate_transform(transform);
            }
        }

        *rolling_key ^= self as u64;

        self
    }
}

impl EmulateEncryption for u8 {
    fn emulate_encryption<'a, I>(mut self,
                                 instruction_iter: I,
                                 rolling_key: &mut u64,
                                 encrypted_reg: Register)
                                 -> Self
        where I: Iterator<Item = &'a Instruction>
    {
        self ^= *rolling_key as u8;

        for instruction in
            instruction_iter.filter(|&insn| check_full_reg_written(&insn, encrypted_reg))
        {
            let transform = get_transform_for_instruction(&instruction);
            if let Some(transform) = transform {
                self = self.emulate_transform(transform);
            }
        }

        *rolling_key ^= self as u64;

        self
    }
}

fn emulate_transform64(transform: Transform,
                       input: u64)
                       -> u64 {
    match transform {
        Transform::ByteSwap64 => input.swap_bytes(),

        Transform::SubtractConstant64(amount) => input.wrapping_sub(amount),

        Transform::AddConstant64(amount) => input.wrapping_add(amount),

        Transform::XorConstant64(amount) => input ^ amount,

        Transform::Negate64 => (!input).wrapping_add(1),

        Transform::Not64 => !input,

        Transform::RotateLeft64(amount) => input.rotate_left(amount),

        Transform::RotateRight64(amount) => input.rotate_right(amount),

        Transform::Decrement64 => input.wrapping_sub(1),

        Transform::Increment64 => input.wrapping_add(1),
        _ => {
            dbg!(transform);
            unreachable!();
        },
    }
}

fn emulate_transform32(transform: Transform,
                       input: u32)
                       -> u32 {
    match transform {
        Transform::ByteSwap32 => input.swap_bytes(),

        Transform::SubtractConstant32(amount) => input.wrapping_sub(amount),

        Transform::AddConstant32(amount) => input.wrapping_add(amount),

        Transform::XorConstant32(amount) => input ^ amount,

        Transform::Negate32 => (!input).wrapping_add(1),

        Transform::Not32 => !input,

        Transform::RotateLeft32(amount) => input.rotate_left(amount),

        Transform::RotateRight32(amount) => input.rotate_right(amount),

        Transform::Decrement32 => input.wrapping_sub(1),

        Transform::Increment32 => input.wrapping_add(1),
        _ => {
            dbg!(transform);
            unreachable!();
        },
    }
}

fn emulate_transform16(transform: Transform,
                       input: u16)
                       -> u16 {
    match transform {
        Transform::ByteSwap16 => input.swap_bytes(),

        Transform::SubtractConstant16(amount) => input.wrapping_sub(amount),

        Transform::AddConstant16(amount) => input.wrapping_add(amount),

        Transform::XorConstant16(amount) => input ^ amount,

        Transform::Negate16 => (!input).wrapping_add(1),

        Transform::Not16 => !input,

        Transform::RotateLeft16(amount) => input.rotate_left(amount),

        Transform::RotateRight16(amount) => input.rotate_right(amount),

        Transform::Decrement16 => input.wrapping_sub(1),

        Transform::Increment16 => input.wrapping_add(1),
        _ => {
            dbg!(transform);
            unreachable!();
        },
    }
}

fn emulate_transform8(transform: Transform,
                      input: u8)
                      -> u8 {
    match transform {
        Transform::SubtractConstant8(amount) => input.wrapping_sub(amount),

        Transform::AddConstant8(amount) => input.wrapping_add(amount),

        Transform::XorConstant8(amount) => input ^ amount,

        Transform::Negate8 => (!input).wrapping_add(1),

        Transform::Not8 => !input,

        Transform::RotateLeft8(amount) => input.rotate_left(amount),

        Transform::RotateRight8(amount) => input.rotate_right(amount),

        Transform::Decrement8 => input.wrapping_sub(1),

        Transform::Increment8 => input.wrapping_add(1),
        _ => {
            dbg!(transform);
            unreachable!();
        },
    }
}
