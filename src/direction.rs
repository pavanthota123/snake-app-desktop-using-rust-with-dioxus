use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down, 
}

pub fn direction_to_i32(direction: Direction) -> i32 {
    match direction {
        Direction::Left => 0,
        Direction::Up => 1,
        Direction::Right => 2,
        Direction::Down => 3,
    }
}

#[component]

pub fn DirectionControls(direction: Signal<Direction>, is_game_over: Signal<bool>,on_reset: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "arrow-buttons",
            button {
                class: "right",
                onclick: move |_| {
                    if *direction.read() != Direction::Left && !*is_game_over.read() {
                        direction.set(Direction::Right);
                    }
                },
                "\u{2192}" // Right Button
            }
            button {
                class: "left",
                onclick: move |_| {
                    if *direction.read() != Direction::Right && !*is_game_over.read() {
                        direction.set(Direction::Left);
                    }
                },
                "\u{2190}" // Left Button
            }
            button {
                class: "down",
                onclick: move |_| {
                    if *direction.read() != Direction::Up && !*is_game_over.read() {
                        direction.set(Direction::Down);
                    }
                },
                "\u{2193}" // Down Button
            }
            button {
                class: "up",
                onclick: move |_| {
                    if *direction.read() != Direction::Down && !*is_game_over.read() {
                        direction.set(Direction::Up);
                    }
                },
                "\u{2191}" // Up Button
            }
            button {
              class: "reset",
              onclick: move |_| on_reset.call(()),
              "\u{21BA}" // Reset button
          }
        }
    }
}
