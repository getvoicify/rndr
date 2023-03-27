#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu, WindowEvent};
use blender_batch_render_helper::{os_fn, process, aws, jobs};
use blender_batch_render_helper::env_mod;
use blender_batch_render_helper::utils::file_logger::{FileLogger, FileLoggerPath};
use blender_batch_render_helper::utils::logger::Logger;
use blender_batch_render_helper::utils::sentry_logger::SentryLogger;


fn main() {
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

    // let create_stack_manager = CreateStack {
    //     client: (),
    //     stack_name: "".to_string(),
    //     template_body: "".to_string(),
    // };


    let sentry_logger = SentryLogger {};
    let mut file_logger: FileLogger = FileLogger {
        file_path: "".to_string(),
    };

    file_logger.set_log_file_path("/tmp/brh.log");

    file_logger.log("[RUST]: Starting app");

    tauri::Builder::default()
        .manage(sentry_logger)
        .manage(file_logger.clone())
        .setup(move |_app| {

            // let path = app.path_resolver().app_data_dir().unwrap();
            // let path = path.join(".config").join(".env");
            // let path = path.to_str().unwrap();
            // env_mod::run_bootstrap(path, app, &file_logger);
            //
            // let app_dir = app.path_resolver().app_data_dir().unwrap();
            // let app_dir = app_dir.to_str().unwrap();
            // let blender_path = format!("{}/.config/.blender", &app_dir);
            // let deps_path = format!("{}/.brh-ext-deps", app_dir);
            // os_fn::create_blender_folder(&blender_path, &file_logger);
            // os_fn::create_blender_folder(&deps_path, &file_logger);
            // os_fn::clone_git_project(&deps_path);
            // os_fn::init_job_list(app, &file_logger);
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
