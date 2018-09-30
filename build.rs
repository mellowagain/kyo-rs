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

extern crate base64;
extern crate html_minifier;

use std::io::Read;

fn main() {
    let css = "<style>".to_owned() + include_str!("resources/index.css") + "</style>";
    let js = "<script>".to_owned() + include_str!("resources/index.js") + "</script>";
    let background_jpg = "data:image/jpeg;base64,".to_owned() + include_str!("resources/img/background.jpg.base64");
    let button_svg = "data:image/svg+xml;base64,".to_owned() + &base64::encode(include_str!("resources/img/button.svg"));

    let fixed_css = &str::replace(include_str!("resources/index.html"), r#"<link rel="stylesheet" href="index.css">"#, css.as_str());
    let fixed_js = &str::replace(fixed_css, r#"<script src="index.js"></script>"#, js.as_str());
    let fixed_bg = &str::replace(fixed_js, r#"img/background.jpg"#, background_jpg.as_str());
    let fixed_btn = &str::replace(fixed_bg, r#"img/button.svg"#, button_svg.as_str());

    let mut minifier = html_minifier::HTMLMinifier::new();
    minifier.digest(fixed_btn).unwrap();

    let html: String = minifier.get_html().chars().skip(5).collect();

    std::fs::write("resources/index.include.html", html);
}
