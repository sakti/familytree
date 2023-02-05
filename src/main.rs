use std::sync::Arc;

use anyhow::Result;
use axum::{extract::State, response::Html, routing::get, Router};
use clap::Parser;
use dioxus::prelude::*;
use surrealdb::{
    sql::{Object, Value},
    Datastore, Response, Session,
};
use tokio::signal;

mod sample;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    #[clap(short, long)]
    dot: bool,
}

#[derive(Clone)]
struct AppState {
    db: Arc<Datastore>,
    session: Session,
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
    let ds = Datastore::new("memory").await?;
    let session = Session::for_db("ns", "family_tree");

    let sql = "CREATE person SET name='Jon Snow', alive=true";
    let res = ds.execute(sql, &session, None, false).await?;
    println!("{res:?}");

    let state = AppState {
        db: Arc::new(ds),
        session: session.clone(),
    };

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .route("/", get(app_endpoint))
                .route("/test", get(|| async { "hallo dunia" }))
                .route("/test2", get(|| async { "hello world 2" }))
                .route("/people", get(list_people))
                .with_state(state)
                .into_make_service(),
        )
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn into_iter_objects(res: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let result = res.into_iter().next().map(|it| it.result).transpose()?;
    match result {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(obj) => Ok(obj),
                _ => Err(anyhow::anyhow!("expected object")),
            });
            Ok(it)
        }
        _ => Err(anyhow::anyhow!("no record found")),
    }
}

async fn list_people(state: State<AppState>) -> Html<String> {
    let sql = "SELECT * FROM person";
    let res = state
        .db
        .execute(sql, &state.session, None, false)
        .await
        .unwrap();
    let people = into_iter_objects(res).unwrap();

    Html(dioxus_ssr::render_lazy(rsx! {
        div {
            h1 { "People" }
            p { "here list of people"}
            ul {
                people.map(|person| rsx!(
                    li {
                        p{ person.unwrap().get("name").unwrap().to_string()}
                    }
                ))
            }
        }
    }))
}

async fn app_endpoint() -> Html<String> {
    // render the rsx! macro to HTML
    Html(dioxus_ssr::render_lazy(rsx! {
        div { "hello world from SSR!, updated" }
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
