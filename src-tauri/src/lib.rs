use std::sync::Mutex;

use rusqlite::{types::Value, Connection, OpenFlags};
use tauri::{Manager, State};

#[tauri::command]
fn connect_to_db(connection_url: &str, state: State<'_, Application>) -> String {
    println!("{connection_url}");
    let conn = Connection::open_with_flags(
        connection_url,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_URI
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    );

    if let Ok(conn) = conn {
        let mut conn_holder = state.connection.lock().unwrap();
        *conn_holder = Some(conn);
        "Connected".to_string()
    } else {
        "Failed".to_string()
    }
}

#[tauri::command]
fn execute_sql(sql: &str, state: State<'_, Application>) -> String {
    let conn = state.connection.lock().unwrap();
    if let Some(conn) = conn.as_ref() {
        let mut stmt = conn.prepare(sql).unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut results: Vec<Value> = Vec::new();

        while let Some(row) = rows.next().unwrap() {
            results.push(row.get(0).unwrap());
        }

        let formatted_val = format!("{results:?}");
        formatted_val
    } else {
        "Query Failed".to_string()
    }
}

struct Application {
    connection: Mutex<Option<Connection>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Application {
                connection: Mutex::new(None),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![connect_to_db, execute_sql])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
