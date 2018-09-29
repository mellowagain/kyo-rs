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
extern crate web_view;

use std::io::Read;
use std::io::Write;
use std::io::BufRead;

#[cfg(windows)]
static HOSTS_PATH: &'static str = r#"C:\Windows\System32\drivers\etc\hosts"#;
#[cfg(windows)]
static NEW_LINE: &'static str = "\r\n";

#[cfg(unix)]
static HOSTS_PATH: &'static str = r#"/etc/hosts"#;
#[cfg(unix)]
static NEW_LINE: &'static str = "\n";

pub fn overwrite(address: &str) -> bool {
    let mut success = false;

    std::thread::spawn(move || {
        let mut hosts = std::fs::File::create(HOSTS_PATH).expect("Hosts file does not exist.");
        let mut content = String::new();

        hosts.read_to_string(&mut content).expect("Unable to read hosts file.");

        let osu = format!("{} osu.ppy.sh", super::SHIRO_IP);
        let c = format!("{} c.ppy.sh", super::SHIRO_IP);
        let c1 = format!("{} c1.ppy.sh", super::SHIRO_IP);
        let a = format!("{} a.ppy.sh", super::SHIRO_IP);
        let i = format!("{} i.ppy.sh", super::SHIRO_IP);
        let bm6 = format!("{} bm6.ppy.sh", super::MIRROR_IP);

        let mut lines: Vec<&str> = content.split(NEW_LINE).collect();

        for i in 0..lines.len() {
            let mut line = lines[i];

            if !line.starts_with("#") && line.contains("ppy.sh") {
                //lines[i] = format!("#{}", line).as_str();
                lines.remove(i);
            }
        }

        lines.push("# shiro - added by kyo-rs server switcher");
        lines.push(osu.as_str()); // String
        lines.push(c.as_str());
        lines.push(c1.as_str());
        lines.push(a.as_str());
        lines.push(i.as_str());
        lines.push(bm6.as_str());

        // Clear the file, making it empty
        hosts.set_len(0);

        let mut writer = std::fs::OpenOptions::new()
            .append(true)
            .open(HOSTS_PATH)
            .expect("Hosts file does not exist.");

        for line in lines {
            writer.write_all(format!("{}{}", line, NEW_LINE).as_bytes());
        }

        writer.flush().unwrap();

        success = true;
    });

    return success;
}

pub fn revert() -> bool {
    std::thread::spawn(move || {

    });

    return false;
}

pub fn is_connected() -> bool {
    let mut file = std::fs::File::open(HOSTS_PATH).unwrap();

    for content in std::io::BufReader::new(file).lines() {
        let unwrapped = content.unwrap();
        let line = unwrapped.as_str();

        if line.starts_with("#") || !line.contains("ppy.sh") {
            continue;
        }

        if line.contains(super::SHIRO_IP) || line.contains(super::MIRROR_IP) {
            return true;
        }
    }

    return false;
}
