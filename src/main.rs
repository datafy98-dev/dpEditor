mod app_state;
mod ui;
mod particle_preview;

use druid::{AppLauncher, WindowDesc};

fn main() {
    let window = WindowDesc::new(ui::ui_builder())
        .title("Particle Editor")
        .window_size((800.0, 600.0));

    let initial_state = app_state::AppState::default();

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}