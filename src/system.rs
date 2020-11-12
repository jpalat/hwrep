use procfs::CpuInfo;
use procfs::Meminfo;
use std::fs;
use std::io::Error;
use std::path::Path;

#[derive(Debug)]
pub struct CPU {
    pub model: String,
    pub physical_cores: usize,   // cores on chip
    pub execution_units: usize,  // execution units (physical_cores * # of threads/core)
    pub threads_per_core: usize, // threads
    pub sockets: usize,
}

impl CPU {
    pub fn new() -> CPU {
        let mut cmod = "No CPU Info Available".to_owned();
        let mut exu = 0;
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
            threads_per_core: siblings / socket_cores,
            sockets: exu / siblings,
        }
    }
}

pub fn get_memory() -> u64 {
    let memory = Meminfo::new().unwrap();
    return memory.mem_total;
}

pub fn get_numalayout() -> Result<Vec<String>, Error> {
    let sys_numa = "/sys/devices/system/node/";
    let mut new_vec: Vec<String> = Vec::new();
    let files = fs::read_dir(sys_numa);
    let mut filtered_files = Vec::new();
    match files {
        Err(e) => return Err(e),
        Ok(files) => {
            files
                .filter_map(Result::ok)
                .filter(|f| f.file_name().into_string().unwrap().starts_with("node"))
                .for_each(|f| (filtered_files.push(f.file_name())));
        }
    }
    for numa_node in filtered_files {
        let path = Path::new(sys_numa);
        let node_dir = Path::new(&numa_node);
        let node_layout = Path::new("cpulist");
        let new_path = path.join(node_dir).join(node_layout);
        let nodes: String;

        match new_path.to_str() {
            None => panic!("new path is not a valid UTF-8 sequence"),
            Some(s) => {
                let contents = fs::read_to_string(s);
                match contents {
                    Err(_e) => {
                        nodes = "-".to_string();
                        // println!("Error {}", e);
                    }

                    Ok(s) => {
                        let node_str = numa_node.into_string().unwrap();
                        nodes = format!("{} : {}", node_str, s.trim_end().to_string());
                    }
                }
            }
        }

        new_vec.push(nodes);
    }

    return Ok(new_vec);
}
