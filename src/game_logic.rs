use rand::Rng;
use crate::direction::Direction;

pub fn get_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..15)
}

pub fn update_snake_position(
    row_head: &mut usize,
    col_head: &mut usize,
    direction: Direction,
) {
    match direction {
        Direction::Left => {
            if *col_head == 0 {
                *col_head = 15;
            } else {
                *col_head -= 1;
            }
        },
        Direction::Up => {
            if *row_head == 0 {
                *row_head = 15;
            } else {
                *row_head -= 1;
            }
        },
        Direction::Right => {
            *col_head += 1;
        },
        Direction::Down => {
            *row_head += 1;
        },
    }
}

pub fn check_collision(
  row_head: usize,
  col_head: usize,
  grid: &[Vec<u32>],
) -> bool {
  col_head > 14 || row_head > 14 || grid[row_head][col_head] == 1
}

pub fn update_score(score: &mut i32,row_head: usize,col_head: usize,m: usize,n: usize,) -> bool {
    if m == row_head && n == col_head {
        *score += 1;
        true
    } else {
        false
    }
}


pub fn reset_game_state( grid: &mut [Vec<u32>],snake_positions: &mut Vec<Vec<i32>>,m: &mut usize,n: &mut usize,head: &mut (usize, usize), tail_pos: &mut u32,score: &mut i32, direction: &mut Direction, is_game_over: &mut bool,) {
    grid.iter_mut().for_each(|row| {
        row.iter_mut().take(15).for_each(|cell| {
            *cell = 0;
        });
    });
    grid[0][0] = 1;
    grid[0][1] = 1;
    grid[0][2] = 1;
    snake_positions.clear();
    snake_positions.push(vec![0, 0]);
    snake_positions.push(vec![0, 1]);
    snake_positions.push(vec![0, 2]);
    *m = get_random_number() as usize;
    *n = get_random_number() as usize;
    grid[*m][*n] = 5;
    *head = (0, 2);
    *tail_pos = 0;
    *score = 0;
    *direction = Direction::Right;
    *is_game_over = false;
}