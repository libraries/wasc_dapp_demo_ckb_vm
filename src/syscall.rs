use std::cell::RefCell;
use std::rc::Rc;

use ckb_vm::instructions::Register;

pub fn get_arr<Mac: ckb_vm::SupportMachine>(
    machine: &mut Mac,
    addr: usize,
    size: usize,
) -> Result<Vec<u8>, ckb_vm::Error> {
    let mut addr = addr;
    let mut buffer = Vec::new();
    for _ in 0..size {
        let byte = machine.memory_mut().load8(&Mac::REG::from_u64(addr as u64))?.to_u8();
        buffer.push(byte);
        addr += 1;
    }
    machine.add_cycles(buffer.len() as u64 * 10)?;
    Ok(buffer)
}

pub struct SyscallDebug {}

impl SyscallDebug {
    pub fn new() -> Self {
        Self {}
    }
}

impl<Mac: ckb_vm::SupportMachine> ckb_vm::Syscalls<Mac> for SyscallGetAssetBalance {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::Error> {
        let code = &machine.registers()[ckb_vm::registers::A7];
        if code.to_i64() != 2100 {
            return Ok(false);
        }
        let s_ptr = machine.registers()[ckb_vm::registers::A0].to_u64() as usize;
        let s_len = machine.registers()[ckb_vm::registers::A1].to_u64() as usize;
        let s = std::str::from_utf8(get_arr(machine, s_ptr, s_len)).unwrap();
        println!("{:?}", s);
        machine.set_register(ckb_vm::registers::A0, Mac::REG::from_i64(0));
        Ok(true)
    }
}
