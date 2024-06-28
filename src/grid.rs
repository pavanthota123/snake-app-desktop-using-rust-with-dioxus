use dioxus::prelude::*;
pub fn initialize_grid(
    grid: &mut Signal<Vec<Vec<u32>>>,
    snake_positions: &mut Signal<Vec<Vec<i32>>>,
    m: &Signal<usize>,
    n: &Signal<usize>,
    is_grid_initialized: &mut Signal<bool>
) {
    if !*is_grid_initialized.read() {
        let mut new_grid = vec![vec![0; 15]; 15];
        new_grid[0][0] = 1;
        new_grid[0][1] = 1;
        new_grid[0][2] = 1;
        new_grid[*m.read()][*n.read()] = 5;
        grid.set(new_grid);
        snake_positions.set(vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, 2],
        ]);

        is_grid_initialized.set(true);
    }
}