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
extern crate user32;
extern crate winapi;
extern crate winrt;
extern crate winrt_notification;

use winapi::um::wincrypt::*;
use winrt_notification::{Duration, Sound, Toast};

pub fn is_root() -> bool {
    /*unsafe {
        winapi::vc::shell::IsUserAnAdmin()
    }*/

    // The current method above is not implemented in the winapi crate
    // Workaround this by *requiring* the program to be started
    // with admin permissions by specifying it in the manifest.
    true
}

pub fn install_cert(cert: &str) {
    unsafe {
        let cert_store: HCERTSTORE = CertOpenSystemStoreA(0, "ROOT");

        CertAddEncodedCertificateToStore(
            cert_store,
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            cert,
            cert.len(),
            CERT_STORE_ADD_USE_EXISTING
        );

        CertCloseStore(cert_store, 0);
    }
}

pub fn send_notify(msg: &str) {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("kyo-rs")
        .text1(msg)
        .duration(Duration::Short)
        .show()
        .unwrap();
}
