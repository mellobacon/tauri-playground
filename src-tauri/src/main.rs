// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use portable_pty::{native_pty_system, CommandBuilder, PtyPair, PtySize};
use std::{
    collections::HashMap, io::{BufRead, BufReader, Read, Write}, process::exit, sync::Arc, thread
};

use tauri::{async_runtime::Mutex as AsyncMutex, State};

struct AppState {
    terminals: HashMap<usize, (Arc<AsyncMutex<PtyPair>>, Arc<AsyncMutex<Box<dyn Write + Send>>>, Arc<AsyncMutex<BufReader<Box<dyn Read + Send>>>>)>,
    next_terminal_id: usize,
}

#[tauri::command]
// create a shell and add to it the $TERM env variable so we can use clear and other commands
async fn async_create_shell(state: State<'_, Arc<AsyncMutex<AppState>>>) -> Result<usize, String> {
    #[cfg(target_os = "windows")]
    let mut cmd = CommandBuilder::new("powershell.exe");

    #[cfg(not(target_os = "windows"))]
    let mut cmd = CommandBuilder::new("bash");

    // add the $TERM env variable so we can use clear and other commands

    #[cfg(target_os = "windows")]
    cmd.env("TERM", "cygwin");

    #[cfg(not(target_os = "windows"))]
    cmd.env("TERM", "xterm-256color");

    let pty_system = native_pty_system();

    let pty_pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();

    let reader = pty_pair.master.try_clone_reader().unwrap();
    let writer = pty_pair.master.take_writer().unwrap();
    let mut app_state = state.lock().await;
    let terminal_id = {
        let terminal_id = app_state.next_terminal_id;
        let terminal_pty = Arc::new(AsyncMutex::new(pty_pair));
        let terminal_reader = Arc::new(AsyncMutex::new(BufReader::new(reader)));
        let terminal_writer = Arc::new(AsyncMutex::new(writer));
        app_state.terminals.insert(terminal_id, (terminal_pty, terminal_writer, terminal_reader));
        app_state.next_terminal_id += 1;
        terminal_id
    };
    let (pty_pair, _, _) = app_state.terminals.get(&terminal_id).unwrap();
    for t in &app_state.terminals {
        println!("{}", t.0);
    }
    let mut child = pty_pair.lock().await.slave.spawn_command(cmd).map_err(|err| err.to_string())?;

    thread::spawn(move || {
        let status = child.wait().unwrap();
        exit(status.exit_code() as i32)
    });
    Ok(terminal_id)
}

#[tauri::command]
async fn async_write_to_pty(data: &str, terminal_id: usize, state: State<'_, Arc<AsyncMutex<AppState>>>) -> Result<(), ()> {
    let app_state = state.lock().await;
    for t in &app_state.terminals {
        println!("{}", t.0);
    }
    if let Some((_, writer, _)) = app_state.terminals.get(&terminal_id) {
        println!("should write data");
        write!(writer.lock().await, "{}", data).map_err(|_| ())?;
    } else {
        println!("Terminal with ID {} not found", terminal_id);
    }
    Ok(())
}

#[tauri::command]
async fn async_read_from_pty(terminal_id: usize, state: State<'_, Arc<AsyncMutex<AppState>>>) -> Result<Option<String>, ()> {
    let app_state = state.lock().await;
    if let Some((_, _, reader)) = app_state.terminals.get(&terminal_id) {
        let mut reader = reader.lock().await;
        let data = {
            // Read all available text
            let data = reader.fill_buf().map_err(|_| ())?;

            // Send the data to the webview if necessary
            if !data.is_empty() {
                std::str::from_utf8(data)
                    .map(|v| Some(v.to_string()))
                    .map_err(|_| ())?
            } else {
                None
            }
        };

        if let Some(data) = &data {
            reader.consume(data.len());
        }
        Ok(data)
    } else {
        Err(())
    }
}

#[tauri::command]
async fn async_resize_pty(rows: u16, cols: u16, terminal_id: usize, state: State<'_, Arc<AsyncMutex<AppState>>>) -> Result<(), ()> {
    let app_state = state.lock().await;
    if let Some((pty_pair, _, _)) = app_state.terminals.get(&terminal_id) {
        pty_pair
            .lock()
            .await
            .master
            .resize(PtySize {
                rows,
                cols,
                ..Default::default()
            })
            .map_err(|_| ())?;
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    let app_state = Arc::new(AsyncMutex::new(AppState {
        terminals: HashMap::new(),
        next_terminal_id: 0,
    }));
    tauri::Builder::default()
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            async_write_to_pty,
            async_resize_pty,
            async_create_shell,
            async_read_from_pty
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
