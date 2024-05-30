use egui::Ui;
use crate::app::Config;

pub fn page_main(ui: &mut Ui, config: &mut Config)
{

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
        });
    });

    ui.separator();
    egui::ScrollArea::vertical().id_source("SettingsLinks").show(ui, |ui|{

        
        ui.heading("How to Use");
        ui.label("The buttons above are sections in the config file, and clicking the buttons will launch each application listed in the given section");
        ui.add_space(5.0);
        ui.label("Check out the configuration sections, or open the config file to edit the sections and applications listed in them. The sections are not set statically and be added, removed, edited to fit your needs.");
        ui.add_space(5.0);
        ui.label("Note: erasing/editing the 'Port' section may result in the API failing to function. if you do not want the API you can remove the section completely.");
        ui.separator();
        ui.heading("API Usage");
        ui.label("The API can be used to integrate this application with outside applications like Home Assistant to provide some external automation. For example if you want to launch work apps at 8am, this application will allow something like Home Assistant to launch specific sections similar to how the buttons perform.");
        ui.add_space(30.0);
    });
}