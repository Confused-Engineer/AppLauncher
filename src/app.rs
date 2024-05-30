use egui::Ui;
use ini::{Error, Ini};
use std::env;
use std::fs::{remove_file, File};
use std::io::Write;
use std::os::windows::process::CommandExt;
use std::process::Command;

use crate::page_config::page_config;
use crate::page_config_edit::page_configedit;
use crate::page_main::page_main;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    page: Pages,
    #[serde(skip)]
    config: Config,

    //#[serde(skip)] // This how you opt-out of serialization of a field

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            page: Pages::default(),
            config: Config::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn update_app(&mut self)
    {

        Command::new("cmd")
            .args(["/C","msg", "%username%","Updating to latest version"])
            .creation_flags(0x08000000)
            .spawn()
            .expect("failed to execute process");

        Command::new("cmd")
            .args(["/C","timeout", "1","&","curl.exe","-L","https://github.com/Confused-Engineer/AppLauncher/releases/download/nightly/AppLauncher.exe","-o",env::current_exe().unwrap().to_str().unwrap(),"&","timeout","1"])
            .creation_flags(0x08000000)
            .spawn()
            .expect("failed to execute process");

        
        std::process::exit(0);

    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!

                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.menu_button("View Page", |ui|{
                        if ui.button("Main").clicked()
                        {
                            ui.close_menu();
                            self.page.set_page("main");
                        }
                        
                        ui.menu_button("Config", |ui| {

                            if ui.button("General").clicked()
                            {
                                ui.close_menu();
                                self.page.set_page("config");
                            }

                            if ui.button("Edit").clicked()
                            {
                                ui.close_menu();
                                self.page.set_page("configedit");
                            }
                            
                        });

                        if ui.button("API").clicked()
                        {
                            ui.close_menu();
                            self.page.set_page("api");
                        }
                    });

                    if ui.button("Update").clicked()
                    {
                        self.update_app()
                    }
                    ui.add_space(16.0);


                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(20.0, eframe::epaint::FontFamily::Proportional),
            );
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Body, 
                egui::FontId::new(16.0, eframe::epaint::FontFamily::Proportional),
            );
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Heading, 
                egui::FontId::new(20.0, eframe::epaint::FontFamily::Proportional),
            );
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Small, 
                egui::FontId::new(15.0, eframe::epaint::FontFamily::Proportional),
            );      
                 


            if self.page.main
            {
                ui.heading("Main");
                ui.separator();
                page_main(ui, &mut self.config)
            }

            if self.page.config
            {
                ui.heading("Config");
                ui.separator();
                page_config(ui, &mut self.config);
            }

            
            if self.page.configedit
            {
                ui.add_space(20.0);
                ui.heading("Config");
                ui.separator();
                page_configedit(ui, ctx, &mut self.config);
            }

            if self.page.api
            {
                ui.heading("API");
                ui.separator();
            }

            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui|{
                    ui.hyperlink_to("Source Code", "https://github.com/Confused-Engineer/AppLauncher");
                    ui.label("Brought to you by a confused engineer.");
                });
                ui.separator();
            });
        });
    }
}


struct Pages {
    main: bool,
    config: bool,
    configedit: bool,
    api: bool,
}

impl Default for Pages {
    fn default() -> Self {
        Self {
            main: true,
            config : false,
            configedit: false,
            api: false,
        }
    }
}

impl Pages {
    fn set_page (&mut self, page: &str) {
        self.main = false;
        self.config = false;
        self.api = false;
        self.configedit = false;

        match page {
            "main" => {
                self.main = true;
            },
            "config" => {
                self.config = true;
            },
            "configedit" => {
                self.configedit = true;
            },
            "api" => {
                self.api = true;
            },
            _ => {
                self.main = true;
            },
        }
    }
}



pub struct  Config {
    pub config: Ini,
    pub config_cp: Ini,
    config_check: Result<Ini, Error>, 

    config_path: String,

    config_change_pending: bool,
    pub key_new: String,
    pub val_new: String,
}

impl Default for Config
{
    fn default() -> Self {
        Self {
            config: Config::config_load(),
            config_cp: Config::config_load(),
            config_check: Config::config_check_load(),
            config_path: Config::config_path(),
            config_change_pending: false,
            key_new: String::new(),
            val_new: String::new(),
        }
    }
}



impl Config
{
    pub fn validate(&mut self)
    {
        self.config_check = Ini::load_from_file_noescape(self.config_path.clone());
        if self.config_check.is_ok()
        {
            self.config = self.config_check.as_ref().unwrap().clone();
            self.config_cp = self.config_check.as_ref().unwrap().clone();
        }
        self.config_change_pending = false;
    }

    fn config_check_load() -> Result<Ini, Error>
    {
        let filename = env::current_exe().unwrap().as_path().display().to_string().split("\\").last().unwrap().to_string();
        let filepath = env::current_exe().unwrap().as_path().display().to_string().replace(&filename, "config.ini");
        let temp_config = Ini::load_from_file_noescape(filepath);
        return temp_config
    }

    fn config_load() -> Ini
    {

        let filename = env::current_exe().unwrap().as_path().display().to_string().split("\\").last().unwrap().to_string();
        let filepath = env::current_exe().unwrap().as_path().display().to_string().replace(&filename, "config.ini");
        let temp_config = Ini::load_from_file_noescape(filepath);
        if temp_config.is_ok()
        {
            return temp_config.unwrap();
        } else {
            return Ini::new();
        }
    }

    pub fn display_config(&self, config_version: &str, ui: &mut Ui)
    {
        #[allow(unused_assignments)]
        let mut config_temp = Ini::new();
        match config_version {
            "original" => {
                config_temp = self.config.clone();
            },
            "copy" => {
                config_temp = self.config_cp.clone();
            },
            _ => {
                config_temp = self.config.clone();
            },
        }

        for section in config_temp.sections()
        {
            if section.is_some()
            {
                ui.heading(section.unwrap());
                for (key, value) in config_temp.section(section).unwrap().clone().iter()
                {
                    ui.horizontal(|ui| {
                        ui.label(key);
                        ui.label(" = ");
                        ui.label(value);
                    });
                    ui.separator();
                }
            } 
        }
    }

    pub fn save(&mut self)
    {
        //let save = self.config.write_to_file("config.ini");
        let save = self.config.write_to_file_policy(self.config_path.clone(), ini::EscapePolicy::Nothing);
        if save.is_ok()
        {
            self.config_cp = self.config.clone();
        }
        self.config_change_pending = false;
    }

    pub fn clear(&mut self)
    {
        self.config = self.config_cp.clone();
        self.config_change_pending = false;
    }

    pub fn reset(&mut self)
    {
        let _ = remove_file(self.config_path.clone());
        let mut filemake = File::create(self.config_path.clone()).unwrap();
        let _ = filemake.write_all(include_bytes!("../config.ini"));
        self.config_change_pending = false;
        self.validate();
    }

    pub fn add_entry(&mut self,section: String, key: String, value: String)
    {

        self.config.with_section(Some(section)).set(key, value.replace("\"", ""));
        self.config_change_pending = true;
        
    }

    pub fn remove_entry(&mut self,section: String, key: String)
    {

        self.config.with_section(Some(section)).delete(&key);
        self.config_change_pending = true;
        
    }

    fn config_path() -> String
    {
        let filename = env::current_exe().unwrap().as_path().display().to_string().split("\\").last().unwrap().to_string();
        let filepath = env::current_exe().unwrap().as_path().display().to_string().replace(&filename, "config.ini");
        return filepath;
    }

    pub fn launch_section(&mut self, section: String)
    {

        for (_, value) in self.config.section(Some(section)).unwrap().clone().iter()
        {
            match value {
                val if (value.ends_with(".ps1")) => {
                    Command::new("powershell")
                    .args(["-executionpolicy","bypass","-windowstyle","minimized","-File",val])
                    .spawn()
                    .expect("failed to execute process");  
                },
                _ => {
                    Command::new("cmd")
                    .args(["/C", value])
                    .creation_flags(0x08000000)
                    .spawn()
                    .expect("failed to execute process");
                },
            }
        }
    }

    pub fn launch_section_standalone(section: String)
    {

        let config = Config::config_load();

        if config.section(Some(section.clone())).is_none()
        {

            return;
        }

        for (_, value) in config.section(Some(section)).unwrap().clone().iter()
        {
            match value {
                val if (value.ends_with(".ps1")) => {
                    Command::new("powershell")
                    .args(["-executionpolicy","bypass","-windowstyle","minimized","-File",val])
                    .spawn()
                    .expect("failed to execute process");  
                },
                _ => {
                    Command::new("cmd")
                    .args(["/C", value])
                    .creation_flags(0x08000000)
                    .spawn()
                    .expect("failed to execute process");
                },
            }
        }
    }

    pub fn get_port() -> String
    {
        let mut config = Config::config_load();

        if config.clone().section(Some("Port")).is_none()
        {
            return "4999".to_string();
        }

        let mut binding = config.with_section(Some("Port"));
        let port = binding.get("Port");
        if port.is_none()
        {
            return "4999".to_string();
        }

        port.unwrap().to_string()
    }

    pub fn key_exists() -> bool
    {
        let mut config = Config::config_load();

        if config.clone().section(Some("Port")).is_none()
        {
            return false;
        }

        let mut binding = config.with_section(Some("Port"));
        let port = binding.get("Key");
        if port.is_none() || port == Some("")
        {
            return false;
        }

        true       
    }

    pub fn key_matches(key: String) -> bool
    {
        let mut config = Config::config_load();
        let mut binding = config.with_section(Some("Port"));
        let config_key = binding.get("Key");
        if config_key.is_none()
        {
            return false;
        }

        if config_key.unwrap() == key
        {
            return true;
        }

        false
    }

    pub fn general_menu(&mut self ,ui: &mut Ui)
    {
        ui.columns(5, |ui| {
            if ui[0].add_sized([100.0, 40.0], egui::widgets::Button::new("Save Changes") ).clicked()
            {
                self.save();
            }
            if ui[1].add_sized([100.0, 40.0], egui::widgets::Button::new("Clear Changes") ).clicked()
            {
                self.clear();
            }
            if ui[2].add_sized([100.0, 40.0], egui::widgets::Button::new("Reload Config") ).clicked()
            {
                self.validate();
            }
            if ui[3].add_sized([100.0, 40.0], egui::widgets::Button::new("Reset Config") ).clicked()
            {
                self.reset();
            }
            if ui[4].add_sized([100.0, 40.0], egui::widgets::Button::new("Open Config") ).clicked()
            {

                Command::new("cmd")
                    .args(["/C", self.config_path.as_str()])
                    //.creation_flags(0x08000000)
                    .spawn()
                    .expect("failed to execute process");
            }
    
        });
        ui.separator();
    }
    pub fn general_menu_without_format(&mut self ,ui: &mut Ui)
    {

        if ui.button("Save Changes").clicked()
        {
            self.save();
        }
        if ui.button ("Clear Changes").clicked()
        {
            self.clear();
        }
        if ui.button("Reload Config").clicked()
        {
            self.validate();
        }
        if ui.button("Reset Config").clicked()
        {
            self.reset();
        }
        if ui.button("Open Config").clicked()
        {
            Command::new("cmd")
            .args(["/C", self.config_path.as_str()])
            //.creation_flags(0x08000000)
            .spawn()
            .expect("failed to execute process");
        }

        if self.config_change_pending
        {
            ui.label("Changes Pending");
        }
    

    }
}