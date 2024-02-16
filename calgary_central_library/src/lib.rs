use chrono::NaiveDate;
use std::fmt::Debug;

mod fantoccini_impl;
mod room;

pub use room::{Availability, Room};

pub trait AsyncQuerySelector {
    type Element: AsyncElement;

    type QuerySelectorError: Debug;
    async fn query_selector(
        &self,
        selector: &str,
    ) -> Result<Self::Element, Self::QuerySelectorError>;

    type QuerySelectorAllError: Debug;
    type ElementIterator: std::iter::IntoIterator<Item = Self::Element>;
    async fn query_selector_all(
        &self,
        selector: &str,
    ) -> Result<Self::ElementIterator, Self::QuerySelectorAllError>;
}

pub trait AsyncElement: AsyncQuerySelector {
    type TextFnError: Debug;
    async fn text(&self) -> Result<String, Self::TextFnError>;
    type ClickError: Debug;
    async fn click(&self) -> Result<(), Self::ClickError>;
}

pub trait AsyncClient: Sized + AsyncQuerySelector {
    type NewError: Debug;
    type NewArgs<'a>;
    async fn new<'a>(args: Self::NewArgs<'a>) -> Result<Self, Self::NewError>;

    type NavigateToUrlError: Debug;
    async fn navigate_to_url(&self, url: &str) -> Result<(), Self::NavigateToUrlError>;

    type AvailableRoomsError: Debug;
    async fn available_rooms(
        &self,
        date: NaiveDate,
        group_size: u8,
    ) -> Result<Vec<(Room, Availability)>, Self::AvailableRoomsError>;

    type FindSearchButtonError: Debug;
    async fn find_search_button(&self) -> Result<Self::Element, Self::FindSearchButtonError>;

    type CloseError: Debug;
    async fn close(self) -> Result<(), Self::CloseError>;
}

pub type Client = fantoccini_impl::ClientImpl;
