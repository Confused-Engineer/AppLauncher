use std::os::windows::process::CommandExt;

use egui::Ui;
use crate::app::Config;
use mslnk::ShellLink;

pub fn page_main(ui: &mut Ui, config: &mut Config)
{

    ui.heading("Launch");
    egui::ScrollArea::horizontal().id_source("SectionButtons").show(ui, |ui|{
        ui.horizontal(|ui|{
            for section in config.config.clone().sections()
            {
                if section.is_some() && (section != Some("Port"))
                {
                    if ui.add_sized([100.0, 40.0], egui::widgets::Button::new(section.unwrap())).clicked()
                    {
                        config.launch_section(section.unwrap().to_string());
                    }
                }
            }
            if ui.add_sized([100.0, 40.0], egui::widgets::Button::new("Start API")).clicked()
            {
                std::process::Command::new("cmd")
                    .args(["/C",std::env::current_exe().unwrap().display().to_string().as_str(),"api"])
                    .creation_flags(0x08000000)
                    .spawn()
                    .expect("failed to execute process");

                std::process::Command::new("cmd")
                    .arg("/c")
                    .arg("curl")
                    .arg(format!("http://localhost:{}/test", Config::get_port()))
                    .arg("&")
                    .args(["timeout", "5"])
                    .spawn()
                    .expect("failed to execute process");
            }
            if ui.add_sized([100.0, 40.0], egui::widgets::Button::new("End API")).clicked()
            {
                std::process::Command::new("curl")
                    .arg(format!("http://localhost:{}/api/v1/quit", Config::get_port()))
                    .creation_flags(0x08000000)
                    .spawn()
                    .expect("failed to execute process");
            }

        });
    });
    ui.separator();
    ui.heading("Generate Shortcuts");
    egui::ScrollArea::horizontal().id_source("ShortcutButtons").show(ui, |ui|{
        ui.horizontal(|ui|{

            for section in config.config.clone().sections()
            {
                if section.is_some() && (section != Some("Port"))
                {
                    if ui.add_sized([100.0, 40.0], egui::widgets::Button::new(section.unwrap())).clicked()
                    {
                        let userdir = directories::UserDirs::new().unwrap();
                        let target = std::env::current_exe().unwrap().display().to_string();
                        let lnk = format!(r"{}\AppLauncher {}.lnk", userdir.desktop_dir().unwrap().display(), section.unwrap());
                        let mut sl = ShellLink::new(target).unwrap();
                        sl.set_arguments(Some(section.unwrap().to_string()));
                        sl.create_lnk(lnk).unwrap();
                    }
                }
            }
            if ui.add_sized([100.0, 40.0], egui::widgets::Button::new("API")).clicked()
            {
                let userdir = directories::UserDirs::new().unwrap();
                let target = std::env::current_exe().unwrap().display().to_string();
                let lnk = format!(r"{}\AppLauncher API.lnk", userdir.desktop_dir().unwrap().display());
                let mut sl = ShellLink::new(target).unwrap();
                sl.set_arguments(Some("api".to_string()));
                sl.create_lnk(lnk).unwrap();
            }
            #[cfg(debug_assertions)]
            if ui.add_sized([100.0, 40.0], egui::widgets::Button::new("TEST")).clicked()
            {
                let userdir = directories::UserDirs::new().unwrap();
                let target = std::env::current_exe().unwrap().display().to_string();
                
                
                let lnk = format!(r"{}\AppLauncher API.lnk", userdir.desktop_dir().unwrap().display());
                let mut sl = ShellLink::new(target).unwrap();
                sl.set_arguments(Some("api".to_string()));
                sl.create_lnk(lnk).unwrap();
            }
        });
    });

    ui.separator();
    egui::ScrollArea::vertical().id_source("SettingsLinks").show(ui, |ui|{

        
        ui.heading("How to Use");
        ui.label("Launch: Select the buttons to launch the applications listed in the matchingsection in the config file.");
        ui.add_space(5.0);
        ui.label("Generate Shortcuts: This will make a shortcut to the section that can be used to launch a section directly without entering the app. The only exception is the API shortcut which launches the API directly.");
        ui.add_space(5.0);
        ui.label("Note: Open the config file in any of the Configuration Pages to add/remove sections, and add apps using the \"Config Edit\" page.");
        ui.separator();
        ui.heading("API Usage");
        ui.label("The API can be used to allow ecternal devices, like Home Assistant or a phone/Other PC to launch sections.");
        ui.add_space(30.0);
    });
}