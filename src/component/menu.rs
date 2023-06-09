use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

#[derive(Props)]
pub struct MenuItemProps<'a> {
    title: String,
    url: String,
    is_active: bool,
    icon: dyn IconShape,
}

pub fn MenuItem<'a>(cx: Scope<'a,MenuItemProps<'a>>) -> Element {
    cx.render(rsx!(
        li {
            a {
                class: "bg-gray-800 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold",
                href: cx.props.url,
                Icon {
                    width: 24,
                    height: 24,
                    fill: "none",
                    icon: cx.props.icon,
                }
                cx.props.title
            }
        }
    ))
}
