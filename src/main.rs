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
static MIRROR_IP: &'static str = r#"209.97.182.162"#; // Won't be displayed to user but put in hosts regardless
static CERT_URL: &'static str = r#"https://shiro.host/cert.pem"#;
static RESULT_CERT_NAME: &'static str = r#"shiro.crt"#; // Always needs to end in .crt
static CONTENT: &'static str = include_str!("../resources/index.include.html");

fn main() {
    let user_data = ();

    if !utils::is_root() {
        utils::send_notify("Please run this program with elevated permissions. (Administrator or root)");
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
                web_view.dispatch(|web_view, _user_data| {
                    web_view.eval(&format!(
                        "document.getElementById('connect-address').value = '{}';",
                        SHIRO_IP
                    ));
                });
            });
        },
        move |web_view, args, _user_data| {
            let json: serde_json::Value = serde_json::from_str(args).unwrap();
            let cmd = json["cmd"].as_str().unwrap();
            let address = json["address"].as_str().unwrap_or_default();

            match cmd {
                "connect" => {
                    web_view.eval(if hosts::overwrite(address) { "toggleConnectButton();" } else { "displayError();" });
                }
                "disconnect" => {
                    web_view.eval(if hosts::revert() { "toggleConnectButton();" } else { "displayError();" });
                }
                "install" => {
                    cert::install_cert();
                }
                _ => unimplemented!()
            }
        },
        user_data
    );
}
