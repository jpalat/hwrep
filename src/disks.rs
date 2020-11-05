#[derive(Debug)]

pub struct Disks {
    pub filesystem: String,
    pub size: u64,
    pub used: u64,
    pub avail: u64,
    pub percent: f64,
    pub mount: String,
} 

impl Disks {
    pub fn new(filesystem: &str, size: u64, avail: u64, mount: &str) -> Disks {
        let used = size - avail;
        let percent = used as f64 / size as f64;
        Disks {
            filesystem: filesystem.to_string(),
            size,
            used: used,
            mount: mount.to_string(),
            percent: percent,
            avail,
        }
    }
}  