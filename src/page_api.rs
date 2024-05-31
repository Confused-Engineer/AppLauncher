use egui::{Context, Ui};
use crate::app::Config;

pub fn show_page_api(ui: &mut Ui, ctx: &Context, config: &mut Config)
{
    
    egui::TopBottomPanel::top("api_top_panel").show(ctx, |ui| { 
        ui.horizontal(|ui| {  
            config.general_menu_without_format(ui);
        });
    });
    
    ui.heading("Use the API to allow communication between this device and your smart home or other external devices.");
    ui.label(format!("To start the api put:\n{} api\ninto Command Prompt, or make a Windows Shortcut.", std::env::current_exe().unwrap().display()));
    ui.separator();
    ui.heading("Example Commands");

    if config.config.with_section(Some("Port")).get("Port").is_some() && config.config.with_section(Some("Port")).get("Key").is_some()
    {
        if (config.config.with_section(Some("Port")).get("Key") != Some("")) && (config.config.with_section(Some("Port")).get("Port") != Some(""))
        {
            ui.label("curl http://".to_string() + local_ip_address::local_ip().unwrap().to_string().as_str() + ":" + config.config.with_section(Some("Port")).get("Port").unwrap().to_string().as_str() + "/api/v1/launch/" + config.config.with_section(Some("Port")).get("Key").unwrap().to_string().as_str() + "/section_name");
            ui.label("curl http://".to_string() + local_ip_address::local_ip().unwrap().to_string().as_str() + ":" + config.config.with_section(Some("Port")).get("Port").unwrap().to_string().as_str() + "/api/v1/" + config.config.with_section(Some("Port")).get("Key").unwrap().to_string().as_str() + "/shutdown");
            ui.label("curl http://".to_string() + local_ip_address::local_ip().unwrap().to_string().as_str() + ":" + config.config.with_section(Some("Port")).get("Port").unwrap().to_string().as_str() + "/api/v1/" + config.config.with_section(Some("Port")).get("Key").unwrap().to_string().as_str() + "/restart");
            ui.label("curl http://".to_string() + local_ip_address::local_ip().unwrap().to_string().as_str() + ":" + config.config.with_section(Some("Port")).get("Port").unwrap().to_string().as_str() + "/api/v1/" + config.config.with_section(Some("Port")).get("Key").unwrap().to_string().as_str() + "/quit");
        
        } else if config.config.with_section(Some("Port")).get("Port") != Some("") {
            ui.label("curl http://".to_string() + local_ip_address::local_ip().unwrap().to_string().as_str() + ":" + config.config.with_section(Some("Port")).get("Port").unwrap().to_string().as_str() + "/api/v1/launch/section_title");
            ui.label("curl http://".to_string() + local_ip_address::local_ip().unwrap().to_string().as_str() + ":" + config.config.with_section(Some("Port")).get("Port").unwrap().to_string().as_str() + "/api/v1/shutdown");
            ui.label("curl http://".to_string() + local_ip_address::local_ip().unwrap().to_string().as_str() + ":" + config.config.with_section(Some("Port")).get("Port").unwrap().to_string().as_str() + "/api/v1/restart"); 
            ui.label("curl http://".to_string() + local_ip_address::local_ip().unwrap().to_string().as_str() + ":" + config.config.with_section(Some("Port")).get("Port").unwrap().to_string().as_str() + "/api/v1/quit"); 
          
        } else {
            ui.label("Specify a Port by typing in any number 5000-60000 below");
        }
    } else {
        ui.label("Generate a Port/Key by reseting the config or clicking 'Set Port/Key' Below");
    }


    
    
    
    ui.separator();
    ui.heading("Current Config");
    ui.horizontal(|ui|{
        ui.label("Port:");
        if config.config.with_section(Some("Port")).get("Port").is_some()
        {
            ui.label(config.config.with_section(Some("Port")).get("Port").unwrap())
        } else {
            ui.label("Port Not Found")
        }
        
        
    });

    ui.horizontal(|ui|{
        ui.label("Key:");
        if config.config.with_section(Some("Port")).get("Key").is_some() && (config.config.with_section(Some("Port")).get("Key") != Some(""))
        {
            ui.label(config.config.with_section(Some("Port")).get("Key").unwrap())
        } else {
            ui.label("Key not Found")
        }
        
        
    });

    ui.separator();
    ui.heading("Configure the Port and Key");
    ui.label("The port should be between 5000-60000 to avoid issues. The default is 4999 is API is launched with no Port specified.");
    ui.label("If the key is left empty then no authentication is required to run commands from seperate devices.");

    ui.horizontal(|ui|{
        ui.label("Port:");

        ui.text_edit_singleline(&mut config.port_new);
        ui.style_mut().text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(16.0, eframe::epaint::FontFamily::Monospace),
        );
        if ui.button("Set Port").clicked()
        {
            config.add_entry("Port".to_string(), "Port".to_string(), config.port_new.clone().chars().filter(|char| char.is_digit(10)).collect());
            
        }
    });

    ui.horizontal(|ui|{
        ui.label("Key:");
        ui.text_edit_singleline(&mut config.token_new);
        
        ui.style_mut().text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(16.0, eframe::epaint::FontFamily::Monospace),
        );

        if ui.button("Set Key").clicked()
        {
            config.add_entry("Port".to_string(), "Key".to_string(), config.token_new.clone().chars().filter(|char| char.is_alphanumeric()).collect());
        }
    });


}