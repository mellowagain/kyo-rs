extern crate std;
extern crate hyper;
extern crate reqwest;

use std::io::Read;

pub fn install_cert() {
    std::thread::spawn(move || {
        download_cert();

        println!("Successfully installed certificate.");
    });
}

fn download_cert() {
    let mut response = reqwest::get(super::CERT_URL).unwrap();

    if !response.status().is_success() {
        super::utils::msg_box(&format!(
            "Unable to download certificate, server returned {}.", response.status()
        ));
        return;
    }

    let mut content = String::new();
    response.read_to_string(&mut content).expect("Unable to read response.");

    super::utils::install_cert(content.as_str());
}
