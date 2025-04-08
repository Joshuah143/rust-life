# Conways Game of Life

This project was an experience in writing high performance rust code. 
It implements Conway's Game of Life using `ggez` and a 2D vector of cells.
The board is a 2D projection of a toroidal grid, as the cells wrap around the edges of the grid.



## Running the project

1. [Install Rust and Cargo](https://www.rust-lang.org/tools/install)
2. Clone the repository
3. From the project root, run `cargo run` to start the game

Once started, the game can be paused and unpaused by pressing the `space` key.
Using the left mouse button will make any cells it touches alive.


### Configuration

Most of the configuration is done through constants in the `src/main.rs` file.
- The size of the grid is defined by `{DIM}_SIZE_CELLS` (in number of cells)
- The size of each cell is defined by `CELL_SIZE` (in pixels, square)
- The max speed of simulation `DESIRED_FPS`
- The portion of random cells to be generated at the start of the simulation `FUZZY` on [0, 1]
- The path to the file to load the initial state of the grid `LOAD_PATH`
  - Both `.txt` and `.rle` files are supported.
    - `.txt` files are a list of coordinates of cells to be alive
    - `.rle` files are a compressed format of the grid, these files can be found online

## Key learnings

1. Using a persistent `Mesh`
    - The instantiation and drawing of the mesh is computationally expensive, so much so that creating a new mesh for each living cell limited the FPS to under 3 FPS with `FUZZY=0.5`.
    - Instead, a single mesh is created for the entire grid, and the cells are drawn using `draw_instanced` with a `DrawParam` for each cell. This improved performance to 10 FPS with `FUZZY=0.5`.
2. Drawing only changed cells
    - The `draw_instanced` function is called for each cell, but only the cells that have changed are drawn. This is done by keeping track of the previous state of the grid and only drawing the cells that have changed.
    - This optimization is debatable useful as it requires more state information, including the last state and multiple meshes. This approach also does not allow for coloring cells based on cell age or other properties.

## Avenues for improvement

- Identify cyclic patterns of cells in the grid
  - This is marginally more complicated because the toroidal grid wraps around the edges, which means that patterns that would otherwise be stable may self-interact of move around the grid.
  - The likely first implementation would be to identify a hash function for the grid and attempt to match the hash to a known pattern.
- Coloring cells based on age
  - Cells would be colored based on the number of generations since last activation.
  - This would require a more complex data structure to keep track of the age of each cell.
- Move the config to a config file
  - This would allow for easier configuration of the simulation without having to recompile the code.
  - This would also allow for different configurations to be loaded at runtime, which would be useful for testing different configurations.

## Use of GenAI tools

This project has been created by hand. 
GenAI tools were used to assist in the development of the project, for code review, ideation, and rust familiarization.