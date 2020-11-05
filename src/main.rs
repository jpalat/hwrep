extern crate nix;
extern crate procfs;

use nix::sys::statvfs::statvfs;
use procfs::CpuInfo;
use procfs::Meminfo;
use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

mod disks;
use disks::Disks;

use num_format::{Locale, ToFormattedString};

const FS_SPEC: usize = 0;
const FS_FILE: usize = 1;

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
    
    println!("\nDisk Info");
    let file = match File::open("/proc/mounts") {
        Ok(f) => f,
        Err(e) => {
            println!("Error 1: Could not open /proc/mounts - {}", e);
            process::exit(1);
        }
    };
    let reader = BufReader::new(&file);
    let mut disks: Vec<Disks> = Vec::new();

    let mut max_width = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split_whitespace().collect();
                // println!("{:#?}", fields);
                let statvfs = match statvfs(fields[FS_FILE]) {
                    Ok(s) => s,
                    Err(_err) => {
                        // println!("Error 2: {}", err);
                        continue;
                    }
                };
                let size = statvfs.blocks() * statvfs.block_size();
                let avail = statvfs.blocks_available() * statvfs.block_size();
                if size == 0 {
                    continue;
                }

                let d = Disks::new(fields[FS_SPEC], size, avail, fields[FS_FILE]);
                max_width = cmp::max(max_width, d.filesystem.len());

                disks.push(d);
            }
            Err(err) => println!("Error 3: {}", err),
        }
    }

    let headers = [
        "Filesystem",
        "Size",
        "Used",
        "Avail",
        "Use%",
        "Mounted on",
    ];
    // let headers: Vec<ColoredString> = headers.iter().map(|x| x.yellow()).collect();
    println!(
        "{:width$} {:>5} {:>5} {:>5} {:>5} {}",
        headers[0],
        headers[1],
        headers[2],
        headers[3],
        headers[4],
        headers[5],
        width = max_width
    );

    for disk in disks {
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
            width = max_width
        );
    }

    

// http://stackoverflow.com/questions/5194057/better-way-to-convert-file-sizes-in-python


}


pub fn iec(n: u64) -> String {
    let units = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];

    let i = (n as f64).log(1024_f64).floor() as u32;
    let p = 1024_u64.pow(i);
    let s = (n as f64) / (p as f64);
    format!("{:.0}{}", s, units[i as usize])
}