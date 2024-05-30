use egui::Ui;
use crate::app::Config;

pub fn page_config(ui: &mut Ui, config: &mut Config)
{

    config.general_menu(ui);

    
    ui.columns(2, |ui|{
        ui[0].heading("Pending Config");
        egui::ScrollArea::vertical().id_source("pendingconfig").show(&mut ui[0], |ui|{
            config.display_config("original", ui);
            ui.add_space(30.0);
        });
        ui[1].heading("Current Config");
        egui::ScrollArea::vertical().id_source("currentconfig").show(&mut ui[1], |ui|{
            config.display_config("copy", ui);
            ui.add_space(30.0);
        });
    });

    
}