use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler, MouseButton};

const X_SIZE_CELLS: i32 = 200;
const Y_SIZE_CELLS: i32 = 180;
const CELL_SIZE: i32 = 10;
const DESIRED_FPS: u32 = 5;
const LOAD_PATH: &str = "living_cells.txt";
const FUZZY: f32 = 0.0; // amount to fuzz from loaded cells

fn main() {
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
    prev_cells: Vec<Vec<bool>>,
    max_fps: u32,
    n_ticks: u32,
    paused: bool,
    living_mesh: graphics::Mesh,
    dead_mesh: graphics::Mesh,
    mouse_down: bool,
}

impl GameOfLife {
    pub fn new(ctx: &mut Context) -> GameOfLife {
        let mut initial_cells: Vec<Vec<bool>> = load_cells_from_file(LOAD_PATH);
        
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
        let living_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::BLACK)
            .expect("Failed to create cell mesh");
        let dead_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::WHITE)
            .expect("Failed to create cell mesh");

        GameOfLife {
            cells: initial_cells,
            prev_cells: load_cells_from_file(""),
            max_fps: DESIRED_FPS,
            n_ticks: 0,
            paused: true,
            living_mesh,
            dead_mesh,
            mouse_down: false,
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
        // do not set the background color here, it should come from the current frame
        let mut canvas = graphics::Canvas::from_frame(ctx, None);

        // First frame - draw everything
        if self.n_ticks == 0 {
            for y in 0..(Y_SIZE_CELLS) {
                for x in 0..(X_SIZE_CELLS) {
                    let draw_params = graphics::DrawParam::default()
                        .dest([x as f32 * CELL_SIZE as f32, y as f32 * CELL_SIZE as f32]);
                    
                    if self.cells[y as usize][x as usize] {
                        canvas.draw(&self.living_mesh, draw_params);
                    } else {
                        canvas.draw(&self.dead_mesh, draw_params);
                    }
                }
            }
        } else {
            // Subsequent frames - only draw cells that changed
            for y in 0..(Y_SIZE_CELLS) {
                for x in 0..(X_SIZE_CELLS) {
                    if self.prev_cells[y as usize][x as usize] != self.cells[y as usize][x as usize] {
                        let draw_params = graphics::DrawParam::default()
                            .dest([x as f32 * CELL_SIZE as f32, y as f32 * CELL_SIZE as f32]);
                        
                        if self.cells[y as usize][x as usize] {
                            canvas.draw(&self.living_mesh, draw_params);
                        } else {
                            canvas.draw(&self.dead_mesh, draw_params);
                        }
                    }
                }
            }
        }

        canvas.finish(ctx)
    }
}

impl EventHandler for GameOfLife {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.time.check_update_time(self.max_fps) && !self.paused {
            self.prev_cells = self.cells.clone();
            self.cells = self.compute_next_generation();
        }
        
        // print frame rate
        let fps = ctx.time.fps();
        ctx.gfx.set_window_title(&format!("Life - FPS: {:.0}", fps));
        
        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> GameResult {
        if character == ' ' {
            self.paused = !self.paused;
        }
        Ok(())
    }
    
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
        if button == MouseButton::Left {
            self.mouse_down = true;
            let x = (x / CELL_SIZE as f32).floor() as i32;
            let y = (y / CELL_SIZE as f32).floor() as i32;
            if x >= 0 && x < X_SIZE_CELLS && y >= 0 && y < Y_SIZE_CELLS {
                self.cells[y as usize][x as usize] = true;
            }
        }
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> GameResult {
        if button == MouseButton ::Left {
            self.mouse_down = false;
        }
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) -> GameResult {
        if self.mouse_down {
            let x = (x / CELL_SIZE as f32).floor() as i32;
            let y = (y / CELL_SIZE as f32).floor() as i32;
            if x >= 0 && x < X_SIZE_CELLS && y >= 0 && y < Y_SIZE_CELLS {
                self.cells[y as usize][ x as usize] = true;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_grid(ctx)?;
        Ok(())
    }
}

fn load_cells_from_file(path: &str) -> Vec<Vec<bool>> {
    let file = std::fs::read_to_string(path);
    let mut cells = vec![vec![false; (X_SIZE_CELLS) as usize]; (Y_SIZE_CELLS) as usize];
    
    match file {
        Ok(content) => if path.ends_with(".txt") {
            for line in content.lines() {
                if line.starts_with('#') || line.is_empty() {
                    continue; // Skip comments and empty lines
                }
                let coords: Vec<i32> = line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if coords.len() == 2 {
                    cells[coords[0] as usize][coords[1] as usize] = true;
                }
            }
        } else if path.ends_with(".rle") {
            let start_x = 50;
            let start_y = 50;
            // Handle RLE format
            let mut x = start_x;
            let mut y = start_y;
            for line in content.lines() {
                if line.starts_with('#') || line.is_empty() {
                    continue; // Skip comments and empty lines
                }
                let mut multiplier = 1;
                for c in line.chars() {
                    match c {
                        // dead cells
                        'b' => x += multiplier,
                        // living cells
                        'o' => { 
                            fill_cells(&mut cells, x, y, multiplier);
                            x += multiplier;
                        }
                        '$' => {
                            y += 1;
                            x = start_x;
                        }
                        // an integer followed by 'b' or 'o'
                        '0'..='9' => {
                            multiplier = c.to_digit(10).unwrap() as i32;
                        }
                        _ => {}
                    }
                }
            }
        },
        Err(_) => {
            eprintln!("Error reading file: {}", path);
        }
    };
    cells
}

fn fill_cells(cells: &mut Vec<Vec<bool>>, x: i32, y: i32, len: i32) {
    for i in 0..len {
        cells[y as usize][(x + i) as usize] = true;
    }
}
