use eframe::egui;
use rand::{thread_rng, Rng, seq::SliceRandom};
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    row: usize,
    col: usize,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct MyApp {
    grid: Vec<Vec<bool>>,        // true = стена (белый), false = проход (черный)
    path: Vec<(usize, usize)>,   // Список координат для пути
    grid_size: usize,
    path_color: egui::Color32,
    wall_color: egui::Color32,
    passage_color: egui::Color32,
    entrance_row: usize,
    exit_row: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        let grid_size = 21; // Размер сетки (нечетное число для правильной работы алгоритма)
        let grid = vec![vec![false; grid_size]; grid_size]; // Инициализация сетки (все проходы)
        MyApp {
            grid,
            path: Vec::new(),
            grid_size,
            path_color: egui::Color32::GREEN,
            wall_color: egui::Color32::WHITE,
            passage_color: egui::Color32::BLACK,
            entrance_row: 1,        // Строка для входа по умолчанию
            exit_row: grid_size - 2 // Строка для выхода по умолчанию
        }
    }
}

impl MyApp {
    fn generate_maze(&mut self) {
        loop {
            let rows = self.grid_size;
            let cols = self.grid_size;
    
            self.grid = vec![vec![true; cols]; rows];
    
            let mut rng = thread_rng();
            self.entrance_row = rng.gen_range(1..rows - 1);
            self.exit_row = rng.gen_range(1..rows - 1);
    
            self.grid[self.entrance_row][0] = false; 
            self.grid[self.exit_row][cols - 1] = false;
    
            let start_row = 1;
            let start_col = 1;
            self.grid[start_row][start_col] = false; 
            let mut stack = Vec::new();
            stack.push((start_row, start_col));
    
            while let Some((row, col)) = stack.pop() {
                let mut neighbors = vec![
                    (row.wrapping_sub(2), col, row.wrapping_sub(1), col), 
                    (row + 2, col, row + 1, col), 
                    (row, col.wrapping_sub(2), row, col.wrapping_sub(1)), 
                    (row, col + 2, row, col + 1), 
                ];
                neighbors.shuffle(&mut rng);
    
                for (nr, nc, wr, wc) in neighbors {
                    if nr > 0 && nr < rows && nc > 0 && nc < cols && self.grid[nr][nc] {
                        self.grid[nr][nc] = false; 
                        self.grid[wr][wc] = false; 
                        stack.push((nr, nc));
                    }
                }
            }
    

            let path = self.dijkstra((self.entrance_row, 0), (self.exit_row, self.grid_size - 1));
            if path.is_empty() {
                continue;
            }

            let entrance_cell = (self.entrance_row, 0);
            let exit_cell = (self.exit_row, self.grid_size - 1);
            let valid_path = path.first() == Some(&entrance_cell) && path.last() == Some(&exit_cell);
            if !valid_path {
                continue; 
            }
    
            break; 
        }
    
        self.path.clear(); 
    }
    fn solve_maze(&mut self) {
        let start = (self.entrance_row, 0); // Вход
        let end = (self.exit_row, self.grid_size - 1); // Выход
        self.path = self.dijkstra(start, end);

        // Проверка, что путь найден
        if self.path.is_empty() {
            println!("Путь не найден! Возможно, лабиринт некорректен.");
        }
    }

    fn dijkstra(&self, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
        let mut heap = BinaryHeap::new();
        let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let (start_row, start_col) = start;
        distances.insert(start, 0);
        heap.push(Node { row: start_row, col: start_col, cost: 0 });

        while let Some(Node { row, col, cost }) = heap.pop() {
            if (row, col) == end {
                break; // Достигли конечной точки
            }

            let neighbors = [
                (row.wrapping_sub(1), col), // Верхний
                (row + 1, col),            // Нижний
                (row, col.wrapping_sub(1)), // Левый
                (row, col + 1),            // Правый
            ];

            for &(n_row, n_col) in &neighbors {
                if n_row < self.grid.len() && n_col < self.grid[0].len() && !self.grid[n_row][n_col] {
                    let next_cost = cost + 1;
                    if next_cost < *distances.get(&(n_row, n_col)).unwrap_or(&usize::MAX) {
                        distances.insert((n_row, n_col), next_cost);
                        came_from.insert((n_row, n_col), (row, col));
                        heap.push(Node { row: n_row, col: n_col, cost: next_cost });
                    }
                }
            }
        }

        let mut current = end;
        let mut path = Vec::new();
        while let Some(&prev) = came_from.get(&current) {
            path.push(current);
            current = prev;
        }
        path.push(start);
        path.reverse();
        path
    }
}

pub struct MazeApp {
    inner: MyApp, // Обертка для основной логики
}

impl Default for MazeApp {
    fn default() -> Self {
        MazeApp {
            inner: MyApp::default(),
        }
    }
}

impl eframe::App for MazeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    if ui.button("Сгенерировать лабиринт").clicked() {
                        self.inner.path.clear(); // Очищаем путь при генерации нового лабиринта
                        self.inner.generate_maze();
                    }

                    if ui.button("Решить лабиринт").clicked() {
                        self.inner.solve_maze();
                    }

                    ui.separator();

                    ui.label("Размер лабиринта:");
                    let mut new_size = self.inner.grid_size;
                    if ui.add(egui::Slider::new(&mut new_size, 5..=51)).changed() {
                        self.inner.grid_size = if new_size % 2 == 0 { new_size + 1 } else { new_size };
                        self.inner.path.clear();
                        self.inner.generate_maze();
                    }

                    ui.label("Цвет пути:");
                    ui.color_edit_button_srgba(&mut self.inner.path_color);

                    ui.label("Цвет стен:");
                    ui.color_edit_button_srgba(&mut self.inner.wall_color);

                    ui.label("Цвет прохода:");
                    ui.color_edit_button_srgba(&mut self.inner.passage_color);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let cell_size = (available_size.x.min(available_size.y) / self.inner.grid_size as f32).max(1.0);
            let total_maze_size = cell_size * self.inner.grid_size as f32;
            let offset_x = (available_size.x - total_maze_size).max(0.0) / 2.0;
            let offset_y = (available_size.y - total_maze_size).max(0.0) / 2.0;

            for row in 0..self.inner.grid_size {
                for col in 0..self.inner.grid_size {
                    let color = if self.inner.grid[row][col] {
                        self.inner.wall_color // Стена
                    } else if self.inner.path.contains(&(row, col)) {
                        self.inner.path_color // Путь
                    } else {
                        self.inner.passage_color // Проход
                    };

                    ui.painter().rect_filled(
                        egui::Rect::from_min_size(
                            egui::Pos2::new(offset_x + col as f32 * cell_size, offset_y + row as f32 * cell_size),
                            egui::vec2(cell_size, cell_size),
                        ),
                        0.0,
                        color,
                    );
                }
            }
        });
    }
}
