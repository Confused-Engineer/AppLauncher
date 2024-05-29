use egui::{Context, Ui};
use crate::app::Config;


pub fn page_configedit(ui: &mut Ui, ctx: &Context, config: &mut Config)
{

    egui::TopBottomPanel::top("config_edit_top_panel").show(ctx, |ui| {

        
        ui.horizontal(|ui| {  
            config.general_menu_without_format(ui);
        

        });


        
    });

    



    for section in config.config.clone().sections()
    {
        
        if section.is_some()
        {
            
            ui.heading(section.unwrap());
            for (key, value) in config.config.section(section).unwrap().clone().iter()
            {
                ui.horizontal(|ui| {
                    ui.label(key);
                    ui.label(" = ");
                    ui.label(value);
                });
            }


            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut config.key_new);
                ui.label("=");
                ui.text_edit_singleline(&mut config.val_new);
                if ui.button("Add App").clicked()
                {

                    config.add_entry(section.unwrap().to_string());
                    add_entry(section.unwrap().to_string(), config.key_new.clone().to_string(), config.val_new.clone().to_string());
                }
            });
        } 
    }



}


fn add_entry(section: String, key: String, value: String)
{

    let temp_config = ini::Ini::load_from_file_noescape("config.ini");
    if temp_config.is_ok()
    {
        let mut write = temp_config.unwrap();
        write.with_section(Some(section)).set(key, value);
        
        let _ = write.write_to_file_policy("config.ini", ini::EscapePolicy::Nothing);
        
        
    } 


    
    //self.config.with_section(Some(sec)).set(self.key_new.clone(), self.val_new.clone());
    
}