#![windows_subsystem = "windows"]
use auto_launch::AutoLaunchBuilder;
use emergency_backup::{app, config::Config, log};
use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::env;

fn main() {
    let exe = env::current_exe().unwrap();
    let app_name = "backup-group1";
    let wd = exe.parent().unwrap();
    let app_path = wd.join("emergency-backup");

    let auto = AutoLaunchBuilder::new()
        .set_app_name(&app_name)
        .set_app_path(&app_path.to_str().unwrap())
        .set_use_launch_agent(true)
        .build()
        .unwrap();

    auto.enable().unwrap();
    auto.is_enabled().unwrap();
    println!("Autostart enabled");

    let config = Config::initialize();

    println!("Config file located at {}", config.config_file());
    println!("whose content is : {:#?}", config);

    // Writing to the log
    _ = log::write_log(config.log_file());

    let mut pressed_keys = HashSet::new();

    let callback = {
        move |event: Event| match event.event_type {
            EventType::KeyPress(key) => {
                pressed_keys.insert(key);

                if pressed_keys.contains(&Key::ControlLeft)
                    && pressed_keys.contains(&Key::KeyB)
                    && pressed_keys.contains(&Key::ShiftLeft)
                    && pressed_keys.contains(&Key::Alt)
                {
                    app::run();
                }
            }
            EventType::KeyRelease(key) => {
                pressed_keys.remove(&key);
            }
            _ => {}
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error in listening to events: {:?}", error);
    }

    auto.disable().unwrap();
    auto.is_enabled().unwrap();
}
