// TODO: consider using futures::future::join_all for async iteration

use std::env;

use chrono::{DateTime, NaiveDate};
use fantoccini::{elements::Element, wd::Capabilities, ClientBuilder, Locator};
use thiserror::Error;

mod room;

use room::{Availability, RoomChoice};

use crate::room::Room;

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

trait TakeNextScreenshot {
    async fn take_next_screenshot(&self, screenshot_counter: &mut usize);
}

impl TakeNextScreenshot for Element {
    async fn take_next_screenshot(&self, screenshot_counter: &mut usize) {
        let screenshot_png = self.screenshot().await.unwrap();
        std::fs::write(format!("{screenshot_counter}.png"), screenshot_png).unwrap();
        *screenshot_counter += 1;
    }
}

impl TakeNextScreenshot for fantoccini::Client {
    async fn take_next_screenshot(&self, screenshot_counter: &mut usize) {
        let screenshot_png = self.screenshot().await.unwrap();
        std::fs::write(format!("{screenshot_counter}.png"), screenshot_png).unwrap();
        *screenshot_counter += 1;
    }
}

#[derive(Error, Debug)]
enum FindSearchButtonError {
    #[error("No search button found")]
    NoButtonFound,
    #[error("More than one search button found")]
    MoreThanOneButtonFound,
    #[error("Fantoccini error: {0}")]
    FantocciniError(#[from] fantoccini::error::CmdError),
}

async fn find_search_button(c: &fantoccini::Client) -> Result<Element, FindSearchButtonError> {
    let buttons = c
        .find_all(Locator::Css("button.btn-submission.red[value='Search']"))
        .await?;
    let mut button: Option<Element> = None;
    for b in buttons {
        if b.is_displayed().await? {
            button = match button {
                None => Some(b),
                Some(_) => return Err(FindSearchButtonError::MoreThanOneButtonFound),
            };
            break;
        }
    }
    button.ok_or(FindSearchButtonError::NoButtonFound)
}

async fn available_rooms(
    c: &fantoccini::Client,
    date: NaiveDate,
    group_size: u8,
) -> Result<Vec<(Room, Availability)>, fantoccini::error::CmdError> {
    let mut rooms = Vec::new();
    let booking_url = format!(
        "https://calgarylibrary.ca/events-and-programs/book-a-space/book-a-room/?date={}&location=1&groupsize={}",
        date.format("%Y-%m-%d"),
        group_size
    );
    c.goto(&booking_url).await?;

    // screenshot counter
    let mut sc: usize = 0;
    // we shadow sc in order to avoid taking &mut references to it everywhere
    let sc = &mut sc;

    c.take_next_screenshot(sc).await;

    let search_button = find_search_button(c).await.unwrap();
    search_button.take_next_screenshot(sc).await;

    for room_elem in c.find_all(Locator::Css(".room-booking-card")).await? {
        room_elem.take_next_screenshot(sc).await;
        let title: Element = room_elem.find(Locator::Css(".uk-card-title")).await?;
        let title: String = title.text().await?;
        let room_choice: RoomChoice = RoomChoice::from_title(&title);
        let description: Element = room_elem.find(Locator::Css("p")).await?;
        let description: String = description.text().await?;
        let room = Room::new(room_choice, title, description);

        let view_availability_button = room_elem.find(Locator::Css("a.availability")).await?;
        view_availability_button.take_next_screenshot(sc).await;
        view_availability_button.click().await?;

        // let availability_panel = room_elem.find(Locator::Css(".availability-panel")).await?;
        // availability_panel.take_next_screenshot(sc).await;

        room_elem.take_next_screenshot(sc).await;

        let time_slots: Vec<Element> = room_elem.find_all(Locator::Css("li.time-slot")).await?;
        let time_slots: Vec<String> = {
            let mut v: Vec<String> = Vec::new();
            for time_slot in time_slots {
                let time_slot: String = time_slot.text().await?;
                v.push(time_slot);
            }
            v
        };
        println!("{room:?}");
        let availability = Availability::from(time_slots);
        println!("{availability}");

        rooms.push((room, availability));
    }

    Ok(rooms)
}

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    // https://github.com/jonhoo/fantoccini/issues/45#issuecomment-1546600219
    let cap: Capabilities = serde_json::from_str(
        r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless" , "--no-sandbox"]}}"#,
    )
    .unwrap();
    let addr = format!(
        "http://{host}:{port}",
        host = env::var("CHROMEDRIVER_HOST").unwrap(),
        port = env::var("CHROMEDRIVER_PORT").unwrap()
    );
    println!("Connecting to WebDriver at {addr}...");
    let c = ClientBuilder::native()
        .capabilities(cap)
        .connect(&addr)
        .await
        .unwrap_or_else(|e| panic!("failed to connect to WebDriver: {e:?}"));

    c.set_window_rect(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT).await?;

    let now: DateTime<chrono::Local> = chrono::Local::now();
    let today: NaiveDate = now.date_naive();
    let available_rooms: Vec<(Room, Availability)> =
        available_rooms(&c, today.succ_opt().unwrap().succ_opt().unwrap(), 10).await?;
    for (room, availability) in available_rooms {
        println!("{room:?}");
        println!("{availability}");
    }

    c.close().await
}
