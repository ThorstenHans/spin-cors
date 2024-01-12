use models::Item;
use spin_sdk::http::{IntoResponse, Json, Params, Request, Response, Router};
use spin_sdk::http_component;
use spin_sdk::sqlite::{Connection, Value};

use crate::cors::handle_preflight;

mod cors;
mod models;
use crate::cors::WithCors;

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();

    router.options("*", handle_preflight);
    router.get("/api/items", get_items);
    router.post("/api/items", post_item);
    router.delete("/api/items/:id", delete_item);
    println!("Handing {:?} {:?}", req.method(), req.uri());
    Ok(router.handle(req))
}

fn get_items(_req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let c = Connection::open_default()?;
    let values = [];
    let result = c.execute("SELECT ID, NAME FROM ITEMS", values.as_slice())?;
    let items: Vec<_> = result
        .rows()
        .map(|row| {
            let id: i64 = row.get::<i64>("ID").expect("ID not set");
            let name: String = row.get::<&str>("NAME").unwrap_or_default().to_string();
            Item::new(id, name)
        })
        .collect();
    let payload = serde_json::to_vec(&items)?;

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .with_cors()
        .body(payload)
        .build())
}

fn post_item(req: http::Request<Json<Item>>, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let c = Connection::open_default()?;

    let values = [Value::Text(req.body().name.clone())];
    c.execute("INSERT INTO ITEMS (NAME) VALUES (?)", values.as_slice())?;

    Ok(Response::builder().status(200).with_cors().body(()).build())
}

fn delete_item(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(404, ()));
    };

    let Ok(id) = id.parse::<i64>() else {
        return Ok(Response::new(400, ()));
    };

    let connection = Connection::open_default()?;
    let values = [Value::Integer(id)];
    connection.execute("DELETE FROM ITEMS WHERE ID = ?", values.as_slice())?;
    Ok(Response::builder().status(200).with_cors().body(()).build())
}
