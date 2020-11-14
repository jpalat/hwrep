use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::Path;

use crate::DisplayWidth;

pub struct Manufacturer {
    pub data: HashMap<String, String>,
}

impl Manufacturer {
    pub fn new() -> Result<Manufacturer, Error> {
        let mut newMan = HashMap::new();
        let base = "/sys/class/dmi/id/";
        let files: Vec<_> = fs::read_dir(base)?
            .into_iter()
            .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
            .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
            .filter(|r| r.is_file()) // Filter out non-folders
            .collect();

        for path in files {
            // let path = entry.path();
            let file_name = path.file_name().unwrap();
            let info: String;
            // println!("path: {:?}", path);
            match path.to_str() {
                None => panic!("new path is not a valid UTF-8 sequence"),
                Some(s) => {
                    let contents = fs::read_to_string(s);
                    match contents {
                        Err(_e) => {
                            info = "-".to_string();
                            // println!("Error {}", e);
                        }
                        Ok(s) => info = s.trim_end().to_string(),
                    }
                }
            }
            newMan.insert(file_name.to_str().unwrap().to_string(), info);
        }

        return Ok(Manufacturer { data: newMan });
    }
}

impl DisplayWidth for Manufacturer {
    fn get_max(&self) -> usize {
        let mut max_width = 0;
        for (topic, detail) in &self.data {
            max_width = cmp::max(max_width, topic.len());
        }
        return max_width;
    }
}
