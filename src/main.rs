#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use axum::{
    routing::get,
    Router,
    extract::Path,
};
// When compiling natively:

fn main() -> eframe::Result<()> {
    
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    for argument in args.clone() {

        match argument.as_str() {
            "api" => {
                api()
            },
            _ => {
                applauncher::Config::launch_section_standalone(argument);
            },
        }
    }

    if !args.is_empty()
    {
        std::process::exit(0);
    }


    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1500.0, 800.0])
            .with_min_inner_size([1100.0, 700.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-512.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "AppLauncher",
        native_options,
        Box::new(|cc| Box::new(applauncher::TemplateApp::new(cc))),
    )
}

#[tokio::main]
async fn api()
{

    let port = applauncher::Config::get_port();
    let mut addr = "0.0.0.0:".to_string();
    addr.push_str(port.as_str());

    let app: Router = Router::new()
    .route("/test", get(test));



    let api_routes: Router = Router::new()
    .merge(app)
    .merge(app_router());


    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, api_routes).await.unwrap();

}

fn app_router() -> Router
{
    if applauncher::Config::key_exists()
    {
        return Router::new().route("/api/v1/launch/:key/:section", get(launch_key))
        .route("/api/v1/:key/shutdown", get(key_shutdown))
        .route("/api/v1/:key/restart", get(key_restart))
        .route("/api/v1/:key/quit", get(key_quit));
    } else {
        return Router::new().route("/api/v1/launch/:section", get(launch))
        .route("/api/v1/shutdown", get(shutdown))
        .route("/api/v1/restart", get(restart))
        .route("/api/v1/quit", get(quit))

    }
}


async fn test() -> String
{
    "Connection Successful".to_string()
}

async fn quit()
{
 std::process::exit(0);
}

async fn key_quit(Path(key):Path<String>)
{
    if applauncher::Config::key_matches(key)
    {
        std::process::exit(0);
    } 
}

async fn shutdown() -> String
{
    std::process::Command::new("shutdown")
        .arg("/t")
        .arg("1")
        .arg("/s")
        .spawn()
        .expect("failed to execute process");
    "Shutting Down".to_string()
}

async fn restart() -> String
{
    std::process::Command::new("shutdown")
        .arg("/t")
        .arg("1")
        .arg("/r")
        .spawn()
        .expect("failed to execute process");
    "Restarting".to_string()
}

async fn key_shutdown(Path(key):Path<String>) -> String
{
    if applauncher::Config::key_matches(key)
    {
        std::process::Command::new("shutdown")
            .arg("/t")
            .arg("1")
            .arg("/s")
            .spawn()
            .expect("failed to execute process");
        "Restarting".to_string()
    } else {
        return "Wrong".to_string();
    }
}

async fn key_restart(Path(key):Path<String>) -> String
{
    if applauncher::Config::key_matches(key)
    {
        std::process::Command::new("shutdown")
            .arg("/t")
            .arg("1")
            .arg("/r")
            .spawn()
            .expect("failed to execute process");
        "Restarting".to_string()
    } else {
        return "Wrong".to_string();
    }

}

async fn launch_key(Path((key, section)):Path<(String, String)>) -> String

{

    if applauncher::Config::key_matches(key)
    {
        applauncher::Config::launch_section_standalone(section.clone());
        return format!("Launching {}", section).to_string();
    } else {
        return "Wrong".to_string();
    }

}


async fn launch(Path(section):Path<String>) -> String {
    applauncher::Config::launch_section_standalone(section);
    "Launching".to_string()
}