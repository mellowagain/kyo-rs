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

extern crate web_view;
extern crate serde_json;

mod cert;
mod hosts;
mod platform_utils;

#[cfg(windows)]
use platform_utils::win32 as utils;
#[cfg(unix)]
use platform_utils::nix as utils;

static SHIRO_IP: &'static str = r#"209.97.182.162"#;
static MIRROR_IP: &'static str = r#"209.97.182.162"#;
static CERT_URL: &'static str = r#"https://shiro.host/cert.pem"#;
static CONTENT: &'static str = include_str!("../resources/index.include.html");

fn main() {
    let user_data = ();

    if !utils::is_root() {
        utils::msg_box("This program needs to be run as administrator.");
        std::process::exit(1);
    }

    web_view::run(
        "kyo-rs",
        web_view::Content::Html(CONTENT),
        Some((400, 260)),
        false,
        true,
        move |web_view| {
            std::thread::spawn(move || {
                web_view.dispatch(|web_view, user_data| {
                    web_view.eval(&format!(
                        "document.getElementById('connect-address').value = '{}';",
                        SHIRO_IP
                    ));
                });
            });
        },
        move |web_view, args, user_data| {
            let json: serde_json::Value = serde_json::from_str(args).unwrap();
            let cmd: &str = json["cmd"].as_str().unwrap();

            match cmd {
                "connect" => {
                    println!("Connect invoked with {}", json["address"].as_str().unwrap());
                }
                "disconnect" => {
                    println!("Disconnect invoked");
                }
                "install" => {
                    println!("Installing certificate in trusted root authorities.");
                    cert::install_cert();
                }
                _ => unimplemented!()
            }
        },
        user_data
    );
}
