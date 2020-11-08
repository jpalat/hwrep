extern crate nix;

mod disks;
use disks::Disks;
mod system;
use system::CPU;
mod utils;
use utils::iec;

fn main() {
    println!("Gathering CPU Info");
    let cpu = CPU::new();
    println!("Model: {}", cpu.model);
    println!("Cores per Socket: {}", cpu.physical_cores);
    println!("Thread(s) per Core: {}", cpu.threads_per_core);
    println!("Total Cores: {:?}", cpu.execution_units);
    println!("Sockets {}", cpu.sockets);

    println!("Memory: {}", iec(system::get_memory()));

    let dlist = Disks::new();

    let headers = ["Filesystem", "Size", "Used", "Avail", "Use%", "Mounted on"];
    // let headers: Vec<ColoredString> = headers.iter().map(|x| x.yellow()).collect();
    println!(
        "{:width$} {:>5} {:>5} {:>5} {:>5} {}",
        headers[0],
        headers[1],
        headers[2],
        headers[3],
        headers[4],
        headers[5],
        width = dlist.max_width
    );

    for disk in dlist.disks {
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
            width = dlist.max_width
        );
    }
}
