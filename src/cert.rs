extern crate std;
extern crate hyper;
extern crate reqwest;

use std::io::Read;

pub fn install_cert() {
    std::thread::spawn(move || {
        if download_cert() {
            super::utils::send_notify("Certificate has been successfully installed.");
        }
    });
}

fn download_cert() -> bool {
    let mut response = reqwest::get(super::CERT_URL).unwrap();

    if !response.status().is_success() {
        super::utils::send_notify(format!(
            "Certificate could not be downloaded. Remote server returned {}.", response.status()
        ).as_str());

        return false;
    }

    let mut content = String::new();
    response.read_to_string(&mut content).expect("Unable to read response.");

    super::utils::install_cert(content.as_str());

    return true;
}
