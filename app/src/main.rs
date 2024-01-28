use fantoccini::{elements::Element, wd::Capabilities, ClientBuilder, Locator};

// async fn available_rooms(
//     c: &fantoccini::Client,
//     date:
// )

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    // https://github.com/jonhoo/fantoccini/issues/45#issuecomment-1546600219
    let cap: Capabilities = serde_json::from_str(
        r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
    )
    .unwrap();
    let c = ClientBuilder::native()
        .capabilities(cap)
        .connect("http://localhost:9515")
        .await
        .expect("failed to connect to WebDriver");

    c.set_window_rect(0, 0, 1920, 1080).await?;

    c.goto("https://calgarylibrary.ca/events-and-programs/book-a-space/book-a-room/")
        .await?;
    let url = c.current_url().await?;
    assert_eq!(
        url.as_ref(),
        "https://calgarylibrary.ca/events-and-programs/book-a-space/book-a-room/"
    );

    let screenshot_png = c.screenshot().await.unwrap();
    std::fs::write("0.png", screenshot_png).unwrap();

    // There are more than one button that matches the selector
    let buttons = c
        .find_all(Locator::Css("button.btn-submission.red[value='Search']"))
        .await?;
    let mut button: Option<Element> = None;
    for b in buttons {
        if b.is_displayed().await? {
            button = match button {
                None => Some(b),
                Some(_) => panic!("More than one button found"),
            };
            break;
        }
    }
    let button = button.unwrap();
    let screenshot_png = button.screenshot().await.unwrap();
    std::fs::write("1.png", screenshot_png).unwrap();

    // // click "Foo Lake"
    // c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

    // let url = c.current_url().await?;
    // assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

    c.close().await
}
