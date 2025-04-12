use axum::{
    routing::{get, post},
    Router,
};
use maud::{html, Markup, DOCTYPE};
use navs::*;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod navs;

async fn index() -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "Space's site" }
            link rel="stylesheet" href="assets/style.css"{}
            script src="https://unpkg.com/htmx.org@2.0.1"{}
        }
        body {
            div class="vid-container"{
                video id="bg-video" data-autoplay autoPlay playsinline webkit-playsinline muted loop data-object-fit="cover"{
                    source src="assets/output.mp4" type="video/mp4" {}
                }
            }
            div id="navbar" {
                div {
                    img id="pfp" src="assets/vuWHRTzV_400x400.jpg" alt="" {}
                }
                div class="capsulecontainer"{
                    (capsule("/", "Home"))
                    (capsule("/projects", "Projects"))
                    (capsule("/blogs", "Blogs"))
                    (capsule("/socials", "Socials"))
                }
            }
            hr {}
            div id="content" {
                p { "Home stuff over here :(" }
            }
        }
    }
}

fn capsule(link: &str, text: &str) -> Markup {
    html! {
        div class="capsule" {
            a hx-post=(link) hx-target="#content" hx-swap="innerHTML" { h1 {(text)} }
        }
    }
}

#[tokio::main]
async fn main() {
    // Asset manager
    let serve_dir = ServeDir::new("assets");

    // Router initialization
    let app = Router::new()
        .route("/", get(index))
        .route("/", post(home))
        .route("/projects", post(projects))
        .route("/blogs", post(blogs))
        .route("/socials", post(socials))
        .nest_service("/assets", serve_dir);

    let listener = TcpListener::bind("127.0.0.1:5001").await.unwrap();

    println!("Listening on 127.0.0.1:5001");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
