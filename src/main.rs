extern crate rv32im_sim;

use std::env;
use std::process;
use rv32im_sim::elfload;
use rv32im_sim::Simulator;
use rv32im_sim::cpu::CPU;
use rv32im_sim::ExeOption;
use rv32im_sim::Arguments;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = Arguments::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("In file {}", args.filename);

    let loader = match elfload::ElfLoader::try_new(&args.filename) {
        Ok(loader) => loader,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    if loader.is_elf() {
        println!("elfcheck: OK");

        let simulator: Simulator = Simulator {
            loader: loader,
            cpu: CPU {
                pc: 0 as u32,
                reg: [0; 32],
            },
        };

        match args.exe_option {
            ExeOption::OPT_NONE     => simulator.loader.dump_section(),
            ExeOption::OPT_ELFHEAD  => simulator.loader.ident_show(),
            ExeOption::OPT_PROG     => simulator.loader.dump_segment(),
            ExeOption::OPT_SECT     => simulator.loader.dump_section(),
            ExeOption::OPT_SHOWALL  => simulator.loader.show_all_header(),
            ExeOption::OPT_DISASEM  => simulator.loader.ident_show(),
        }
    } else {
        panic!("This file is not an ELF.");
    }
}
