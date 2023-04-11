#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu, WindowEvent};
use blender_batch_render_helper::{os_fn, process, aws, jobs};
use blender_batch_render_helper::utils::file_logger::{FileLogger, FileLoggerPath};
use blender_batch_render_helper::utils::logger::Logger;
use blender_batch_render_helper::utils::sentry_logger::SentryLogger;
use clap::Parser;
use aws_sdk_cloudformation as cloudformation;
use blender_batch_render_helper::env_mod;
use blender_batch_render_helper::stack_file_repo;
use blender_batch_render_helper::create_stack::CreateStack;

#[derive(Debug, Parser)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The name of the AWS CloudFormation stack.
    #[structopt(short, long)]
    stack_name: String,

    /// The name of the file containing the stack template.
    #[structopt(short, long)]
    template_file: String,

    /// Whether to display additional runtime information.
    #[structopt(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let _guard = sentry::init(("https://3bba3730ab29474d8991d8e057fce4b9@o4504853594832896.ingest.sentry.io/4504860192931840", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
    let _sentry = sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        // To set a uniform sample rate
        traces_sample_rate: 0.2,
        // The Rust SDK does not currently support `traces_sampler`

        ..Default::default()
    });

    let config = aws_config::load_from_env().await;
    let client = cloudformation::Client::new(&config);

    let create_stack = CreateStack {
        client,
        stack_name: "stack".to_string(),
        template_body: "".to_string(),
    };


    let sentry_logger = SentryLogger {};
    let mut file_logger: FileLogger = FileLogger {
        file_path: "".to_string(),
    };

    file_logger.set_log_file_path("/tmp/brh.log");

    file_logger.log("[RUST]: Starting app");

    tauri::Builder::default()
        .manage(sentry_logger)
        .manage(file_logger.clone())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(move |_app| {
            Ok(())
        })
        .menu(Menu::with_items([
            MenuEntry::Submenu(Submenu::new(
                "File",
                Menu::with_items([
                    #[cfg(target_os = "macos")]
                        CustomMenuItem::new("hello", "Hello").into(),
                    MenuItem::CloseWindow.into(),
                ]),
            )),
        ]))
        .on_window_event(|e| match e.event() {
            WindowEvent::Resized(_) => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested { .. } => {}
            WindowEvent::Destroyed => {}
            WindowEvent::Focused(focused) => {
                e.window().emit("focused", focused).unwrap();
            }
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::FileDrop(_) => {}
            WindowEvent::ThemeChanged(_) => {}
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            aws::create_stack,
            os_fn::get_os,
            os_fn::check_os_feature,
            os_fn::create_blender_file,
            os_fn::has_stack,
            os_fn::open_url,
            os_fn::open_folder_beginning_with_string,
            jobs::parse_csv,
            process::process_render,
            process::start_render,
            env_mod::check_env_var,
            env_mod::get_env_var,
            env_mod::add_or_update_env_var,
            env_mod::check_aws_auth_file,
            env_mod::write_aws_auth_to_file,
            env_mod::get_aws_credentials,
            stack_file_repo::has_stack_file_repo,
            stack_file_repo::create_stack_file_repo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
