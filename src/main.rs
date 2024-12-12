use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info, error};

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    launch(app);
}

#[derive(Props, Clone, PartialEq)]
struct CustomProps {
    text: String,
    #[props(default = 0, optional)]
    size: i32
}

fn app() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        button { 
            onclick: move |event| {
                count += 1;
            },
            "Click Me"
         }
         p { 
            "Clicked: {count}"
          }
    }
}


#[component]
fn Notes(props: CustomProps) -> Element {
    rsx! {
        p { "{props.text}" }
        p { "size == {props.size}" }
    }
}
