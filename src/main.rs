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
    rsx! {
        button { 
            onclick: move |event| {
                info!("{:?}", event);
                error!("error failed to process");
            },
            "Click Me"
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
