#![allow(non_snake_case)]
use std::sync::Arc;

use anyhow::Result;
use axum::extract::{Path, State};
use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;
use clap::Parser;
use component::Wrapper;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;
use tokio::signal;

mod component;
mod sample;

const DB_PATH: &str = "family.db";

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    #[clap(short, long)]
    dot: bool,
}

#[derive(Clone)]
struct AppState {
    db: Arc<Surreal<Db>>,
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    alive: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // print dot if flag set
    if args.dot {
        sample::print_dot();
        return Ok(());
    }

    // setup db
    let db = Surreal::new::<RocksDb>(DB_PATH).await?;
    db.use_ns("test").use_db("familytree").await?;

    let sql = "CREATE person SET name='Jon Snow', alive=true";
    let res = db.query(sql).await?;
    println!("{res:?}");

    let state = AppState { db: Arc::new(db) };

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .route("/", get(app_endpoint))
                .route("/person/new", get(|| async { "get create a new person" }))
                .route("/person/new", post(|| async { "post create a new person" }))
                .route("/person", get(list_person))
                .route("/person/:id", get(get_person))
                .route("/stats", get(stats))
                .route("/settings", get(settings))
                .route("/debug", get(debug))
                .with_state(state)
                .into_make_service(),
        )
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

pub fn About(cx: Scope) -> Element {
    cx.render(rsx!(p {
        b {"Dioxus Labs"}
        " An Open Source project dedicated to making Rust UI wonderful."
    }))
}

async fn debug() -> Html<String> {
    Html(dioxus_ssr::render_lazy(rsx! {
        Wrapper {
            div { "hello world!" }
            About {}
        }
    }))
}

async fn stats() -> Html<String> {
    Html(dioxus_ssr::render_lazy(rsx! {
        Wrapper {
            h1 { "Stats" }
        }
    }))
}

async fn settings() -> Html<String> {
    Html(dioxus_ssr::render_lazy(rsx! {
        Wrapper {
            h1 { "Settings" }
        }
    }))
}

async fn get_person(_state: State<AppState>, Path(id): Path<String>) -> Html<String> {
    dbg!(id);
    Html(dioxus_ssr::render_lazy(rsx! {
        Wrapper{
            h1 {
                "Get person"
            }
        }
    }))
}

async fn list_person(state: State<AppState>) -> Html<String> {
    let sql = "SELECT * FROM person";
    let res: Vec<Person> = state.db.query(sql).await.unwrap().take(0).unwrap();

    let people = res.into_iter();

    Html(dioxus_ssr::render_lazy(rsx! {
        Wrapper{
            h1 {
                class: "text-3xl font-bold underline",
                "People"
            }
            p { "here list of people"}
            ul {
                people.map(|person| rsx!(
                    li {
                        p{ person.name}
                    }
                ))
            }
        }
    }))
}

async fn app_endpoint() -> Html<String> {
    // render the rsx! macro to HTML
    Html(dioxus_ssr::render_lazy(rsx! {
        Wrapper { "hello world from SSR!, updated" }
    }))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
