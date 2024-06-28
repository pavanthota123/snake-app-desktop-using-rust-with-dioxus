pub mod app;
pub mod display_grid;
pub mod direction;
pub mod grid;
pub mod game_logic;

#[cfg(test)]
mod tests {
    use crate::game_logic::get_random_number;
    use crate::game_logic::update_score;
    use crate:: game_logic::check_collision;
    use crate::game_logic::update_snake_position;
    use crate::direction::Direction;
    use crate::game_logic::reset_game_state;

    #[test]
    fn test_get_random_number() {
        for _ in 0..1000 {
            let number = get_random_number();
            assert!(number < 15);   //checking if the random number is less than 15 no need to check greater than zero. 
        }
        }

        #[test]
        fn test_check_collision() {
            let mut grid = vec![vec![0; 15]; 15];
            assert!(check_collision(15, 7, &grid));  //edge checking
            grid[5][5] = 1;
            assert!(check_collision(5, 5, &grid));   //collision with snake body
        }
    
        #[test]
        fn test_update_score() {
            let mut score = 0;
           //food and Position Match Score should increase
            assert!(update_score(&mut score, 3, 3, 3, 3));
            assert_eq!(score, 1);
            assert!(!update_score(&mut score, 4, 4, 3, 3));
            assert_eq!(score, 1);
        }

        #[test]
        fn test_update_snake_position() {
            let mut row = 5;
            let mut col = 5;

            update_snake_position(&mut row, &mut col, Direction::Left); // checking if snake is moving left
            assert_eq!(col, 4);

            row = 15;
            update_snake_position(&mut row, &mut col, Direction::Up);   //checking if snake is moving up
            assert_eq!(row, 14);
    
            col = 5;
            update_snake_position(&mut row, &mut col, Direction::Right); //checking if snake is moving right
            assert_eq!(col, 6);
    
            row = 5;
            update_snake_position(&mut row, &mut col, Direction::Down); //checking if snake is moving down
            assert_eq!(row, 6);
        }
    
        #[test]
        fn test_reset_game_state() {
            let mut grid = vec![vec![1; 15]; 15];  
            let mut snake_positions = vec![vec![5, 5], vec![5, 6], vec![5, 7]];
            let mut m = 10;
            let mut n = 10;
            let mut head = (5, 7);
            let mut tail_pos = 0;
            let mut score = 10;
            let mut direction = Direction::Left;
            let mut is_game_over = true;
    
            reset_game_state(
                &mut grid,
                &mut snake_positions,
                &mut m,
                &mut n,
                &mut head,
                &mut tail_pos,
                &mut score,
                &mut direction,
                &mut is_game_over,
            );
    
            assert_eq!(grid[0][0], 1);
            assert_eq!(grid[0][1], 1);
            assert_eq!(grid[0][2], 1);
            assert_eq!(snake_positions, vec![vec![0, 0], vec![0, 1], vec![0, 2]]);  //checking the snake positions again after the reset
    
            // Check other state variables if they are updated corectly after reset.
            assert_eq!(head, (0, 2));
            assert_eq!(tail_pos, 0);
            assert_eq!(score, 0);
            assert_eq!(direction, Direction::Right);
            assert_eq!(is_game_over, false);
        }
    
}