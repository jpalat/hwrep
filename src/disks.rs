use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

use nix::sys::statvfs::statvfs;


const FS_SPEC: usize = 0;
const FS_FILE: usize = 1;

#[derive(Debug)]
pub struct Disk {
    pub filesystem: String,
    pub size: u64,
    pub used: u64,
    pub avail: u64,
    pub percent: f64,
    pub mount: String,
} 

impl Disk {
    pub fn new(filesystem: &str, size: u64, avail: u64, mount: &str) -> Disk {
        let used = size - avail;
        let percent = used as f64 / size as f64;
        Disk {
            filesystem: filesystem.to_string(),
            size,
            used: used,
            mount: mount.to_string(),
            percent: percent,
            avail,
        }
    }
}  

pub struct Disks {
    pub disks: Vec<Disk>,
    pub max_width: usize,
}

impl Disks {
    pub fn new() -> Disks{
        println!("\nDisk Info");
        let file = match File::open("/proc/mounts") {
            Ok(f) => f,
            Err(e) => {
                println!("Error 1: Could not open /proc/mounts - {}", e);
                process::exit(1);
            }
        };
        let reader = BufReader::new(&file);
        let mut disk_list: Vec<Disk> = Vec::new();
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
    
                    let d = Disk::new(fields[FS_SPEC], size, avail, fields[FS_FILE]);
                    max_width = cmp::max(max_width, d.filesystem.len());
    
                    disk_list.push(d);
                }
                Err(err) => println!("Error 3: {}", err),
            }
        }


        Disks {
            disks: disk_list,
            max_width,
        }
    }
}