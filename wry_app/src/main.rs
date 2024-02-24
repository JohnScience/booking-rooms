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

    let commands = {
        fn commands<'a>(builder: wry::WebViewBuilder<'a>) -> wry::WebViewBuilder<'a> {
            todo!()
        }
        move |builder: wry::WebViewBuilder| {
            builder.with_custom_protocol(
                "tauriless".to_string(),
                move |req: wry::http::request::Request<Vec<u8>>| {
                    let (parts, body): (wry::http::request::Parts, Vec<u8>) = req.into_parts();
                    let uri: wry::http::uri::Uri = parts.uri;
                    let path: &str = uri.path();
                    let path: &str = path.trim_start_matches('/');
                    let resp_body: std::result::Result<Vec<u8>, tauriless::pot::Error> = match path
                    {
                        <__command_do_stuff_with_num as tauriless::Command>::NAME => {
                            let args: <__command_do_stuff_with_num as tauriless::Command>::Args =
                                match tauriless::pot::from_slice(body.as_slice()) {
                                    Ok(args) => args,
                                    Err(e) => return tauriless::handle_deserialization_error(
                                        <__command_do_stuff_with_num as tauriless::Command>::NAME,
                                        e,
                                    ),
                                };
                            let ret: <__command_do_stuff_with_num as tauriless::Command>::RetTy =
                                __command_do_stuff_with_num::command(args);
                            tauriless::pot::to_vec(&ret)
                        }
                        _ => return tauriless::handle_unknown_command(path),
                    };
                    let resp_body: Vec<u8> = match resp_body {
                        Ok(body) => body,
                        Err(e) => return tauriless::handle_serialization_error(e),
                    };
                    wry::http::response::Response::builder()
                        .status(wry::http::StatusCode::OK)
                        .header(
                            wry::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                            wry::http::HeaderValue::from_static("*"),
                        )
                        .body(std::borrow::Cow::Owned(resp_body))
                        .unwrap()
                },
            )
        }
    };

    // starting the webview
    let _webview = WebViewBuilder::new(&window)
        .with_url(&format!("http://localhost:{port}/"))
        .with_initialization_script(&format!("console.log('Server running at port ', {port});"))
        //.with_html(r#"<html><body><h1>Hello, world!</h1></body></html>"#)?
        //.with_custom_protocol(
        //    "tauriless".to_string(),
        //    __command_do_stuff_with_num::custom_protocol,
        //)
        .with_tauriless_commands(commands)
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
