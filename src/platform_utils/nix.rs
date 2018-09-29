extern crate libc;

pub fn is_root() -> bool {
    /*unsafe {
        libc::getuid() == 0
    }*/
    true
}

pub fn msg_box(msg: &str) {
    println!("{}", str::replace(msg, "administrator", "root"));
}
