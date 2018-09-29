extern crate std;
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

pub fn install_cert(cert: &str) {
    let fmt_path = format!(
        "/etc/ca-certificates/trust-source/anchors/{}", super::RESULT_CERT_NAME
    );

    let result_path = std::path::Path::new(&fmt_path);

    if result_path.exists() {
        // Certificate was already installed. Abort.
        return;
    }

    std::fs::write(result_path, cert).expect("Unable to write certificate to disk.");

    let status_code = std::process::Command::new("trust")
        .arg("extract-compat")
        .status()
        .expect("Unable to execute trust extract-compat command.");

    if status_code.code().unwrap() != 0 {
        msg_box(format!(
            "Unable to install certificate, process exited with exit code {}.", status_code.code().unwrap()
        ).as_str());
    }
}
