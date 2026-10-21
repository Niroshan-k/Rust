use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Task {
    id: usize,
    text: String,
    done: bool,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn Header() -> Element {
    rsx! {
        header { 
            class: "header",
            h1 { "Ferris Tasks "},
            p { 
                class: "subtitle",
                "A tiny task tracker made in dioxus"
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { 
            class: "footer",
            "Built with dioxus"
         }
    }
}

#[component]
fn TaskItem (task: Task) -> Element {
    rsx! {
        li {
            class: "task",
            "{task.text}"

            if task.done {
                span { class: "badge", "done"}
            }
        }
    }
}

#[component]
fn App() -> Element {

    let tasks = use_signal(|| vec! [
        Task {id:0, text:"Learn RSX".to_string(), done:true},
        Task {id:1, text:"Learn RSX".to_string(), done:true},
        Task {id:2, text:"Learn RSX".to_string(), done:true},
    ]);

    let mut count = use_signal(|| 0);
    let mut doublecount = count*2;
    rsx! {
        div {
            class: "app",
            Header {}
            
            h1 { "Count: {count} : Double Count: {doublecount}"}
            button {
                onclick: move |_| count += 1,
                "Add one"
            }

            button {
                onclick: move |_| count.set(0),
                "Reset"
            }

            Footer {}
        }
    }
}





