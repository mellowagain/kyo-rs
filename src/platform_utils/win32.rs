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

pub fn is_root() -> bool {
    unsafe {
        user32::IsUserAnAdmin() // This doesn't exist
    }
}

pub fn send_notify(msg: &str) {
    use winrt::*;
    use winrt::windows::data::xml::dom::*;
    use winrt::windows::ui::notifications::*;

    unsafe {
        let xml = ToastNotificationManager::get_template_content(ToastTemplateType_ToastText02).unwrap();
        let elements = xml.get_elements_by_tag_name(&winrt::FastHString::new("text")).unwrap();

        elements.item(0).unwrap().append_child(&*xml.create_text_node(&FastHString::from("kyo-rs")).unwrap().query_interface::<IXmlNode>().unwrap()).unwrap();
        elements.item(1).unwrap().append_child(&*xml.create_text_node(&FastHString::from(msg)).unwrap().query_interface::<IXmlNode>().unwrap()).unwrap();

        let toast = ToastNotification::create_toast_notification(&*xml).unwrap();
        ToastNotificationManager::create_toast_notifier_with_id(
            &winrt::FastHString::new(r#"{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe"#)
        ).unwrap().show(&*toast).unwrap();
    }
}
