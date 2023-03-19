#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu, WindowEvent};
use blender_batch_render_helper::{os_fn, process, aws, jobs};
use blender_batch_render_helper::env_mod;
use blender_batch_render_helper::installers::{installer};


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

    tauri::Builder::default()
        .setup(|app| {
            let path = app.path_resolver().app_data_dir().unwrap();
            let path = path.join(".config").join(".env");
            let path = path.to_str().unwrap();
            env_mod::run_bootstrap(path, app);

            let app_dir = app.path_resolver().app_data_dir().unwrap();
            let app_dir = app_dir.to_str().unwrap();
            let blender_path = format!("{}/.config/.blender", &app_dir);
            let deps_path = format!("{}/.brh-ext-deps", app_dir);
            os_fn::create_blender_folder(&blender_path);
            os_fn::create_blender_folder(&deps_path);
            os_fn::clone_git_project(&deps_path);
            os_fn::init_job_list(app);
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
            env_mod::bootstrap_env,
            env_mod::add_or_update_env_var,
            installer::has_dependencies,
            installer::start_installation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
