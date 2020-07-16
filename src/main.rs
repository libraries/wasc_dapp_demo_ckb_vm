use ckb_vm::machine::SupportMachine;

mod cost_model;
mod syscall;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<bytes::Bytes> = std::env::args().map(|a| a.into()).collect();
    let data = bytes::Bytes::from(std::fs::read(std::str::from_utf8(&args[1])?)?);

    let core_machine =
        ckb_vm::DefaultCoreMachine::<u64, ckb_vm::FlatMemory<u64>>::new_with_max_cycles(1 << 32);
    let mut machine = ckb_vm::DefaultMachineBuilder::<
        ckb_vm::DefaultCoreMachine<u64, ckb_vm::FlatMemory<u64>>,
    >::new(core_machine)
    .instruction_cycle_func(Box::new(cost_model::instruction_cycles))
    .syscall(Box::new(syscall::SyscallDebug::new()))
    .build();

    machine.load_program(&data, &args[1..])?;
    let exit = machine.run()?;
    let cycles = machine.cycles();

    println!("exit={:?} cycles={:?}", exit, cycles);
    Ok(())
}
