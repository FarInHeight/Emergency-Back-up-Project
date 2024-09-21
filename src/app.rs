use std::{
    env,  process::Command
};

use crate:: {
    utils::beep,
    config::Config,
    backup,
};
use cpu_time::ThreadTime;
use eframe::{run_native, App, Frame, NativeOptions};
use egui::{CentralPanel, Context, Key, ScrollArea, TopBottomPanel, Vec2, ViewportBuilder};

#[derive(Default)]
struct EmergencyBackupApp {
    config: Config,
    text: String,
}

impl EmergencyBackupApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.style_mut(|style| {
            style.spacing.item_spacing = Vec2::new(0., 10.0);
        });
        cc.egui_ctx.set_zoom_factor(1.1);
        beep(0.3, 0);
        Self { config: Config::initialize(), text: "".to_string()}
    }
}

impl App for EmergencyBackupApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top").show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.label("The emergency back-up phase has been initiated"));
        });

        TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.label("© P. Pitarresi, D. Sferrazza, D. Vitabile - 2024"))
        });

        CentralPanel::default().show(&ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() / 2.0 - 50.0);
                    ui.label("Are you sure to start the emergency back-up?\nPress the button or use the shortcut Ctrl + B");
                    if ui.button("Confirm").clicked() || ctx.input(|i| i.key_pressed(Key::B) && i.modifiers.command_only())  {
                        let start_backup_time = ThreadTime::now();
                        match backup::all(self.config.source_as_path(), self.config.destination_as_path()){
                            Ok(size) => {
                                let cpu_time = start_backup_time.elapsed();
                                match backup::create_backup_summary(
                                    size, 
                                    cpu_time, 
                                    self.config.destination_as_path()
                                ) {
                                    Ok(()) => {
                                        beep(0.3, 1);
                                        self.text = "Backup completed".to_string();
                                    },
                                    Err(e) => {
                                        eprintln!("{e}");
                                        beep(0.4, 2);
                                        self.text = format!("{e}");
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("{e}");
                                beep(0.4, 2);
                                self.text = format!("{e}");
                            }
                        }
                        
                    }
                    ui.label(&self.text);
                    
                });
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let path = env::current_exe()
            .expect("Cannot get executable file path!")
            .to_str()
            .unwrap()
            .to_string();
        Command::new(path)
            .spawn()
            .expect("Cannot rerun the application!");
    }
}

pub fn run() {
    let options = NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(Vec2 { x: 450., y: 260. }),
            ..Default::default()
        },
        centered: true,
        run_and_return: false,
        ..Default::default()
    };
    let _ = run_native(
        "Emergency Backup App",
        options,
        Box::new(|cc| Ok(Box::new(EmergencyBackupApp::new(cc)))),
    );
}
pub fn run_with_size(size: (f32, f32)) {
    let options = NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(Vec2::from(size)),
            ..Default::default()
        },
        centered: true,
        run_and_return: false,
        ..Default::default()
    };
    let _ = run_native(
        "Emergency Backup App",
        options,
        Box::new(|cc| Ok(Box::new(EmergencyBackupApp::new(cc)))),
    );
}
