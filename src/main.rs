use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

const X_SIZE_CELLS: i32 = 200;
const Y_SIZE_CELLS: i32 = 180;
const CELL_SIZE: i32 = 10;
const DESIRED_FPS: u32 = 20;
const LOAD_PATH: &str = "living_cells.txt";
const FUZZY: f32 = 0.5; // amount to fuzz from loaded cells

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Life", "Joshua Himmens")
        .window_setup(ggez::conf::WindowSetup::default().title("Life"))
        .window_mode(ggez::conf::WindowMode::default()
            .dimensions(
                (X_SIZE_CELLS * CELL_SIZE) as f32,
                (Y_SIZE_CELLS * CELL_SIZE) as f32))
        .build()
        .expect("Error creating ggez context, panic!");

    let my_game = GameOfLife::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct GameOfLife {
    cells: Vec<Vec<bool>>,
    max_fps: u32,
    n_ticks: u32,
    paused: bool,
    cell_mesh: graphics::Mesh,
}

impl GameOfLife {
    pub fn new(ctx: &mut Context) -> GameOfLife {
        let mut initial_cells: Vec<Vec<bool>> = vec![vec![false; (X_SIZE_CELLS) as usize]; (Y_SIZE_CELLS) as usize];
        for (cell_x, cell_y) in load_cells_from_file(LOAD_PATH) {
            initial_cells[(cell_y) as usize][(cell_x) as usize] = true;
        }

        // Fuzz the cells a bit
        for y in 0..(Y_SIZE_CELLS) {
            for x in 0..(X_SIZE_CELLS) {
                // if random number is less than FUZZY, flip the cell
                if rand::random::<f32>() < FUZZY {
                    initial_cells[y as usize][x as usize] = !initial_cells[y as usize][x as usize];
                }
            }
        }
        
        let rect = graphics::Rect::new(0.0, 0.0, CELL_SIZE as f32, CELL_SIZE as f32);
        let cell_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::BLACK)
            .expect("Failed to create cell mesh");

        GameOfLife {
            cells: initial_cells,
            max_fps: DESIRED_FPS,
            n_ticks: 0,
            paused: true,
            cell_mesh,
        }
    }

    fn count_alive_neighbors(&self, x: i32, y: i32) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // Skip the cell itself
                }
                let nx = (x + dx + X_SIZE_CELLS) % X_SIZE_CELLS;
                let ny = (y + dy + Y_SIZE_CELLS) % Y_SIZE_CELLS;
                if self.cells[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
        count
    }

    fn compute_next_generation(&mut self) -> Vec<Vec<bool>> {
        let mut new_cells = self.cells.clone();
        for y in 0..(Y_SIZE_CELLS) {
            for x in 0..(X_SIZE_CELLS) {
                let alive_neighbors = self.count_alive_neighbors(x, y);
                if self.cells[y as usize][x as usize] {
                    new_cells[y as usize][x as usize] = alive_neighbors == 2 || alive_neighbors == 3;
                } else {
                    new_cells[y as usize][x as usize] = alive_neighbors == 3;
                }
            }
        }
        self.n_ticks += 1;
        new_cells
    }

    fn draw_grid(&self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        for y in 0..(Y_SIZE_CELLS) {
            for x in 0..(X_SIZE_CELLS) {
                if self.cells[y as usize][x as usize] {
                    let draw_params = graphics::DrawParam::default()
                        .dest([x as f32 * CELL_SIZE as f32, y as f32 * CELL_SIZE as f32]);
                    
                    canvas.draw(&self.cell_mesh, draw_params);
                }
            }
        }

        canvas.finish(ctx)
    }
}

impl EventHandler for GameOfLife {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.time.check_update_time(self.max_fps) && !self.paused {
            self.cells = self.compute_next_generation();
        }
        
        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> GameResult {
        if character == ' ' {
            self.paused = !self.paused;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_grid(ctx)?;
        Ok(())
    }
}

fn load_cells_from_file(path: &str) -> Vec<(i32, i32)> {
    let file = std::fs::read_to_string(path).expect("Failed to read file");
    let mut cells = vec![];
    
    if path.ends_with(".txt") {
        for line in file.lines() {
            let coords: Vec<i32> = line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if coords.len() == 2 {
                cells.push((coords[0], coords[1]));
            }
        }
    } else if path.ends_with(".rle") { 
        // custom life format
        
        
    }
    cells
}