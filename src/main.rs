extern crate procfs;
use procfs::CpuInfo;

fn main() {
    println!("Gathering CPU & Memory Info");
    let cpu = CpuInfo::new().unwrap();
    println!("Cores: {:?}", cpu.num_cores());
}
