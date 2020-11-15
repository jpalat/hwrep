extern crate colored;
extern crate nix;

mod cliopts;
mod disks;
mod manufacturer;
mod network;
mod system;
mod utils;

use crate::utils::DisplayWidth;
use clap::Clap;
use cliopts::Opts;
use colored::*;
use disks::Disks;
use manufacturer::Manufacturer;
use network::Networks;
use nix::unistd;
use std::cmp;
use system::Memory;
use system::CPU;
use utils::iec;

fn main() {
    let opts: Opts = Opts::parse();

    match opts.json {
        0 => display(),
        3 | _ => build_json(),
    }
}

fn display() {
    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed getting hostname");
    let hostname = hostname_cstr.to_str().expect("Hostname wasn't valid UTF-8");
    println!("Hostname: {}\n", hostname.blue());
    println!("{}", "CPU Info".yellow());
    let cpu = CPU::new();
    println!("Model: {}", cpu.model);
    println!("Cores per Socket: {}", cpu.physical_cores);
    println!("Thread(s) per Core: {}", cpu.threads_per_core);
    println!("Total Cores: {:?}", cpu.execution_units);
    println!("Sockets: {}", cpu.sockets);

    let memory = Memory::new();
    println!("\n{}", "Memory".yellow());
    println!("{:10}: {}", "Total Ram", iec(memory.mem_total));
    println!("{:10}: {}", "Total Swap", iec(memory.swap_total));

    if memory.numa_layout.len() == 0 {
        println!("No NUMA info available.");
    } else {
        println!("\nNUMA layout");
        for n in memory.numa_layout {
            println!("{}", n);
        }
    }

    let networks = Networks::new();

    match networks {
        Err(e) => println!("error : {}", e),
        Ok(n) => {
            let width = cmp::max(n.get_max(), 10);
            println!(
                "\n{:width$} {:>5}",
                "Interface".yellow(),
                "Speed".yellow(),
                width = width
            );
            for network in n.networks {
                println!(
                    "{:width$} {:>5}",
                    network.name,
                    network.speed,
                    width = width
                );
            }
        }
    }

    let dlist = Disks::new();

    let headers = ["Filesystem", "Size", "Used", "Avail", "Use%", "Mounted on"];
    let headers: Vec<ColoredString> = headers.iter().map(|x| x.yellow()).collect();
    println!(
        "\n{:width$} {:>5} {:>5} {:>5} {:>5} {}",
        headers[0],
        headers[1],
        headers[2],
        headers[3],
        headers[4],
        headers[5],
        width = dlist.get_max()
    );

    for disk in &dlist.disks {
        // let fs = if stat.is_network() {
        //     disks.filesystem.cyan()
        // } else {
        //     stat.filesystem.normal()
        // };
        let percent = if disk.percent.is_nan() {
            "    -".to_string()
        } else {
            format!("{:>5.1}", disk.percent)
        };
        println!(
            "{:width$} {:>5} {:>5} {:>5} {} {}",
            disk.filesystem,
            iec(disk.size),
            iec(disk.used),
            iec(disk.avail),
            percent,
            disk.mount,
            width = dlist.get_max()
        );
    }
    println!("\n{}", "Id info".yellow());
    match Manufacturer::new() {
        Err(e) => println!("Unable to get info. {:?}", e),
        Ok(m) => {
            let width = m.get_max();
            for (topic, detail) in m.data {
                println!("{:width$} : {} ", topic, detail, width = width);
            }
        }
    }
    
}

fn build_json() {
    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed getting hostname");
    let hostname = hostname_cstr.to_str().expect("Hostname wasn't valid UTF-8");
    let json_hn = format!("{{\"hostname\":\"{}\"}}", hostname);
    let dlist = Disks::new();
    let networks = Networks::new().unwrap();
    let manu = Manufacturer::new().unwrap();
    let cpu = CPU::new();
    let memory = Memory::new();

    let json_disk = serde_json::to_string(&dlist).unwrap();
    let json_network = serde_json::to_string(&networks).unwrap();
    let json_manu = serde_json::to_string(&manu).unwrap();
    let json_cpu = serde_json::to_string(&cpu).unwrap();
    let json_memory = serde_json::to_string(&memory).unwrap();

    // let system = [json_disk, json_network, json_manu];
    // let serialized_disks = serde_json::to_string(&dlist).unwrap();
    let system_json = format!("{{
        \"hostname\": \"{}\",
        \"cpu_info\": {}, 
        \"memory_info\": {}, 
        \"disk_info\":{},
        \"networks\": {},
        \"manufacturer\": {}
    }}", hostname, json_cpu, json_memory, json_disk, json_network, json_manu);
    println!("{}", system_json);
}
