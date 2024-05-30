use egui::{Context, Ui};
use crate::app::Config;


pub fn page_configedit(ui: &mut Ui, ctx: &Context, config: &mut Config)
{

    ui.heading("Default layout");
    ui.label("Friendly App Name = C:\\full\\App\\Directory");
    ui.separator();
    egui::TopBottomPanel::top("config_edit_top_panel").show(ctx, |ui| { 
        ui.horizontal(|ui| {  
            config.general_menu_without_format(ui);
        });
    });

    ui.heading("New Entry");
        


    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut config.key_new);
        ui.label("=");
        ui.text_edit_singleline(&mut config.val_new);

        ui.style_mut().text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(16.0, eframe::epaint::FontFamily::Monospace),
        );
        ui.menu_button("Add to", |ui| {
            for section in config.config.clone().sections()
            {
                if section.is_some() && (section != Some("Port"))
                {

                    if ui.button(section.unwrap()).clicked()
                    {
                        config.add_entry(section.unwrap().to_string(), config.key_new.clone().to_string(), config.val_new.clone().to_string());
                    }
                }

            }
        });
    });

    
    ui.add_space(30.0);
    ui.heading("Current Sections with Entries");
    ui.separator();
    egui::ScrollArea::vertical().id_source("SettingsLinks").show(ui, |ui|{
        for section in config.config.clone().sections()
        {
            
            if section.is_some() && (section != Some("Port"))
            {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Heading, 
                    egui::FontId::new(20.0, eframe::epaint::FontFamily::Monospace));
                ui.heading(section.unwrap());
                ui.add_space(5.0);
                for (key, value) in config.config.section(section).unwrap().clone().iter()
                {
                    


                    ui.horizontal(|ui| {
                        ui.label(key);
                        ui.label(" = ");
                        ui.label(value);
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            ui.style_mut().text_styles.insert(
                                egui::TextStyle::Button,
                                egui::FontId::new(16.0, eframe::epaint::FontFamily::Monospace),
                            );
                            if ui.add_sized([100.0, 10.0], egui::widgets::Button::new("Remove")).clicked()
                            {
                                config.remove_entry(section.clone().unwrap().to_string(), key.to_owned());
                            }
                        }); 
                    });

                    ui.separator();
                }
    
                ui.add_space(10.0);
            } 
        }

        ui.add_space(30.0);
    });





}


