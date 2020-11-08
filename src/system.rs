use procfs::CpuInfo;
use procfs::Meminfo;

#[derive(Debug)]
pub struct CPU {
    pub model: String,
    pub physical_cores: usize,  // cores on chip
    pub execution_units: usize,   // execution units (physical_cores * # of threads/core)
    pub threads_per_core: usize,  // threads
    pub sockets: usize,
}

impl CPU {
    pub fn new() -> CPU {
        let mut cmod = "No CPU Info Available".to_owned();
        let mut exu = 0;
        let sock =1;
        let mut socket_cores = 0;
        let mut siblings = 1;

        let cpu = CpuInfo::new();

        match cpu {
            Err(_) => println!("No CPU available"),
            Ok(cpu) => {
                match cpu.model_name(0) {
                    None => println!("No CPU model info available"),
                    Some(model_id) => {
                        cmod = model_id.to_string();
                    }
                }
                match cpu.get_info(0) {
                    None => println!("No additional info available."),
                    Some(details) => {
                        // println!("{:#?}", details);
                        // physical cores
                        let physical_cores = details.get(&"cpu cores").unwrap();
                        socket_cores = physical_cores.parse().unwrap_or(0);
                        // execution units with hyperthreading
                        let exec_cores = details.get(&"siblings").unwrap();
                        siblings = exec_cores.parse().unwrap_or(0);
                    }
                }
                exu = cpu.num_cores();
            }
        }
        CPU {
            model: cmod.to_string(),
            execution_units: exu,
            physical_cores: socket_cores,
            threads_per_core: siblings/socket_cores,
            sockets: exu/siblings,
        }
    }
}

pub fn get_memory() -> u64 {
    let memory = Meminfo::new().unwrap();
    return memory.mem_total;
}
