use image::GenericImageView;
use eframe::{egui, App};
use eframe::egui::TextureHandle;

mod maze;

enum AppState {
    Menu,
    MazeGame(maze::MazeApp),
}

pub struct MyApp {
    state: AppState,
    background: Option<TextureHandle>, // Для хранения текстуры фона
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

        // Настройка шрифтов
        configure_fonts(ctx);

        match &mut self.state {
            AppState::Menu => self.show_menu(ctx),
            AppState::MazeGame(app) => app.update(ctx, frame),
        }
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
            let menu_height = 200.0;
            let centered_rect = egui::Rect::from_center_size(
                ui.available_rect_before_wrap().center(),
                egui::vec2(menu_width, menu_height),
            );

            ui.allocate_ui_at_rect(centered_rect, |ui| {
                ui.vertical_centered(|ui| {
                    // Настройка заголовка с применением шрифтов
                    ui.label(
                        egui::RichText::new("Меню мини-игр")
                            .font(egui::FontId::new(32.0, egui::FontFamily::Proportional))
                            .color(egui::Color32::YELLOW),
                    );
                    ui.add_space(30.0);

                    // Настройка кнопки с применением шрифтов
                    let button = egui::Button::new(
                        egui::RichText::new("Лабиринт")
                            .font(egui::FontId::new(24.0, egui::FontFamily::Proportional)),
                    )
                    .min_size(egui::vec2(200.0, 60.0))
                    .fill(egui::Color32::DARK_GRAY)
                    .stroke(egui::Stroke::new(2.0, egui::Color32::BLACK));

                    if ui.add(button).clicked() {
                        self.state = AppState::MazeGame(maze::MazeApp::default());
                    }
                });
            });
        });
    }
}

fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Загружаем шрифт
    let font_data = include_bytes!(r"C:\Users\SystemX\dijkstra_maze\pixel_font.ttf");
    fonts.font_data.insert(
        "pixel_font".to_owned(),
        egui::FontData::from_static(font_data),
    );

    // Указываем, что шрифт `pixel_font` будет использоваться как пропорциональный
    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "pixel_font".to_owned());

    // Указываем шрифт для монопространственных шрифтов (опционально)
    fonts.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("pixel_font".to_owned());

    ctx.set_fonts(fonts);
}

fn load_background(ctx: &egui::Context) -> TextureHandle {
    let image_data = include_bytes!(r"C:\Users\SystemX\dijkstra_maze\hz.png");
    let image = image::load_from_memory(image_data).expect("Failed to load image");
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
