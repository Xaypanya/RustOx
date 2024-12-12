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
    let mut email = use_signal(||String::from(""));
    rsx! {
        input { 
            value: "{email}",
            oninput: move |e| { email.set(e.value()); }
         }
         button {  
             onclick: move |_| { email.set(String::from("")); },
            "Reset",
         }
         p { 
            "Your email is: {email}"
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
