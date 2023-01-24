#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use blender_batch_render_helper::{os_fn, process, aws};
use blender_batch_render_helper::env_mod;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let path = app.path_resolver().app_data_dir().unwrap();
            let path = path.join(".config").join(".env");
            let path = path.to_str().unwrap();
            env_mod::bootstrap_env(path);

            let app_dir = app.path_resolver().app_data_dir().unwrap();
            let app_dir = app_dir.to_str().unwrap();
            os_fn::create_blender_folder(&app_dir);

            os_fn::init_job_list(app);
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            Ok(())

        })
        .invoke_handler(tauri::generate_handler![
            aws::create_stack,
            os_fn::get_os,
            os_fn::check_os_feature,
            os_fn::create_blender_file,
            os_fn::has_stack,
            process::process_render,
            process::start_render,
            env_mod::check_env_var,
            env_mod::get_env_var,
            env_mod::bootstrap_env,
            env_mod::add_or_update_env_var,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
