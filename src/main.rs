use std::thread;
use std::process;
use std::time::Duration;
use std::ffi::OsStr;
use std::path::Path;
use std::os::windows::ffi::OsStrExt;

extern crate systray;
extern crate user32;
extern crate winapi;
extern crate ini;

use ini::Ini;

use user32::MessageBoxW;
use winapi::winuser::{MB_OK};

fn main() {
    if !Path::new("settings.ini").exists() {
        create_config();
    }

    let mut s_title = to_wstring("Error!");
    let mut s_content = to_wstring("Error!");
    let mut s_delay = 30;

    reload_config(&mut s_title, &mut s_content, &mut s_delay);

    thread::spawn(|| {
        let mut app;
        match systray::Application::new() {
            Ok(w) => app = w,
            Err(e) => panic!("Can't create window! {}", e),
        }
        let mut w = &mut app.window;
        let icon = include_bytes!("assets/icon.ico");
        let _ = w.set_icon_from_buffer(icon, 16, 16);
        let _ = w.set_tooltip(&"Notification".to_string());

        let _ = w.add_menu_item(&"Quit".to_string(), |window| {
            window.quit();
            process::exit(0);
        });

        w.wait_for_message();
    });

    loop {
        unsafe {
            MessageBoxW(std::ptr::null_mut(),
                s_content.as_ptr(),
                s_title.as_ptr(),
                MB_OK);
        }

        reload_config(&mut s_title, &mut s_content, &mut s_delay);

        // Sleep for user-specified delay
        thread::sleep(Duration::from_secs(s_delay*60));
    }
}

fn to_wstring(str: &str) -> Vec<u16> {
    let v: Vec<u16> = OsStr::new(str)
    .encode_wide()
    .chain(Some(0).into_iter())
    .collect::<Vec<_>>();
    v
}

fn reload_config(title: &mut Vec<u16>, content: &mut Vec<u16>, delay: &mut u64) {
    let conf = Ini::load_from_file("settings.ini").unwrap();

    let settings = conf.section(Some("Settings".to_owned())).unwrap();
    *title = to_wstring(settings.get("title").unwrap_or(&"Error!".to_string()));
    *content = to_wstring(settings.get("content").unwrap_or(&"Error!".to_string()));
    *delay = settings.get("delay").unwrap().parse::<u64>().unwrap_or(30);
}

fn create_config() {
    let mut conf = Ini::new();
    conf.with_section(Some("Settings".to_owned())).set("title", "Notification");
    conf.with_section(Some("Settings".to_owned())).set("content", "Hey!");
    conf.with_section(Some("Settings".to_owned())).set("delay", "30");
    conf.write_to_file("settings.ini").unwrap();
}
