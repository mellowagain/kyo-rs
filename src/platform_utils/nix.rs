/*
 * kyo-rs - Rust rewrite of kyo, a modern osu! server switcher
 * Copyright (C) 2018 Marc3842h
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

extern crate std;
extern crate libc;
extern crate notify_rust;

pub fn is_root() -> bool {
    unsafe {
        libc::getuid() == 0
    }
}

pub fn install_cert(cert: &str) {
    let fmt_path = format!(
        "/etc/ca-certificates/trust-source/anchors/{}", super::RESULT_CERT_NAME
    );

    let result_path = std::path::Path::new(&fmt_path);

    if result_path.exists() {
        send_notify("The certificate has already been installed.");
        return;
    }

    std::fs::write(result_path, cert).expect("Unable to write certificate to disk.");

    let status_code = std::process::Command::new("trust")
        .arg("extract-compat")
        .status()
        .expect("Unable to execute trust extract-compat command.");

    if status_code.code().unwrap() != 0 {
        send_notify(format!(
            "Trust store could not be refreshed. Trust exited with exit code {}.", status_code.code().unwrap()
        ).as_str());
    }
}

pub fn send_notify(msg: &str) {
    notify_rust::Notification::new()
        .appname("kyo-rs")
        .summary("kyo-rs")
        .body(msg)
        .auto_icon()
        .show()
        .unwrap();
}
