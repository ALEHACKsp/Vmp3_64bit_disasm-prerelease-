use crate::{
    match_assembly::{
        match_fetch_encrypted_vip, match_fetch_vip, match_push_rolling_key,
        match_xor_16_rolling_key_dest, match_xor_16_rolling_key_source,
        match_xor_32_rolling_key_source, match_xor_64_rolling_key_dest,
        match_xor_64_rolling_key_source, match_xor_8_rolling_key_dest,
        match_xor_8_rolling_key_source,
    },
    transforms::{get_transform_for_instruction, EmulateEncryption, EmulateTransform},
    util::*,
};
use iced_x86::{Code, Instruction, OpKind};
use pelite::pe64::PeFile;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct VmRegisterAllocation {
    pub vip: Registers,
    pub vsp: Registers,
    pub key: Registers,
    pub handler_address: Registers,
}

#[derive(Debug)]
pub struct VmContext {
    /// Register allocation of the vm
    pub register_allocation: VmRegisterAllocation,
    /// VmEntry address
    pub vm_entry_address: u64,
    /// Pushed value
    pub pushed_val: u64,
    /// Vip direction
    pub vip_direction_forwards: bool,
    /// Register push order
    pub push_order: Vec<Registers>,
    /// Key value
    pub rolling_key: u64,
    /// Current vip
    pub vip_value: u64,
    /// Next handler address
    pub handler_address: u64,
}

impl VmContext {
    pub fn new(pe_file: &PeFile,
               pe_bytes: &[u8],
               vm_call_address: u64)
               -> Self {
        let (pushed_val, vm_entry_address) = handle_vm_call(&pe_file, &pe_bytes, vm_call_address);

        let vm_entry_handler = VmHandler::new(vm_entry_address, &pe_file, &pe_bytes);

        let push_order = vm_entry_handler.get_push_order_vm_entry();

        let register_allocation = vm_entry_handler.get_register_allocation_vm_entry();

        let direction_is_forwards = vm_entry_handler.determine_is_forwards(&register_allocation);

        // Get the initial_vip
        let initial_vip =
            vm_entry_handler.get_initial_vip(&register_allocation, pushed_val) + 0x100000000;
        let mut vip = initial_vip;

        // Rolling key is initialized to the initial vip
        let mut rolling_key = initial_vip;

        // Get the handler base address value
        let instruction_iter = vm_entry_handler.instructions.iter();
        let mut instruction_iter = instruction_iter.skip_while(|insn| {
                                                       !(insn.code() == Code::Lea_r64_m &&
                                                         insn.memory_displacement64() != 0)
                                                   });
        let handler_base_address = instruction_iter.next().unwrap().memory_displacement64();
        let mut instruction_iter =
            instruction_iter.skip_while(|insn| !match_fetch_vip(insn, &register_allocation));

        // Get the reg where the encrypted offset has been loaded into
        let encrypted_offset_reg = instruction_iter.next().unwrap().op0_register();

        let encrypted_offset = fetch_dword_vip(pe_file, pe_bytes, &mut vip, direction_is_forwards);

        let encryption_iter = instruction_iter.take_while(|&insn| {
                                                  !(match_push_rolling_key(insn,
                                                                           &register_allocation))
                                              });

        let unencrypted_offset = encrypted_offset.emulate_encryption(encryption_iter,
                                                                     &mut rolling_key,
                                                                     encrypted_offset_reg);

        // hmmm yes
        // movsxd offset_reg, offset_reg_32
        // add handler_base, offset_reg
        let next_handler_address =
            handler_base_address.wrapping_add(unencrypted_offset as i32 as i64 as u64);

        let vip_value = vip;
        Self { register_allocation,
               vm_entry_address,
               pushed_val,
               vip_direction_forwards: direction_is_forwards,
               push_order,
               rolling_key,
               vip_value,
               handler_address: next_handler_address }
    }

    pub fn disassemble_single_dword_operand(&mut self,
                                            vm_handler: &VmHandler,
                                            pe_file: &PeFile,
                                            pe_bytes: &[u8])
                                            -> u32 {
        let instruction_iter = vm_handler.instructions.iter();
        let mut instruction_iter = instruction_iter.skip_while(|insn| {
                                                       !match_xor_32_rolling_key_source(insn,
                                                                       &self.register_allocation)
                                                   });
        let encrypted_reg = instruction_iter.next().unwrap().op0_register();

        let encryption_iter = instruction_iter.take_while(|insn| {
                                                  !match_push_rolling_key(insn,
                                                                          &self.register_allocation)
                                              });

        let encrypted_dword = fetch_dword_vip(pe_file,
                                              pe_bytes,
                                              &mut self.vip_value,
                                              self.vip_direction_forwards);

        let return_dword = encrypted_dword.emulate_encryption(encryption_iter,
                                                              &mut self.rolling_key,
                                                              encrypted_reg);

        let encrypted_offset = fetch_dword_vip(pe_file,
                                               pe_bytes,
                                               &mut self.vip_value,
                                               self.vip_direction_forwards);

        let instruction_iter = vm_handler.instructions.iter();
        // Skip it twice because dword arg
        let mut match_count = 0;
        let mut instruction_iter = instruction_iter.skip_while(|insn| {
                                                       if match_xor_32_rolling_key_source(insn,
                                                                          &self.register_allocation)
                                       {
                                           match_count += 1;
                                       }

                                                       match_count != 2
                                                   });

        let encrypted_reg = instruction_iter.next().unwrap().op0_register();
        let encryption_iter = instruction_iter.take_while(|insn| {
                                                  !match_push_rolling_key(insn,
                                                                          &self.register_allocation)
                                              });

        let unencrypted_offset = encrypted_offset.emulate_encryption(encryption_iter,
                                                                     &mut self.rolling_key,
                                                                     encrypted_reg);

        // hmmm yes
        // movsxd offset_reg, offset_reg_32
        // add handler_base, offset_reg
        let next_handler_address = self.handler_address
                                       .wrapping_add(unencrypted_offset as i32 as i64 as u64);

        self.handler_address = next_handler_address;

        return_dword
    }

    pub fn disassemble_single_qword_operand(&mut self,
                                            vm_handler: &VmHandler,
                                            pe_file: &PeFile,
                                            pe_bytes: &[u8])
                                            -> u64 {
        let instruction_iter = vm_handler.instructions.iter();
        let mut instruction_iter = instruction_iter.skip_while(|insn| {
                                                       !match_xor_64_rolling_key_source(insn,
                                                                       &self.register_allocation)
                                                   });
        let encrypted_reg = instruction_iter.next().unwrap().op0_register();

        let encryption_iter = instruction_iter.take_while(|insn| {
                                  !match_xor_64_rolling_key_dest(insn, &self.register_allocation)
                              });
        let encrypted_qword = fetch_qword_vip(pe_file,
                                              pe_bytes,
                                              &mut self.vip_value,
                                              self.vip_direction_forwards);

        let return_qword = encrypted_qword.emulate_encryption(encryption_iter,
                                                              &mut self.rolling_key,
                                                              encrypted_reg);

        let encrypted_offset = fetch_dword_vip(pe_file,
                                               pe_bytes,
                                               &mut self.vip_value,
                                               self.vip_direction_forwards);

        let instruction_iter = vm_handler.instructions.iter();
        let mut instruction_iter = instruction_iter.skip_while(|insn| {
                                                       !match_xor_32_rolling_key_source(insn,
                                                                       &self.register_allocation)
                                                   });
        let encrypted_reg = instruction_iter.next().unwrap().op0_register();
        let encryption_iter = instruction_iter.take_while(|insn| {
                                                  !match_push_rolling_key(insn,
                                                                          &self.register_allocation)
                                              });

        let unencrypted_offset = encrypted_offset.emulate_encryption(encryption_iter,
                                                                     &mut self.rolling_key,
                                                                     encrypted_reg);

        // hmmm yes
        // movsxd offset_reg, offset_reg_32
        // add handler_base, offset_reg
        let next_handler_address = self.handler_address
                                       .wrapping_add(unencrypted_offset as i32 as i64 as u64);

        self.handler_address = next_handler_address;

        return_qword
    }

    pub fn disassemble_single_word_operand(&mut self,
                                           vm_handler: &VmHandler,
                                           pe_file: &PeFile,
                                           pe_bytes: &[u8])
                                           -> u16 {
        let instruction_iter = vm_handler.instructions.iter();
        let mut instruction_iter = instruction_iter.skip_while(|insn| {
                                                       !match_xor_16_rolling_key_source(insn,
                                                                       &self.register_allocation)
                                                   });
        let encrypted_reg = instruction_iter.next().unwrap().op0_register();

        let encryption_iter = instruction_iter.take_while(|insn| {
                                  !match_xor_16_rolling_key_dest(insn, &self.register_allocation)
                              });
        let encrypted_word = fetch_word_vip(pe_file,
                                            pe_bytes,
                                            &mut self.vip_value,
                                            self.vip_direction_forwards);

        let return_word = encrypted_word.emulate_encryption(encryption_iter,
                                                            &mut self.rolling_key,
                                                            encrypted_reg);

        let encrypted_offset = fetch_dword_vip(pe_file,
                                               pe_bytes,
                                               &mut self.vip_value,
                                               self.vip_direction_forwards);

        let instruction_iter = vm_handler.instructions.iter();
        let mut instruction_iter = instruction_iter.skip_while(|insn| {
                                                       !match_xor_32_rolling_key_source(insn,
                                                                       &self.register_allocation)
                                                   });
        let encrypted_reg = instruction_iter.next().unwrap().op0_register();
        let encryption_iter = instruction_iter.take_while(|insn| {
                                                  !match_push_rolling_key(insn,
                                                                          &self.register_allocation)
                                              });

        let unencrypted_offset = encrypted_offset.emulate_encryption(encryption_iter,
                                                                     &mut self.rolling_key,
                                                                     encrypted_reg);

        // hmmm yes
        // movsxd offset_reg, offset_reg_32
        // add handler_base, offset_reg
        let next_handler_address = self.handler_address
                                       .wrapping_add(unencrypted_offset as i32 as i64 as u64);

        self.handler_address = next_handler_address;

        return_word
    }

    pub fn disassemble_single_byte_operand(&mut self,
                                           vm_handler: &VmHandler,
                                           pe_file: &PeFile,
                                           pe_bytes: &[u8])
                                           -> u8 {
        let instruction_iter = vm_handler.instructions.iter();
        let mut instruction_iter = instruction_iter.skip_while(|insn| {
                                                       !match_xor_8_rolling_key_source(insn,
                                                                       &self.register_allocation)
                                                   });
        let encrypted_reg = instruction_iter.next().unwrap().op0_register();

        let encryption_iter = instruction_iter.take_while(|insn| {
                                  !match_xor_8_rolling_key_dest(insn, &self.register_allocation)
                              });
        let encrypted_byte = fetch_byte_vip(pe_file,
                                            pe_bytes,
                                            &mut self.vip_value,
                                            self.vip_direction_forwards);

        let return_byte = encrypted_byte.emulate_encryption(encryption_iter,
                                                            &mut self.rolling_key,
                                                            encrypted_reg);

        let encrypted_offset = fetch_dword_vip(pe_file,
                                               pe_bytes,
                                               &mut self.vip_value,
                                               self.vip_direction_forwards);

        let instruction_iter = vm_handler.instructions.iter();
        let mut instruction_iter = instruction_iter.skip_while(|insn| {
                                                       !match_xor_32_rolling_key_source(insn,
                                                                       &self.register_allocation)
                                                   });
        let encrypted_reg = instruction_iter.next().unwrap().op0_register();
        let encryption_iter = instruction_iter.take_while(|insn| {
                                                  !match_push_rolling_key(insn,
                                                                          &self.register_allocation)
                                              });

        let unencrypted_offset = encrypted_offset.emulate_encryption(encryption_iter,
                                                                     &mut self.rolling_key,
                                                                     encrypted_reg);

        // hmmm yes
        // movsxd offset_reg, offset_reg_32
        // add handler_base, offset_reg
        let next_handler_address = self.handler_address
                                       .wrapping_add(unencrypted_offset as i32 as i64 as u64);

        self.handler_address = next_handler_address;

        return_byte
    }

    pub fn disassemble_no_operand(&mut self,
                                  vm_handler: &VmHandler,
                                  pe_file: &PeFile,
                                  pe_bytes: &[u8]) {
        let encrypted_offset = fetch_dword_vip(pe_file,
                                               pe_bytes,
                                               &mut self.vip_value,
                                               self.vip_direction_forwards);

        let instruction_iter = vm_handler.instructions.iter();
        let mut instruction_iter = instruction_iter.skip_while(|insn| {
                                                       !match_xor_32_rolling_key_source(insn,
                                                                       &self.register_allocation)
                                                   });
        let encrypted_reg = instruction_iter.next().unwrap().op0_register();
        let encryption_iter = instruction_iter.take_while(|insn| {
                                                  !match_push_rolling_key(insn,
                                                                          &self.register_allocation)
                                              });

        let unencrypted_offset = encrypted_offset.emulate_encryption(encryption_iter,
                                                                     &mut self.rolling_key,
                                                                     encrypted_reg);

        // hmmm yes
        // movsxd offset_reg, offset_reg_32
        // add handler_base, offset_reg
        let next_handler_address = self.handler_address
                                       .wrapping_add(unencrypted_offset as i32 as i64 as u64);

        self.handler_address = next_handler_address;
    }
}

pub struct VmHandler {
    pub instructions: Vec<Instruction>,
}

impl VmHandler {
    pub fn new(address: u64,
               pe_file: &PeFile,
               pe_bytes: &[u8])
               -> Self {
        let mut instruction_address = address;
        let mut instructions = Vec::new();

        loop {
            let instruction = disassemble_instruction_at_va(pe_file, pe_bytes, instruction_address);

            match instruction.code() {
                Code::Retnq | Code::Jmp_rm64 => {
                    instructions.push(instruction);
                    break;
                },
                Code::Jmp_rel32_64 => {
                    let jmp_target = instruction.near_branch64();
                    instruction_address = jmp_target;
                },

                _ => {
                    instruction_address += instruction.len() as u64;
                    instructions.push(instruction);
                },
            }
        }

        Self { instructions }
    }

    pub fn get_register_allocation_vm_entry(&self) -> VmRegisterAllocation {
        // Find the handler_address register
        let handler_address_reg = {
            let instruction_last = self.instructions.last().unwrap();
            if instruction_last.code() == Code::Jmp_rm64 {
                instruction_last.op0_register().into()
            } else {
                let instruction = self.instructions
                                      .iter()
                                      .rev()
                                      .find(|&&insn| insn.code() == Code::Push_r64)
                                      .unwrap();
                instruction.op0_register().into()
            }
        };

        // Find key register
        let pop_instruction = self.instructions
                                  .iter()
                                  .rev()
                                  .find(|&&insn| insn.code() == Code::Pop_r64)
                                  .unwrap();

        let key = pop_instruction.op0_register().into();

        // Find vsp register
        let mov_vsp_instruction = self.instructions
                                      .iter()
                                      .find(|&&insn| {
                                          insn.code() == Code::Mov_r64_rm64 &&
                                          insn.op1_register() == iced_x86::Register::RSP
                                      })
                                      .unwrap();
        let vsp = mov_vsp_instruction.op0_register().into();

        // Find vip register
        let mov_vip_instruction = self.instructions
                                      .iter()
                                      .find(|&&insn| {
                                          insn.code() == Code::Mov_r64_rm64 &&
                                          insn.op1_kind() == OpKind::Memory &&
                                          insn.memory_displacement64() == 0x90
                                      })
                                      .unwrap();
        let vip = mov_vip_instruction.op0_register().into();

        VmRegisterAllocation { vip,
                               vsp,
                               key,
                               handler_address: handler_address_reg }
    }

    pub fn get_push_order_vm_entry(&self) -> Vec<Registers> {
        let mut registers = Vec::new();

        for instruction in self.instructions
                               .iter()
                               .take_while(|&&insn| !(insn.code() == Code::Mov_r64_imm64))
        {
            match instruction.code() {
                Code::Push_r64 => {
                    let reg = instruction.op0_register();
                    registers.push(reg.into());
                },
                Code::Pushfq => {
                    registers.push(Registers::Flags);
                },
                _ => {},
            }
        }

        registers
    }

    pub fn determine_is_forwards(&self,
                                 reg_allocation: &VmRegisterAllocation)
                                 -> bool {
        for instruction in self.instructions.iter() {
            match instruction.code() {
                Code::Add_rm64_imm32 => {
                    if reg_allocation.vip == instruction.op0_register().into() {
                        if instruction.immediate32() == 0x4 {
                            return true;
                        }
                    }
                },
                Code::Sub_rm64_imm32 => {
                    if reg_allocation.vip == instruction.op0_register().into() {
                        if instruction.immediate32() == 0x4 {
                            return false;
                        }
                    }
                },
                _ => continue,
            }
        }
        panic!("Direction of vm_entry not found")
    }

    pub fn get_initial_vip(&self,
                           reg_allocation: &VmRegisterAllocation,
                           pushed_val: u64)
                           -> u64 {
        let mut encrypted_vip = pushed_val as u32;
        for instruction in
            self.instructions
                .iter()
                .skip_while(|&insn| !match_fetch_encrypted_vip(insn, reg_allocation))
                .take_while(|&insn| {
                    !((insn.code() == Code::Lea_r64_m || insn.code() == Code::Add_r64_rm64) &&
                      check_full_reg_written(insn, reg_allocation.vip.into()))
                })
                .filter(|&insn| check_full_reg_written(insn, reg_allocation.vip.into()))
        {
            let transform = get_transform_for_instruction(instruction);

            if let Some(transform) = transform {
                encrypted_vip = encrypted_vip.emulate_transform(transform);
            }
        }

        encrypted_vip as u64
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Registers {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rsp,
    Rbp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    Flags,
}

impl From<iced_x86::Register> for Registers {
    fn from(reg: iced_x86::Register) -> Self {
        match reg {
            iced_x86::Register::RAX => Registers::Rax,
            iced_x86::Register::RBX => Registers::Rbx,
            iced_x86::Register::RCX => Registers::Rcx,
            iced_x86::Register::RDX => Registers::Rdx,
            iced_x86::Register::RSI => Registers::Rsi,
            iced_x86::Register::RDI => Registers::Rdi,
            iced_x86::Register::RSP => Registers::Rsp,
            iced_x86::Register::RBP => Registers::Rbp,
            iced_x86::Register::R8 => Registers::R8,
            iced_x86::Register::R9 => Registers::R9,
            iced_x86::Register::R10 => Registers::R10,
            iced_x86::Register::R11 => Registers::R11,
            iced_x86::Register::R12 => Registers::R12,
            iced_x86::Register::R13 => Registers::R13,
            iced_x86::Register::R14 => Registers::R14,
            iced_x86::Register::R15 => Registers::R15,

            _ => unimplemented!("Register not implemented"),
        }
    }
}

impl Into<iced_x86::Register> for Registers {
    fn into(self) -> iced_x86::Register {
        match self {
            Registers::Rax => iced_x86::Register::RAX,
            Registers::Rbx => iced_x86::Register::RBX,
            Registers::Rcx => iced_x86::Register::RCX,
            Registers::Rdx => iced_x86::Register::RDX,
            Registers::Rsi => iced_x86::Register::RSI,
            Registers::Rdi => iced_x86::Register::RDI,
            Registers::Rsp => iced_x86::Register::RSP,
            Registers::Rbp => iced_x86::Register::RBP,
            Registers::R8 => iced_x86::Register::R8,
            Registers::R9 => iced_x86::Register::R9,
            Registers::R10 => iced_x86::Register::R10,
            Registers::R11 => iced_x86::Register::R11,
            Registers::R12 => iced_x86::Register::R12,
            Registers::R13 => iced_x86::Register::R13,
            Registers::R14 => iced_x86::Register::R14,
            Registers::R15 => iced_x86::Register::R15,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum HandlerType {
    VmEntry,
}

pub fn fetch_qword_vip(pe_file: &PeFile,
                       pe_bytes: &[u8],
                       vip: &mut u64,
                       direction_is_forwards: bool)
                       -> u64 {
    let return_value;

    if direction_is_forwards {
        return_value = u64::from_le_bytes(read_bytes_at_va(pe_file, pe_bytes, *vip, 8).unwrap()
                                                                                      .try_into()
                                                                                      .unwrap());
        *vip += 8;
    } else {
        *vip -= 8;
        return_value = u64::from_le_bytes(read_bytes_at_va(pe_file, pe_bytes, *vip, 8).unwrap()
                                                                                      .try_into()
                                                                                      .unwrap());
    }

    return_value
}

pub fn fetch_word_vip(pe_file: &PeFile,
                      pe_bytes: &[u8],
                      vip: &mut u64,
                      direction_is_forwards: bool)
                      -> u16 {
    let return_value;

    if direction_is_forwards {
        return_value = u16::from_le_bytes(read_bytes_at_va(pe_file, pe_bytes, *vip, 2).unwrap()
                                                                                      .try_into()
                                                                                      .unwrap());
        *vip += 2;
    } else {
        *vip -= 2;
        return_value = u16::from_le_bytes(read_bytes_at_va(pe_file, pe_bytes, *vip, 2).unwrap()
                                                                                      .try_into()
                                                                                      .unwrap());
    }

    return_value
}

pub fn fetch_dword_vip(pe_file: &PeFile,
                       pe_bytes: &[u8],
                       vip: &mut u64,
                       direction_is_forwards: bool)
                       -> u32 {
    let return_value;

    if direction_is_forwards {
        return_value = u32::from_le_bytes(read_bytes_at_va(pe_file, pe_bytes, *vip, 4).unwrap()
                                                                                      .try_into()
                                                                                      .unwrap());
        *vip += 4;
    } else {
        *vip -= 4;
        return_value = u32::from_le_bytes(read_bytes_at_va(pe_file, pe_bytes, *vip, 4).unwrap()
                                                                                      .try_into()
                                                                                      .unwrap());
    }

    return_value
}

pub fn fetch_byte_vip(pe_file: &PeFile,
                      pe_bytes: &[u8],
                      vip: &mut u64,
                      direction_is_forwards: bool)
                      -> u8 {
    let return_value;

    if direction_is_forwards {
        return_value = read_bytes_at_va(pe_file, pe_bytes, *vip, 4).unwrap()[0];
        *vip += 1;
    } else {
        *vip -= 1;
        return_value = read_bytes_at_va(pe_file, pe_bytes, *vip, 4).unwrap()[0];
    }

    return_value
}
