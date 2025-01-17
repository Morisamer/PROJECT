use eframe::{egui, App};

mod maze;

enum AppState {
    Menu,
    MazeGame(maze::MazeApp),
}

pub struct MyApp {
    state: AppState,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            state: AppState::Menu,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match &mut self.state {
            AppState::Menu => self.show_menu(ctx),
            AppState::MazeGame(app) => app.update(ctx, frame), // Убираем состояние из аргументов
        }
    }
}

impl MyApp {
    fn show_menu(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_rect = ui.available_rect_before_wrap(); // Получаем доступный прямоугольник
            ui.painter().rect_filled(available_rect, 0.0, egui::Color32::from_black_alpha(240));
            ui.vertical_centered(|ui| {
                ui.heading("Меню мини-игр");
                ui.add_space(20.0); // Пробел между заголовком и кнопками
                if ui.button("Лабиринт").clicked() {
                    self.state = AppState::MazeGame(maze::MazeApp::default());
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Мини-игры", options, Box::new(|_cc| Box::new(MyApp::default())))
}
