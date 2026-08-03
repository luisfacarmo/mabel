mod commands;
mod device;
pub mod settings;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let (cmd_tx, cmd_rx) = device::command_channel();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(cmd_tx)
        .setup(|app| {
            let handle = app.handle().clone();
            tray::setup_tray(&handle)?;
            tauri::async_runtime::spawn(device::run_loop(handle, cmd_rx));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::set_sound_mode,
            commands::set_equalizer,
            commands::set_ldac,
            commands::set_dolby,
            commands::set_sidetone,
            commands::set_auto_power_off,
            commands::set_mode_cycle,
            commands::get_settings,
            commands::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
