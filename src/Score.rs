use dioxus::prelude::*;
#[component]

pub fn Controls(is_game_over: bool) -> Element {

  rsx!{
    div {
      style: "color: white;",
      "hello"
    }

    button {
      id: "4", 
        onclick: move |_| {
            if(direction !=3 && !is_game_over){
            direction = 1;
            }
            },
           "\u{2191}"                      //down Button
        }
  }
}