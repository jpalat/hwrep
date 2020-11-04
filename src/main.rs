extern crate procfs;
use procfs::CpuInfo;
use procfs::Meminfo;

use num_format::{Locale, ToFormattedString};

fn main() {
    println!("Gathering CPU Info");
    let cpu = CpuInfo::new() ;
    let model;
    let mut cores = 1;
    

    match cpu {
        Err(_) => println!("No CPU available"),
        Ok(cpu) =>{
            match cpu.model_name(0) {
                None => println!("No CPU model info available"),
                Some(model_id) => {
                    model = model_id;
                    println!("Model: {}", model);
                }
            }
            match cpu.get_info(0) {
                None => println!("No additional info available."),
                Some(details) => {
                    // println!("{:#?}", details);
                    let cpuc = details.get(&"cpu cores").unwrap();
                    cores = cpuc.parse().unwrap_or(0);
                    println!("Cores / CPU: {}", cores);
                }
            }
            let total_cores = cpu.num_cores();
            
            println!("Total Cores: {:?}", total_cores);
            println!("Sockets: {}", total_cores/cores);
        }

    }
    let memory = Meminfo::new().unwrap();
    let b = memory.mem_total;
    let kb = b / 1024;
    let mb = kb / 1024;
    let gb = mb / 1024;

    println!("\nMemory in KiB: {} MiB: {} GiB: {}", kb.to_formatted_string(&Locale::en), mb.to_formatted_string(&Locale::en), gb);
    

}
