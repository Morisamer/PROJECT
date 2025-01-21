use piston_window::types::Color;
use eframe::{egui};
use eframe::egui::TextureHandle;
use std::process::Command;
extern crate piston_window;
extern crate rand;
const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
use drawing::to_gui_coord_u32;
mod maze;
mod game; 
mod drawing; 
mod snake;
enum AppState {
    Menu,
    MazeGame(maze::MazeApp),
}

pub struct MyApp {
    state: AppState,
    background: Option<TextureHandle>, 
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            state: AppState::Menu,
            background: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.background.is_none() {
            self.background = Some(load_background(ctx));
        }

        configure_fonts(ctx);

        match &mut self.state {
            AppState::Menu => self.show_menu(ctx),
            AppState::MazeGame(app) => app.update(ctx, frame),
            AppState::SnakeGame => {
                
                run_snake_game();
                self.state = AppState::Menu; 
            }
        }
    }
}

fn run_snake_game() {
    use piston_window::*; 
    let (width, height) = (20, 20);
    let mut window_settings = piston_window::WindowSettings::new(
        "Rust Snake",
        [to_gui_coord_u32(width), to_gui_coord_u32(height)],
    ).exit_on_esc(true);
    window_settings.set_vsync(true);
    let mut window: PistonWindow = window_settings.build().unwrap();
    let mut game = game::Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}

impl MyApp {
    fn show_menu(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(background) = &self.background {
                let available_rect = ui.available_rect_before_wrap();
                ui.painter().image(
                    background.id(),
                    available_rect,
                    egui::Rect::from_min_max([0.0, 0.0].into(), [1.0, 1.0].into()),
                    egui::Color32::WHITE,
                );
            }

            let menu_width = 300.0;
            let menu_height = 300.0;
            let centered_rect = egui::Rect::from_center_size(
                ui.available_rect_before_wrap().center(),
                egui::vec2(menu_width, menu_height),
            );

            ui.allocate_ui_at_rect(centered_rect, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Меню мини-игр")
                            .font(egui::FontId::new(32.0, egui::FontFamily::Proportional))
                            .color(egui::Color32::YELLOW),
                    );
                    ui.add_space(30.0);

                    let maze_button = egui::Button::new(
                        egui::RichText::new("Лабиринт")
                            .font(egui::FontId::new(24.0, egui::FontFamily::Proportional)),
                    )
                    .min_size(egui::vec2(200.0, 60.0))
                    .fill(egui::Color32::DARK_GRAY)
                    .stroke(egui::Stroke::new(2.0, egui::Color32::BLACK));

                    if ui.add(maze_button).clicked() {
                        self.state = AppState::MazeGame(maze::MazeApp::default());
                    }

                    ui.add_space(20.0);

                    let snake_button = egui::Button::new(
                        egui::RichText::new("Змейка")
                            .font(egui::FontId::new(24.0, egui::FontFamily::Proportional)),
                    )
                    .min_size(egui::vec2(200.0, 60.0))
                    .fill(egui::Color32::DARK_GRAY)
                    .stroke(egui::Stroke::new(2.0, egui::Color32::BLACK));

                    if ui.add(snake_button).clicked() {
                        Command::new("snake_game.exe")
                        .spawn()
                        .expect("Не удалось запустить игру Змейка");
                    }
                });
            });
        });
    }
}

fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    let font_data = include_bytes!(r"C:\Users\SystemX\dijkstra_maze\pixel_font.ttf");
    fonts.font_data.insert(
        "pixel_font".to_owned(),
        egui::FontData::from_static(font_data),
    );

    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "pixel_font".to_owned());

    fonts.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("pixel_font".to_owned());

    ctx.set_fonts(fonts);
}

fn load_background(ctx: &egui::Context) -> TextureHandle {
    let image_data = include_bytes!(r"C:\Users\SystemX\dijkstra_maze\hz.png");
    let image = ::image::load_from_memory(image_data).expect("Не удалось загрузить изображение");
    let image = image.into_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();

    ctx.load_texture(
        "background",
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        egui::TextureOptions::default(),
    )
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Мини-игры", options, Box::new(|_cc| Box::new(MyApp::default())))
}
