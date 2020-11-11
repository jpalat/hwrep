use std::fs;
use std::io::Error;

#[derive(Debug)]
pub struct Netinterface {
    pub name: String,
    pub speed: usize,
}

impl Netinterface{
    pub fn new(entry: String, speed: usize) -> Netinterface {
        Netinterface{
            name: entry,
            speed,
        }
    }
}

pub struct Networks {
    pub networks: Vec<Netinterface>
}

impl Networks {
    pub fn new() -> Result<Networks, Error> {
        let sysnetwork = "/sys/class/net";
        let mut new_vec : Vec<Netinterface> = Vec::new();
        for entry in fs::read_dir(sysnetwork)? {
            let entry = entry?;

            let path = entry.path();
            // println!("entry: {:?}",entry );
            let file_name = path.file_name().unwrap();
            let net_int = Netinterface::new(file_name.to_str().unwrap().to_string(), 100);
            new_vec.push(net_int);

        }
        let n = Networks {
            networks: new_vec,
        };

       return Ok(n);
    }
}
