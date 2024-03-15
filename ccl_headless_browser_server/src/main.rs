use arcstr::ArcStr;
use axum::{debug_handler, extract::State, http::StatusCode, routing::get, Json, Router};
use calgary_central_library::{AsyncClient, Availability, Client, Room};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct AvailableRoomsArgs {
    date: NaiveDate,
    group_size: u8,
}

#[derive(Clone)]
struct CCLSHBServerState {
    web_driver_host: ArcStr,
    web_driver_port: u16,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/available_rooms", get(available_rooms))
        .with_state(CCLSHBServerState {
            web_driver_host: arcstr::literal!("localhost"),
            web_driver_port: 4444,
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn available_rooms(
    State(state): State<CCLSHBServerState>,
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<AvailableRoomsArgs>,
) -> Result<Json<Vec<(Room, Availability)>>, StatusCode> {
    let AvailableRoomsArgs { date, group_size } = payload;
    let CCLSHBServerState {
        web_driver_host,
        web_driver_port,
    } = state;
    let client = Client::new((web_driver_host.as_str(), web_driver_port))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let v = client
        .available_rooms(date, group_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(v))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_available_rooms_in_a_week() {
        let now = chrono::Local::now();
        let today = now.date_naive();

        let day: chrono::NaiveDate = today + chrono::Duration::try_days(7).unwrap();
        let group_size = 10;

        let args = crate::AvailableRoomsArgs {
            date: day,
            group_size,
        };

        let client = reqwest::Client::new();
        let resp = client
            .get("http://localhost:3000/available_rooms")
            .json(&args)
            .send()
            .await
            .unwrap();
        let resp: Vec<(
            calgary_central_library::Room,
            calgary_central_library::Availability,
        )> = resp.json().await.unwrap();

        for (room, availability) in resp.iter() {
            println!("{:?} {}", room, availability);
        }
    }
}
