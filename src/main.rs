use std::{error::Error, path::PathBuf};

use iced_x86::{Formatter, NasmFormatter};
use pelite::pe64::PeFile;
use pelite::FileMap;

mod match_assembly;
mod transforms;
mod util;
mod vm_handler;
mod vm_matchers;

use clap::Parser;
use vm_handler::{VmContext, VmHandler};

use crate::util::handle_vm_call;
use crate::vm_matchers::HandlerClass;

#[derive(Parser, Debug)]
struct CommandLineArgs {
    /// Input file
    #[clap(value_parser)]
    pub input_file:      String,
    /// Vm call address
    /// Address of the push instruction in
    /// push <const>
    /// call vm_entry
    #[clap(short, long, value_parser)]
    pub vm_call_address: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let command_line_args = CommandLineArgs::parse();
    let input_file = &command_line_args.input_file;

    let map = FileMap::open(input_file)?;
    let pe_file = PeFile::from_bytes(&map)?;
    let pe_bytes = std::fs::read(input_file)?;

    let mut vm_context = VmContext::new(&pe_file, &pe_bytes, command_line_args.vm_call_address);

    loop {
        let vm_handler = VmHandler::new(vm_context.handler_address, &pe_file, &pe_bytes);
        let handler_class = vm_handler.match_handler_class(&vm_context.register_allocation);
        match handler_class {
            HandlerClass::UnconditionalBranch => {
                println!("Disassembled unconditional branch");
                println!("[Stopping]");
                break;
            },
            HandlerClass::VmExit => {
                println!("Disassembled vm_exit");
                println!("[Stopping]");
                break;
            },
            HandlerClass::ByteOperand => {
                println!("Disassembled single byte operand");
                vm_context.disassemble_single_byte_operand(&vm_handler, &pe_file, &pe_bytes);
            },
            HandlerClass::DwordOperand => {
                println!("Disassembled single dword operand");
                vm_context.disassemble_single_dword_operand(&vm_handler, &pe_file, &pe_bytes);
            },
            HandlerClass::WordOperand => {
                println!("Disassembled single word operand");
                vm_context.disassemble_single_word_operand(&vm_handler, &pe_file, &pe_bytes);
            },
            HandlerClass::QwordOperand => {
                println!("Disassembled single qword operand");
                vm_context.disassemble_single_qword_operand(&vm_handler, &pe_file, &pe_bytes);
            },
            HandlerClass::NoOperand => {
                println!("Disassembled no operand");
                vm_context.disassemble_no_operand(&vm_handler, &pe_file, &pe_bytes);
            },
        }
        println!("{:#x?}", vm_context);
    }
    Ok(())
}
