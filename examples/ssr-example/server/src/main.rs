use app::{shell, App};
use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

#[tokio::main]
async fn main() {
    let configuration = get_configuration(None).expect("Leptos configuration");
    let address = configuration.leptos_options.site_addr;
    let options = configuration.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&options, routes, {
            let options = options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(options);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("bind HTTP listener");
    leptos::logging::log!("listening on http://{address}");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("serve application");
}
