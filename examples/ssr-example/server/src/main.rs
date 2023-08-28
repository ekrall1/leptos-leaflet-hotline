use app::*;
use axum::{routing::post, Router};
use clap::Parser;
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

pub mod fileserv;

#[derive(Parser, Debug)]
#[clap(name = "site", about = "site addr for example")]
struct SiteOverrides {
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    #[clap(short = 'p', long = "port", default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;

    let site_opt = SiteOverrides::parse();
    let addr = SocketAddr::from((
        IpAddr::from_str(site_opt.addr.as_str()).unwrap(),
        site_opt.port,
    ));
    let routes = generate_route_list(|| view! { <App/> }).await;

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, || view! { <App/> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
