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

static SHIRO_IP: &'static str = r#"209.97.182.162"#;
static MIRROR_IP: &'static str = r#"209.97.182.162"#;
static CERT_URL: &'static str = r#"https://shiro.host/cert.pem"#;
static CONTENT: &'static str = include_str!("../resources/index.include.html");

fn main() {
    let init_cb = |_webview| {};
    let user_data = ();

    web_view::run(
        "kyo-rs",
        web_view::Content::Html(CONTENT),
        Some((400, 260)),
        false,
        true,
        init_cb,
        move |web_view, args, user_data| {
            let json: serde_json::Value = serde_json::from_str(args).unwrap();
            let cmd: &str = json["cmd"].as_str().unwrap();

            match cmd {
                "connect" => {
                    println!("Connect invoked with {}", json["address"].as_str().unwrap());
                }
                "disconnect" => {
                    println!("Disconnect invoked")
                }
                "install" => {
                    println!("Install invoked")
                }
                _ => unimplemented!()
            }
        },
        user_data
    );
}
