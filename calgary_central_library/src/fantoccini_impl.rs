use crate::{
    room::{Availability, Room, RoomChoice},
    AsyncClient, AsyncElement, AsyncQuerySelector,
};
use fantoccini::{elements::Element, Locator};
use thiserror::Error;

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

pub struct ClientImpl(fantoccini::Client);

#[derive(Error, Debug)]
pub enum NewError {
    #[error("Failed to create a new session")]
    NewSessionError(#[from] fantoccini::error::NewSessionError),
    #[error("Failed to set window rect")]
    SetWindowRectError(#[from] fantoccini::error::CmdError),
}

#[derive(Error, Debug)]
pub enum AvailableRoomsError {
    #[error("Failed to navigate to URL")]
    NavigateToUrlError(fantoccini::error::CmdError),
    #[error("Failed to find the search button")]
    FindSearchButtonError(#[from] FindSearchButtonError),
    #[error("Failed to execute querySelectorAll")]
    QuerySelectorError(fantoccini::error::CmdError),
    #[error("Failed to get text")]
    FailedGetText(fantoccini::error::CmdError),
    #[error("Failed to click")]
    ClickError(fantoccini::error::CmdError),
}

#[derive(Error, Debug)]
pub enum FindSearchButtonError {
    #[error("No search button found")]
    NoButtonFound,
    #[error("More than one search button found")]
    MoreThanOneButtonFound,
    #[error("Fantoccini error: {0}")]
    FantocciniError(#[from] fantoccini::error::CmdError),
}

impl AsyncQuerySelector for Element {
    type Element = Self;

    type QuerySelectorError = fantoccini::error::CmdError;
    async fn query_selector(
        &self,
        selector: &str,
    ) -> Result<Self::Element, Self::QuerySelectorError> {
        self.find(Locator::Css(selector)).await
    }

    type QuerySelectorAllError = fantoccini::error::CmdError;
    type ElementIterator = Vec<Self>;
    async fn query_selector_all(
        &self,
        selector: &str,
    ) -> Result<Self::ElementIterator, Self::QuerySelectorAllError> {
        self.find_all(Locator::Css(selector)).await
    }
}

impl AsyncElement for Element {
    type TextFnError = fantoccini::error::CmdError;
    async fn text(&self) -> Result<String, Self::TextFnError> {
        self.text().await
    }

    type ClickError = fantoccini::error::CmdError;
    async fn click(&self) -> Result<(), Self::ClickError> {
        self.click().await
    }
}

impl AsyncQuerySelector for ClientImpl {
    type Element = fantoccini::elements::Element;

    type QuerySelectorError = fantoccini::error::CmdError;
    async fn query_selector(
        &self,
        selector: &str,
    ) -> Result<Self::Element, Self::QuerySelectorError> {
        self.0.find(Locator::Css(selector)).await
    }

    type QuerySelectorAllError = fantoccini::error::CmdError;
    type ElementIterator = Vec<Self::Element>;
    async fn query_selector_all(
        &self,
        selector: &str,
    ) -> Result<Self::ElementIterator, Self::QuerySelectorAllError> {
        self.0.find_all(Locator::Css(selector)).await
    }
}

impl AsyncClient for ClientImpl {
    type NewError = NewError;
    type NewArgs<'a> = (&'a str, u16);
    async fn new<'a>((host, port): Self::NewArgs<'a>) -> Result<Self, Self::NewError> {
        // On `--headless` argument: https://github.com/jonhoo/fantoccini/issues/45#issuecomment-1546600219
        let cap: fantoccini::wd::Capabilities = serde_json::from_str(
            r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
        )
        .unwrap();
        let addr = format!("http://{}:{}", host, port);
        let c = fantoccini::ClientBuilder::native()
            .capabilities(cap)
            .connect(&addr)
            .await?;
        c.set_window_rect(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT).await?;

        Ok(Self(c))
    }

    type NavigateToUrlError = fantoccini::error::CmdError;
    async fn navigate_to_url(&self, url: &str) -> Result<(), Self::NavigateToUrlError> {
        self.0.goto(url).await
    }

    type AvailableRoomsError = AvailableRoomsError;
    async fn available_rooms(
        &self,
        date: chrono::prelude::NaiveDate,
        group_size: u8,
    ) -> Result<Vec<(crate::room::Room, crate::room::Availability)>, Self::AvailableRoomsError>
    {
        let mut rooms = Vec::new();
        let booking_url = format!(
                "https://calgarylibrary.ca/events-and-programs/book-a-space/book-a-room/?date={}&location=1&groupsize={}",
                date.format("%Y-%m-%d"),
                group_size
            );
        self.navigate_to_url(&booking_url)
            .await
            .map_err(AvailableRoomsError::NavigateToUrlError)?;
        let _search_button = self.find_search_button().await?;
        for room_elem in self
            .query_selector_all(".room-booking-card")
            .await
            .map_err(AvailableRoomsError::QuerySelectorError)?
        {
            let title: Element = room_elem
                .query_selector(".uk-card-title")
                .await
                .map_err(AvailableRoomsError::QuerySelectorError)?;
            let title: String = title
                .text()
                .await
                .map_err(AvailableRoomsError::FailedGetText)?;
            let room_choice: RoomChoice = RoomChoice::from_title(&title);
            let description: Element = room_elem
                .query_selector("p")
                .await
                .map_err(AvailableRoomsError::QuerySelectorError)?;
            let description: String = description
                .text()
                .await
                .map_err(AvailableRoomsError::FailedGetText)?;
            let room = Room::new(room_choice, title, description);

            let view_availability_button = room_elem
                .query_selector("a.availability")
                .await
                .map_err(AvailableRoomsError::QuerySelectorError)?;
            view_availability_button
                .click()
                .await
                .map_err(AvailableRoomsError::ClickError)?;

            let time_slots = room_elem
                .query_selector_all("li.time-slot")
                .await
                .map_err(AvailableRoomsError::QuerySelectorError)?;
            let time_slots: Vec<String> = {
                let mut v: Vec<String> = Vec::new();
                for time_slot in time_slots {
                    let time_slot: String = time_slot
                        .text()
                        .await
                        .map_err(AvailableRoomsError::FailedGetText)?;
                    v.push(time_slot)
                }
                v
            };
            let availability = Availability::from(time_slots);
            rooms.push((room, availability));
        }

        Ok(rooms)
    }

    type FindSearchButtonError = FindSearchButtonError;
    async fn find_search_button(&self) -> Result<Self::Element, Self::FindSearchButtonError> {
        let buttons = self
            .query_selector_all("button.btn-submission.red[value='Search']")
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

    type CloseError = fantoccini::error::CmdError;

    async fn close(self) -> Result<(), Self::CloseError> {
        self.0.close().await
    }
}
