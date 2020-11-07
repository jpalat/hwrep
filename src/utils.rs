
 // http://stackoverflow.com/questions/5194057/better-way-to-convert-file-sizes-in-python
pub fn iec(n: u64) -> String {
    let units = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];

    let i = (n as f64).log(1024_f64).floor() as u32;
    let p = 1024_u64.pow(i);
    let s = (n as f64) / (p as f64);
    format!("{:.0}{}", s, units[i as usize])
}