use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::{
    HiBell, HiChartPie, HiCog, HiCubeTransparent, HiDocumentSearch, HiUser,
};
use dioxus_free_icons::Icon;

// mod menu;

#[derive(Props)]
pub struct WrapperProps<'a> {
    children: Element<'a>,
}

pub fn Wrapper<'a>(cx: Scope<'a, WrapperProps<'a>>) -> Element {
    cx.render(rsx!(
        head {
            title {
                "Family Tree"
            }
            script{
                src:"https://cdn.tailwindcss.com",
            }
        }
        body {
            div {
                // sidebar
                div {
                    class: "lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-72 lg:flex-col",
                    div {
                        class: "flex grow flex-col gap-y-5 overflow-y-auto bg-gray-900 px-6 pb-4",
                        div {
                            class: "flex h-16 shrink-0 items-center",
                            img {
                                class: "h-8 w-auto",
                                src: "https://saktistatic.muezza.workers.dev/icons/android-chrome-192x192.png",
                                alt: "Workflow",
                            }
                        }
                        nav {
                            class: "flex flex-1 flex-col",
                            ul {
                                class: "flex flex-1 flex-col gap-y-7",
                                li {
                                    ul {
                                        class: "-mx-2 space-y-1",
                                        li {
                                            a {
                                                class: "bg-gray-800 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold",
                                                href: "/",
                                                Icon{
                                                    width: 24,
                                                    height: 24,
                                                    fill: "none",
                                                    icon: HiCubeTransparent,
                                                }
                                                "Tree"
                                            }
                                        }
                                        li {
                                            a {
                                                class: "text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold",
                                                href: "/person",
                                                Icon{
                                                    width: 24,
                                                    height: 24,
                                                    fill: "none",
                                                    icon: HiUser,
                                                }
                                                "Person"
                                            }
                                        }
                                        li {
                                            a {
                                                class: "text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold",
                                                href: "/stats",
                                                Icon{
                                                    width: 24,
                                                    height: 24,
                                                    fill: "none",
                                                    icon: HiChartPie,
                                                }
                                                "Stats"
                                            }
                                        }
                                    }
                                }
                                li {
                                    class: "mt-auto",
                                    a {
                                        class: "group -mx-2 flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-gray-400 hover:bg-gray-800 hover:text-white",
                                        href: "/settings",
                                        Icon{
                                            width: 24,
                                            height: 24,
                                            fill: "none",
                                            icon: HiCog,
                                        }
                                        "Settings"
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "lg:pl-72",
                    div {
                        class: "sticky top-0 z-40 flex h-16 shrink-0 items-center gap-x-4 border-b border-gray-200 bg-white px-4 shadow-sm sm:gap-x-6 sm:px-6 lg:px-8",
                        div {
                            class:"flex flex-1 gap-x-4 self-stretch lg:gap-x-6",
                            form {
                                class: "relative flex flex-1 mb-0",
                                action: "#",
                                method: "GET",
                                label {
                                    class: "sr-only",
                                    "Search"
                                }
                                Icon{
                                    class: "pointer-events-none absolute inset-y-0 left-0 h-full w-5 text-gray-400",
                                    width: 20,
                                    height: 20,
                                    fill: "none",
                                    icon: HiDocumentSearch,
                                }
                                input {
                                    class: "block h-full w-full border-0 py-0 pl-8 pr-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 focus:outline-none sm:text-sm",
                                    placeholder: "Search...",
                                    r#type: "search",
                                    name: "search"
                                }
                            }
                            div {
                                class: "flex items-center gap-x-4 lg:gap-x-6",
                                button {
                                    class: "-m-2.5 p-2.5 text-gray-400 hover:text-gray-500",
                                    Icon{
                                        class: "h-6 w-6",
                                        width: 24,
                                        height: 24,
                                        fill: "none",
                                        icon: HiBell,
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "py-10",
                        div {
                            class: "px-4 sm:px-6 lg:px-8",
                            &cx.props.children
                        }
                    }
                }
            }
        }
    ))
}
