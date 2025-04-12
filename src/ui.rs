use druid::{
    widget::{Flex, Label, Slider, Checkbox},
    Widget, WidgetExt,
};
use crate::app_state::AppState;

pub fn ui_builder() -> impl Widget<AppState> {
    // Ползунки для настройки параметров частиц
    let size_slider = Slider::new()
        .with_range(1.0, 50.0)
        .lens(AppState::initial_size);

    let shrink_slider = Slider::new()
        .with_range(0.01, 1.0)
        .lens(AppState::shrink_speed);

    let direction_slider = Slider::new()
        .with_range(0.0, 360.0)
        .lens(AppState::direction);

    let speed_slider = Slider::new()
        .with_range(1.0, 20.0)
        .lens(AppState::speed);

    let gravity_slider = Slider::new()
        .with_range(0.0, 2.0)
        .lens(AppState::gravity);

    let rotation_slider = Slider::new()
        .with_range(-1.0, 1.0)
        .lens(AppState::rotation_speed);

    let random_deviation_slider = Slider::new()
        .with_range(0.0, 20.0)
        .lens(AppState::random_deviation);

    // Ползунки для RGB компонентов начального цвета
    let start_color_r = Slider::new()
        .with_range(0.0, 1.0)
        .lens(AppState::start_color_r);

    let start_color_g = Slider::new()
        .with_range(0.0, 1.0)
        .lens(AppState::start_color_g);

    let start_color_b = Slider::new()
        .with_range(0.0, 1.0)
        .lens(AppState::start_color_b);

    // Ползунки для RGB компонентов конечного цвета
    let end_color_r = Slider::new()
        .with_range(0.0, 1.0)
        .lens(AppState::end_color_r);

    let end_color_g = Slider::new()
        .with_range(0.0, 1.0)
        .lens(AppState::end_color_g);

    let end_color_b = Slider::new()
        .with_range(0.0, 1.0)
        .lens(AppState::end_color_b);

    // Чекбокс для паузы спавна
    let pause_spawn_checkbox = Checkbox::new("Pause Spawn")
        .lens(AppState::is_spawning_paused);

    // Левая панель управления
    let controls = Flex::column()
        .with_child(Label::new("Initial Size"))
        .with_child(size_slider)
        .with_spacer(10.0)
        .with_child(Label::new("Shrink Speed"))
        .with_child(shrink_slider)
        .with_spacer(10.0)
        .with_child(Label::new("Direction"))
        .with_child(direction_slider)
        .with_spacer(10.0)
        .with_child(Label::new("Speed"))
        .with_child(speed_slider)
        .with_spacer(10.0)
        .with_child(Label::new("Gravity"))
        .with_child(gravity_slider)
        .with_spacer(10.0)
        .with_child(Label::new("Rotation Speed"))
        .with_child(rotation_slider)
        .with_spacer(10.0)
        .with_child(Label::new("Random Deviation"))
        .with_child(random_deviation_slider)
        .with_spacer(10.0)
        .with_child(Label::new("Start Color R"))
        .with_child(start_color_r)
        .with_child(Label::new("Start Color G"))
        .with_child(start_color_g)
        .with_child(Label::new("Start Color B"))
        .with_child(start_color_b)
        .with_spacer(10.0)
        .with_child(Label::new("End Color R"))
        .with_child(end_color_r)
        .with_child(Label::new("End Color G"))
        .with_child(end_color_g)
        .with_child(Label::new("End Color B"))
        .with_child(end_color_b)
        .with_spacer(10.0)
        .with_child(pause_spawn_checkbox)
        .padding(10.0)
        .fix_width(200.0);

    // Область превью
    let preview = super::particle_preview::ParticlePreview::new();

    // Горизонтальная компоновка: левая панель + превью
    Flex::row()
        .with_flex_child(controls, 1.0)
        .with_flex_child(preview, 2.0)
}