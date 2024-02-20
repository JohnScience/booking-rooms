use axum::Router;
use axum_embed::ServeEmbed;
use rust_embed::RustEmbed;
use std::{borrow::Cow, net::SocketAddr};
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[derive(RustEmbed, Clone)]
#[folder = "../front/dist"]
struct Assets;

fn custom_protocol(
    _req: wry::http::request::Request<Vec<u8>>,
) -> wry::http::response::Response<Cow<'static, [u8]>> {
    let resp = wry::http::response::Response::builder()
        .status(wry::http::status::StatusCode::OK)
        .header(
            wry::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            wry::http::HeaderValue::from_static("*"),
        )
        .body(Cow::<'static, [u8]>::Borrowed(b"Hello, world!"))
        .unwrap();

    resp
}

async fn local_http_server_main(port_tx: tokio::sync::oneshot::Sender<u16>) {
    let app = Router::new().nest_service("/", ServeEmbed::<Assets>::new());
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    port_tx.send(listener.local_addr().unwrap().port()).unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Booking Rooms")
        .build(&event_loop)
        .unwrap();

    let (port_tx, port_rx) = tokio::sync::oneshot::channel::<u16>();

    let _local_http_server_handle = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();
        rt.block_on(local_http_server_main(port_tx));
    });

    let port: u16 = port_rx.blocking_recv().unwrap();

    // starting the webview
    let _webview = WebViewBuilder::new(&window)
        .with_url(&format!("http://localhost:{port}/"))?
        //.with_html(r#"<html><body><h1>Hello, world!</h1></body></html>"#)?
        .with_custom_protocol("myproto".to_string(), custom_protocol)
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => (),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
