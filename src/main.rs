use clap::Parser;
use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

mod sample;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    #[clap(short, long)]
    dot: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // print dot if flag set
    if args.dot {
        sample::print_dot();
        return;
    }

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .route("/", get(app_endpoint))
                .into_make_service(),
        )
        .await
        .unwrap();
}


async fn app_endpoint() -> Html<String> {
    // render the rsx! macro to HTML
    Html(dioxus_ssr::render_lazy(rsx! {
        div { "hello world!" }
    }))
}
