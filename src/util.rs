use iced_x86::{
    Code, Decoder, DecoderOptions, Formatter, Instruction, InstructionInfoFactory,
    InstructionInfoOptions, NasmFormatter, OpAccess, Register,
};
use pelite::pe64::{Pe, PeFile};

pub fn read_bytes_at_va<'a>(pe_file: &'_ PeFile,
                            pe_bytes: &'a [u8],
                            va: u64,
                            size: usize)
                            -> Result<&'a [u8], pelite::Error> {
    let rva = pe_file.va_to_rva(va)?;
    let file_offset = pe_file.rva_to_file_offset(rva)?;

    let bytes = &pe_bytes[file_offset .. file_offset + size];
    Ok(bytes)
}

pub fn disassemble_instruction_at_va(pe_file: &PeFile,
                                     pe_bytes: &[u8],
                                     instruction_address: u64)
                                     -> Instruction {
    let instruction_bytes = read_bytes_at_va(pe_file, pe_bytes, instruction_address, 16).unwrap();

    let mut decoder = Decoder::with_ip(64,
                                       instruction_bytes,
                                       instruction_address,
                                       DecoderOptions::NONE);

    let instruction = decoder.decode();
    instruction
}

pub fn handle_vm_call(pe_file: &PeFile,
                      pe_bytes: &[u8],
                      push_call_addr: u64)
                      -> (u64, u64) {
    let push_instruction = disassemble_instruction_at_va(pe_file, pe_bytes, push_call_addr);
    let call_instruction = disassemble_instruction_at_va(pe_file,
                                                         pe_bytes,
                                                         push_call_addr +
                                                         push_instruction.len() as u64);
    if push_instruction.code() != Code::Pushq_imm32 {
        panic!("Vm Entry address is not correctly chosen");
    }

    if call_instruction.code() != Code::Call_rel32_64 {
        panic!("Vm Entry address is not correctly chosen");
    }

    let pushed_val = push_instruction.immediate32to64() as u64;
    let vm_entry_address = call_instruction.near_branch64();

    (pushed_val, vm_entry_address)
}

pub fn check_full_reg_written(instruction: &Instruction,
                              reg: Register)
                              -> bool {
    let mut instruction_info_factory = InstructionInfoFactory::new();
    let instruction_info =
        instruction_info_factory.info_options(instruction, InstructionInfoOptions::NO_MEMORY_USAGE);

    let used_registers = instruction_info.used_registers();

    for used_register in
        used_registers.iter()
                      .filter(|r| r.register().full_register() == reg.full_register())
    {
        match used_register.access() {
            OpAccess::Write |
            OpAccess::CondWrite |
            OpAccess::ReadWrite |
            OpAccess::ReadCondWrite => {
                return true;
            },
            _ => {
                continue;
            },
        }
    }

    false
}
