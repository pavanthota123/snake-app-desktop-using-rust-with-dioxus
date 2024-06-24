use dioxus::prelude::*;
use dioxus_elements::r#use;
use rand::Rng;
use std::vec;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut grid = use_signal(|| Vec::new());
    let mut tail_pos = use_signal(|| 0u32);
    let mut score: Signal<i32> = use_signal(|| 0);
    let mut rng_r = rand::thread_rng();
    let mut snake_positions = use_signal(|| Vec::new());
    let mut m = use_signal(|| rng_r.gen_range(0..15));
    let mut n = use_signal(|| rng_r.gen_range(0..15));
    let mut is_grid_initialized = use_signal(|| false);
    let mut millis = use_signal(|| 0);
    let mut head = use_signal(|| (0, 2));
    let mut direction: Signal<i32> = use_signal(|| 2);
    let mut is_game_over = use_signal(|| false);
    let mut game_over = use_signal(|| "");

    let (mut row, mut col) = *head.read();
    if !*is_grid_initialized.read() {
        for i in 0..15 {
            let mut row: Vec<u32> = vec![0; 15];
            for j in 0..15 {
                row[j] = 0;
            }
            grid.push(row);
        }
        grid.write()[0][0] = 1;
        grid.write()[0][1] = 1;
        grid.write()[0][2] = 1;
        snake_positions.write().push(vec![0, 0]);
        snake_positions.write().push(vec![0, 1]);
        snake_positions.write().push(vec![0, 2]);
        grid.write()[*m.read()][*n.read()] = 5;
        is_grid_initialized.set(true);
    }

    use_future(move || async move {
        let start = std::time::Instant::now();
        loop {
            let now = tokio::time::Instant::now();
            tokio::time::sleep_until(now + std::time::Duration::from_millis(500)).await;
            let (mut row_head, mut col_head) = *head.read();
            //auto move right
            let dir = *direction.read();
            if dir == 0 {
                if col_head == 0 {
                    col_head = 15;
                } else {
                    col_head -= 1;
                }
            } else if dir == 1 {
                //up
                if row_head == 0 {
                    row_head = 15;     //Edge Cases
                } else {
                    row_head -= 1;
                }
            } else if dir == 2 {
                //right
                col_head += 1;
            } else if dir == 3 {
                //down
                row_head += 1;
            }
            if col_head > 14 || row_head > 14 {
                game_over.set("Game Over!");
                is_game_over.set(true);
                continue;
            } else {
                if grid.read()[row_head][col_head] == 1 {
                    game_over.set("Game Over!");
                    is_game_over.set(true);
                    *head.write() = (16, 16);
                    continue;
                }
                grid.write()[row_head][col_head] = 1;
            }
            snake_positions
                .write()
                .push(vec![row_head as i32, col_head as i32]);

            let m_local = *m.read();
            let n_local = *n.read();
            let mut scored = false;
            if *m.read() == row_head && *n.read() == col_head {
                score += 1;
                m.set((101*row_head+876512)%15);
                n.set(m_local);
                grid.write()[*m.read()][*n.read()] = 5;
                //increase length
                scored = true;
            }
            *head.write() = (row_head, col_head);
            //moving tail
            if !scored {
                let temp_tailpos = *tail_pos.read() as usize;
                let row_tail = snake_positions.read()[temp_tailpos][0] as usize;
                let col_tail = snake_positions.read()[temp_tailpos][1] as usize;
                if grid.read()[row_tail][col_tail] == 1 {
                    grid.write()[row_tail][col_tail] = 0;
                }
                tail_pos += 1;
            }
        }
    });

    rsx! {

        link { rel: "stylesheet", href: "main.css" }

        div {
            class: "score-container",
            "Score: {score}"
          }
        div {
            style: "position: relative; width: 450px; height: 500px;",
            div {
                class: "white_text",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;",
                "{game_over}"
            }
            table {
                style: "border-collapse: collapse; width: 100%; height: 100%; border: 1px solid white;",
                for i in 0..15 {
                    tr {
                        style: "height: 30px;",
                        for j in 0..15 {
                            if grid.read()[i][j] == 0 {
                                td {style: "height: 30px; width: 30px; background-color:blue;",}
                            } else if grid.read()[i][j] == 1 {
                                td {style: "height: 30px; width: 30px; background-color:green;",}
                            } else {
                                td {style: "height: 30px; width: 30px; background-color:red;",}
                            }
                        }
                    }
                }
            }
        }

        div {
            class: "arrow-buttons",
        button { class: "right",
        onclick: move |_| {
            if(*direction.read()!=0 && !*is_game_over.read()){
                    direction.set(2);
                }
            },
            "\u{2192}"      //Right Button
        }

        button { class: "left",
        onclick: move |_| {
            if(*direction.read()!=2 && !*is_game_over.read()){
                direction.set(0);

            }
            },
            "\u{2190}"        //left button
        }

        button {class: "down",
            onclick: move |_| {
                if(*direction.read()!=1&& !*is_game_over.read() ){
                    direction.set(3);

                }

            },
           "\u{2193}"                      //down Button
        }

        button { class: "up",
        onclick: move |_| {
            if(*direction.read()!=3 && !*is_game_over.read()){
            direction.set(1);
            }
            },
            "\u{2191}"                            //UP Button
        }

        button { onclick: move |_| {
            for i in 0..15 {
                for j in 0..15 {
                    grid.write()[i][j] = 0;
                }
            }
            grid.write()[0][0] = 1;
            grid.write()[0][1] = 1;
            grid.write()[0][2] = 1;
            // snake_positions.write().removeAll();
            snake_positions.write().push(vec![0, 0]);
            snake_positions.write().push(vec![0, 1]);
            snake_positions.write().push(vec![0, 2]);
            m.set(rng_r.gen_range(0..15));
            n.set(rng_r.gen_range(0..15));
            grid.write()[*m.read()][*n.read()] = 5;
            *head.write() = (0, 2);
            tail_pos.set(snake_positions.read().len() as u32 - 3);
            score.set(0);
            direction.set(2);
            is_game_over.set(false);
            game_over.set("");
        },
        "\u{21BA}"  //reset button
    }
        
    }
      
    }
}
