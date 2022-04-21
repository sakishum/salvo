use salvo::prelude::*;
use salvo::extra::serve::{Options, StaticDir};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::with_path("<**path>").get(StaticDir::width_options(
        vec!["examples/static/boy", "examples/static/girl"],
        Options {
            dot_files: false,
            listing: true,
            defaults: vec!["index.html".to_owned()],
        },
    ));
    tracing::info!("Listening on http://0.0.0.0:7878");
    Server::new(TcpListener::bind("0.0.0.0:7878")).serve(router).await;
}