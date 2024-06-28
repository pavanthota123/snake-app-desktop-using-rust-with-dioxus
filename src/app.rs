use dioxus::prelude::*;
use rand::Rng;


use crate::display_grid::DisplayGrid;
use crate::grid::initialize_grid;
use crate::direction::{Direction, DirectionControls};
use crate::game_logic::{
    get_random_number, update_snake_position, check_collision,
    update_score, reset_game_state
};



#[component]
pub fn app() -> Element {
    let mut grid = use_signal(Vec::new);
    let mut tail_pos = use_signal(|| 0u32);
    let mut score = use_signal(|| 0);
    let mut rng_r = rand::thread_rng();
    let mut snake_positions = use_signal(Vec::new);
    let mut m = use_signal(|| rng_r.gen_range(0..15));
    let mut n = use_signal(|| rng_r.gen_range(0..15));
    let mut is_grid_initialized = use_signal(|| false);
    let mut head = use_signal(|| (0, 2));
    let mut is_game_over = use_signal(|| false);
    let mut game_over = use_signal(|| "");
    let mut direction = use_signal(|| Direction::Right);

    initialize_grid(&mut grid, &mut snake_positions, &m, &n, &mut is_grid_initialized);
    let grid_copy = grid.read().clone();

    use_future(move || async move {
        loop {
            let now = tokio::time::Instant::now();
            tokio::time::sleep_until(now + std::time::Duration::from_millis(500)).await;
            let (mut row_head, mut col_head) = *head.read();

            update_snake_position(&mut row_head, &mut col_head, *direction.read());

            if check_collision(row_head, col_head, &grid.read()) {
                game_over.set("Game Over!");
                is_game_over.set(true);
                *head.write() = (16, 16);
                continue;
            }

            grid.write()[row_head][col_head] = 1;
            snake_positions.write().push(vec![row_head as i32, col_head as i32]);

            let scored = update_score(&mut score.write(), row_head, col_head, *m.read(), *n.read());
            if scored {
                m.set(get_random_number() as usize);
                n.set(get_random_number() as usize);
                grid.write()[*m.read()][*n.read()] = 5;
            } else {
                let temp_tailpos = *tail_pos.read() as usize;
                let row_tail = snake_positions.read()[temp_tailpos][0] as usize;
                let col_tail = snake_positions.read()[temp_tailpos][1] as usize;
                if grid.read()[row_tail][col_tail] == 1 {
                    grid.write()[row_tail][col_tail] = 0;
                }
                tail_pos += 1;
            }
            *head.write() = (row_head, col_head);
        }
    });

    let reset_game = move |_| {
        reset_game_state(
            &mut grid.write(),
            &mut snake_positions.write(),
            &mut m.write(),
            &mut n.write(),
            &mut head.write(),
            &mut tail_pos.write(),
            &mut score.write(),
            &mut direction.write(),
            &mut is_game_over.write(),
        );
        game_over.set("");
    };

    rsx! {
        link { rel: "stylesheet", href: "main.css" }

        div {
            DisplayGrid {
                grid: grid_copy,
                game_over: *is_game_over.read(),
                score: *score.read()
            }
        }
        DirectionControls {
            direction: direction,
            is_game_over: is_game_over,
            on_reset: reset_game
        }
    }
}