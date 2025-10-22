use std::sync::Arc;

use dioxus::prelude::*;
use rusqlite::{Connection, OpenFlags, types::Value};
use smol::lock::RwLock;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone)]
struct DbConnection(Arc<RwLock<Option<Connection>>>);

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| DbConnection(RwLock::new(None).into()));
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        SqlInput {}

    }
}

#[component]
pub fn SqlInput() -> Element {
    let mut sql = use_signal(|| "SELECT * FROM test;".to_string());
    let mut connection_url = use_signal(String::new);
    let mut connect_txt = use_signal(|| "Connect".to_string());
    let mut button_text = use_signal(|| "Execute".to_string());
    let mut result = use_signal(String::new);

    let connect_to_db = move |_| async move {
        let db_conn = consume_context::<DbConnection>().0;

        connect_txt.set("Connecting".to_string());

        let conn = Connection::open_with_flags(
            format!("{connection_url}"),
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .unwrap();

        let mut conn_holder = db_conn.write().await;
        *conn_holder = Some(conn);
        connect_txt.set("Connected".to_string());
    };

    let execute_query = move |_| async move {
        button_text.set("Executing".to_string());

        let db_conn = consume_context::<DbConnection>().0;

        if let Some(conn) = db_conn.read().await.as_ref() {
            let mut stmt = conn.prepare(&sql()).unwrap();
            let mut rows = stmt.query([]).unwrap();
            let mut results: Vec<Value> = Vec::new();

            while let Some(row) = rows.next().unwrap() {
                results.push(row.get(0).unwrap());
            }

            let formatted_val = format!("{results:?}");
            result.set(formatted_val);
        }

        button_text.set("Execute".to_string());
    };

    rsx! {
        div {
            input {
                id: "connection_url",
                value: "{connection_url}",
                oninput: move |event| connection_url.set(event.value())
            }
            button {
                id: "connect",
                onclick: connect_to_db,
                "{connect_txt}"
            }
            textarea {
                id: "input",
                value: "{sql}",
                oninput: move |event| sql.set(event.value())
            }
            button {
                id: "execute",
                onclick: execute_query,
                "{button_text}"
            }
            div {
                id: "result",
                "{result}"
            }
        }
    }
}
