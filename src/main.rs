use dioxus::prelude::*;

fn main() {
    launch(app);
}

#[derive(Props, Clone, PartialEq)]
struct CustomProps {
    text: String
}

fn app() -> Element {
    rsx! {
        Notes { text: "hello" }
        div {
            style: " height: 200px; width: 200px; background-color: red;",
            "Hello"
        }
        Notes { text: "Sabaidee" }
    }
}


#[component]
fn Notes(props: CustomProps) -> Element {
    rsx! {
        p { "{props.text}" }
    }
}
