use std::fs;
use std::io::Error;
use std::path::Path;

#[derive(Debug)]
pub struct Netinterface {
    pub name: String,
    pub speed: String,
}

impl Netinterface{
    pub fn new(entry: String, speed: String) -> Netinterface {
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
        let mut count = 0;
        for entry in fs::read_dir(sysnetwork)? {
            count += 1;
            let entry = entry?;

            let path = entry.path();
            // println!("entry: {:?}",entry );
            let file_name = path.file_name().unwrap();
            let speed_file = Path::new("speed");
            let new_path = path.join(speed_file);
            let speed:String;
            
            match new_path.to_str() {
                None => panic!("new path is not a valid UTF-8 sequence"),
                Some(s) => {
                    let contents = fs::read_to_string(new_path);
                    match contents {
                        Err(e) => {
                            speed = "-".to_string();
                            // println!("Error {}", e);
                        },
                        Ok(s) => speed = s,
                    }
                }
            }


            let contents = fs::read_to_string(file_name);
            let net_int = Netinterface::new(file_name.to_str().unwrap().to_string(), speed);
            new_vec.push(net_int);

        }
        let n = Networks {
            networks: new_vec,
        };

       return Ok(n);
    }
}
