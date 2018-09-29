extern crate std;
extern crate user32;
extern crate winapi;

pub fn is_root() -> bool {
    unsafe {
        user32::IsUserAnAdmin() // This doesn't exist
    }
}

pub fn msg_box(msg: &str) {
    let content = std::ffi::CString::new(msg).unwrap();
    let title = std::ffi::CString::new("kyo-rs").unwrap();

    unsafe {
        user32::MessageBoxA(
            std::ptr::null_mut(),
            content.as_ptr(),
            title.as_ptr(),
            winapi::winuser::MB_OK | winapi::winuser::MB_ICONINFORMATION,
        );
    }
}
