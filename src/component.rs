use dioxus::prelude::*;

#[derive(Props)]
pub struct WrapperProps<'a> {
    children: Element<'a>,
}

pub fn Wrapper<'a>(cx: Scope<'a, WrapperProps<'a>>) -> Element {
    cx.render(rsx!(
        head {
            script{
                src:"https://cdn.tailwindcss.com",
            }
        }
        body {
            div {
                class: "container mx-auto",
                &cx.props.children
            }
        }
    ))
}
