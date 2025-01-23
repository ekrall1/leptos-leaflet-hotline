use simple_logger::init_with_level;
use leptos::{view, logging};
use leptos::prelude::*;
use leptos_leaflet_hotline_ssr::app::*;
use leptos_axum::{generate_route_list, file_and_error_handler, LeptosRoutes};
use axum::Router;


#[tokio::main]
async fn main() {

    init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;

    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|| view! { <App/> });

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
