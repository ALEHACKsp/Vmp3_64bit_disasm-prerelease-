use std::{error::Error, path::PathBuf};

use iced_x86::{Formatter, NasmFormatter};
use pelite::pe64::PeFile;
use pelite::FileMap;

mod match_assembly;
mod transforms;
mod util;
mod vm_handler;
mod vm_matchers;
mod vm_disassemblers;

use clap::Parser;
use vm_handler::{VmContext, VmHandler};

use crate::util::handle_vm_call;

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

    let vm_context = VmContext::new(&pe_file, &pe_bytes, command_line_args.vm_call_address);
    println!("{:#x?}", vm_context);

    Ok(())
}
