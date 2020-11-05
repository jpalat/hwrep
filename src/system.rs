use procfs::CpuInfo;

#[derive(Debug)]
pub struct CPU {
    model: String,
    logical_cores: usize,
    physical_cores: usize,
    sockets: usize,
}

impl CPU {
    pub fn new () -> CPU {
        
        let mut cmod = "No CPU Info Available".to_owned();;
        let mut lcore = 1;
        let mut sock = 1;

        let cpu = CpuInfo::new();

        match cpu {
            Err(_) => println!("No CPU available"),
            Ok(cpu) =>{
                match cpu.model_name(0) {
                    None => println!("No CPU model info available"),
                    Some(model_id) => {
                        cmod = model_id.to_string();
                        println!("Model: {}", cmod);
                    }
                }
                match cpu.get_info(0) {
                    None => println!("No additional info available."),
                    Some(details) => {
                        println!("{:#?}", details);
                        let cpuc = details.get(&"cpu cores").unwrap();
                        lcore = cpuc.parse().unwrap_or(0);
                        println!("Cores / CPU: {}", lcore);
                    }
                }
                let total_cores = cpu.num_cores();
                
                println!("Total Cores: {:?}", total_cores);
                println!("Sockets: {}", total_cores/lcore);
            }
    
        }
        CPU {
            model: cmod.to_string(),
            logical_cores: lcore,
            physical_cores: lcore /2,
            sockets: 1,
        }

    }
}