use dioxus::prelude::*;

#[component]
pub fn DisplayGrid(grid: Vec<Vec<u32>>, game_over: bool, score: i32) -> Element {
    rsx! {
        div {
            class: "score-container",
            "Score: {score}"
        }

        div {
            style: "position: relative; width: 450px; height: 500px;",
            div {
                class: "white_text",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;",
                if game_over {
                    "Game Over!"
                }
            }
            table {
                style: "border-collapse: collapse; width: 100%; height: 100%; border: 1px solid white;",
                for i in 0..15 {
                    tr {
                        style: "height: 30px;",
                        for j in 0..15 {
                            if grid[i][j] == 0 {
                                td { style: "height: 30px; width: 30px; background-color:blue;" }
                            } else if grid[i][j] == 1 {
                                td { style: "height: 30px; width: 30px; background-color:green;" }
                            } else {
                                td { style: "height: 30px; width: 30px; background-color:red;" }
                            }
                        }
                    }
                }
            }
        }
    }
}
