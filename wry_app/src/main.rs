use axum::Router;
use axum_embed::ServeEmbed;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use tauriless::{command, commands, SyncCommand, WebViewBuilderExt};
use wry::WebViewBuilder;

#[derive(RustEmbed, Clone)]
#[folder = "../front/dist"]
struct Assets;

#[derive(Serialize, Deserialize)]
struct MyStruct {
    num: i32,
}

#[command]
fn do_stuff_with_num(my_struct: MyStruct) -> i32 {
    my_struct.num * 2
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
        .with_url(&format!("http://localhost:{port}/"))
        .with_initialization_script(&format!("console.log('Server running at port ', {port});"))
        //.with_html(r#"<html><body><h1>Hello, world!</h1></body></html>"#)?
        //.with_custom_protocol(
        //    "tauriless".to_string(),
        //    __command_do_stuff_with_num::custom_protocol,
        //)
        .with_tauriless_commands(commands!(do_stuff_with_num))
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
