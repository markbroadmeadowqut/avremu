//use std::env;
use std::path::PathBuf;

use clap::Parser;

mod devices;
mod cores;
mod memory;
mod peripherals;
mod nets;
mod hardware;
mod boards;
mod events;

use crate::boards::quty::QUTy;
use crate::events::Event;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // Firmware file in Intel HEX format
    #[clap(short, long, value_parser, value_name = "HEX FILE")]
    firmware: PathBuf,

    // Events file (plain text)
    #[clap(short, long, value_parser, value_name = "FILE")]
    events: Option<PathBuf>,

    // Cycle limit
    #[clap(short, long, value_parser)]
    cycles: Option<u64>,

    // Dump registers flag
    #[clap(short, long, action)]
    registers: bool,

    // Dump stack flag
    #[clap(short, long, action)]
    stack: bool,

    /// Debug flag
    #[clap(short, long, action)]
    debug: bool
}

fn main() {

    let cli = Cli::parse();

    //let args: Vec<String> = env::args().collect();
    
    //let filename_firmware = &args[1];
    //let filename_events = &args[2];
    //let cycle_limit: u64 = args[3].parse().unwrap();

    let events = Event::from_file(filename_events);

    println!("[FIRMWARE] {}.", cli.firmware.display());
    println!("[EVENTS] {}: Parsed {} events.", &filename_events, events.len());

    let mut quty = QUTy::new();
    quty.events(events);
    quty.mcu_programme(&cli.firmware.to_string_lossy().to_string());
    
    if args.len() > 4 {
        if args[4].eq("debug") {
            quty.core_debug();
        }
    }

    let mut cycles = 0u64;

    println!("[RUN] Cycle limit is {}.", cycle_limit);

    while quty.step() {
         //Run until break
         cycles += 1;
         if cycles == cycle_limit {
            println!("[END] Cycle limit elapsed.");
            break;
         }
    }

    println!("[INFO] Programme terminated after {} cycles.", cycles);

    quty.mcu_dumpstack();
    quty.core_dumpregs();
    
}
