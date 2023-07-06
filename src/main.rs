use std::fs;

use args::Args;
use clap::Parser;
use rsrv::rv::{RV, RV_EECALL, RV_OK};

mod args;

fn main() {
    let args = Args::parse();

    let binary: Vec<u8> =
        fs::read(args.file.clone()).expect(&format!("Failed to load {}", args.file));

    let binary: Vec<u32> = binary
        .chunks_exact(4)
        .map(|chunk| u32::from_ne_bytes(chunk.try_into().unwrap()))
        .collect();

    let cpu = RV::new(args.mem, binary);

    loop {
        let state = RV::step(cpu);
        println!("(STATE) {}", state);
        match state {
            RV_EECALL => {
                println!(
                    "Environment call @ {:08x}: {}",
                    RV::get_pc(cpu),
                    RV::get_r(cpu)[17]
                );
                break;
            }
            RV_OK => {}
            _ => {
                break;
            }
        }
    }

    let pc = RV::get_pc(cpu);
    let csrs = RV::get_csrs(cpu);
    let registers = RV::get_r(cpu);

    println!("=====CPU DUMP=====");
    println!("(PC) 0x{:08x}", pc);

    println!("(REGISTER)");
    for x in 0..31 {
        println!("  ({:02}) 0x{:08x}", x, registers[x]);
    }

    println!("(CSR)");
    println!("  (mhartid)  0x{:08x}", csrs.mhartid);
    println!("  (mstatus)  0x{:08x}", csrs.mstatus);
    println!("  (mstatush) 0x{:08x}", csrs.mstatush);
    println!("  (mscratch) 0x{:08x}", csrs.mscratch);
    println!("  (mepc)     0x{:08x}", csrs.mepc);
    println!("  (mcause)   0x{:08x}", csrs.mcause);
    println!("  (mtval)    0x{:08x}", csrs.mtval);
    println!("  (mip)      0x{:08x}", csrs.mip);
    println!("  (mtinst)   0x{:08x}", csrs.mtinst);
    println!("  (mtval2)   0x{:08x}", csrs.mtval2);
    println!("  (mtvec)    0x{:08x}", csrs.mtvec);
    println!("  (mie)      0x{:08x}", csrs.mie);

    println!("==================");
}
