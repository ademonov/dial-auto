use std::path::Path;
use std::{env, fs};

const ICON_ON: &str = "on.ico";
const ICON_OFF: &str = "off.ico";

fn main() {
    let target_dir_path = env::var("OUT_DIR").unwrap();
    copy(&target_dir_path, ICON_ON);
    copy(&target_dir_path, ICON_OFF);
}

fn copy<S: AsRef<std::ffi::OsStr> + ?Sized, P: Copy + AsRef<Path>>(target_dir_path: &S, file_name: P) {
    let dst_path = Path::new(&target_dir_path).join("../../..").join(file_name);
    fs::copy(file_name, dst_path).unwrap();
}