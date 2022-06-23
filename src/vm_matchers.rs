use iced_x86::Code;

use crate::{
    util::check_full_reg_written,
    vm_handler::{VmHandler, VmRegisterAllocation},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandlerClass {
    ByteOperand,
    WordOperand,
    DwordOperand,
    QwordOperand,
    NoOperand,
    UnconditionalBranch,
    VmExit,
}

impl VmHandler {
    pub fn match_handler_class(&self,
                               reg_allocation: &VmRegisterAllocation)
                               -> HandlerClass {
        let instruction_iter = self.instructions.iter();

        if instruction_iter.clone()
                           .filter(|insn| check_full_reg_written(insn, reg_allocation.vip.into()))
                           .find(|insn| insn.code() == Code::Mov_r64_rm64)
                           .is_some()
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
            &[] => return HandlerClass::VmExit,
            &[4] => return HandlerClass::NoOperand,
            &[8, 4] => return HandlerClass::QwordOperand,
            &[4, 4] => return HandlerClass::DwordOperand,
            &[2, 4] => return HandlerClass::WordOperand,
            &[1, 4] => return HandlerClass::ByteOperand,
            slice => {
                panic!("Unimplemented handler class with slice {:?}", slice)
            },
        }
    }
}
