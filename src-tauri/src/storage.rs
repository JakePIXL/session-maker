use crate::session::Session;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Storage {
    data_dir: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        let app_data_dir = dirs::data_dir()
            .expect("Could not determine app data directory")
            .join("dibikaandaagozi");

        // Create directory if it doesn't exist
        fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

        Self {
            data_dir: app_data_dir,
        }
    }

    fn session_path(&self, id: &str) -> PathBuf {
        self.data_dir.join(format!("session_{}.json", id))
    }

    pub fn save_session(&mut self, session: &Session) -> Result<()> {
        let file_path = self.session_path(&session.id);
        let file = File::create(&file_path)
            .with_context(|| format!("Failed to create file at {:?}", file_path))?;

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &session)
            .with_context(|| format!("Failed to serialize session to {:?}", file_path))?;

        Ok(())
    }

    pub fn get_session(&self, id: &str) -> Result<Option<Session>> {
        let file_path = self.session_path(id);

        if !file_path.exists() {
            return Ok(None);
        }

        let file = File::open(&file_path)
            .with_context(|| format!("Failed to open file at {:?}", file_path))?;

        let reader = BufReader::new(file);
        let session = serde_json::from_reader(reader)
            .with_context(|| format!("Failed to deserialize session from {:?}", file_path))?;

        Ok(Some(session))
    }

    pub fn list_sessions(&self) -> Result<Vec<Session>> {
        let mut sessions: Vec<Session> = Vec::new();

        for entry in fs::read_dir(&self.data_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                    if file_name.starts_with("session_") {
                        if let Ok(file) = File::open(&path) {
                            let reader = BufReader::new(file);
                            if let Ok(session) = serde_json::from_reader(reader) {
                                sessions.push(session);
                            }
                        }
                    }
                }
            }
        }

        // Sort by start time (newest first)
        sessions.sort_by(|a, b| b.start_time.cmp(&a.start_time));

        Ok(sessions)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExportFormat {
    JSON,
    CSV,
    Markdown,
}

#[tauri::command]
pub async fn export_session(
    id: String,
    format: ExportFormat,
    handle: tauri::AppHandle,
    app_state: tauri::State<'_, crate::AppState>,
) -> Result<String, String> {
    let storage = app_state.storage.lock().map_err(|e| e.to_string())?;

    // Get the session
    let session = match storage.get_session(&id) {
        Ok(Some(session)) => session,
        Ok(None) => return Err(format!("Session with ID {} not found", id)),
        Err(e) => return Err(e.to_string()),
    };

    // Use the plugin dialog
    use tauri_plugin_dialog::DialogExt;

    // Create filter
    let filter_name = match format {
        ExportFormat::JSON => "JSON",
        ExportFormat::CSV => "CSV",
        ExportFormat::Markdown => "Markdown",
    };

    let filter_ext = match format {
        ExportFormat::JSON => "json",
        ExportFormat::CSV => "csv",
        ExportFormat::Markdown => "md",
    };

    // Prompt user for save location
    let file_path = handle
        .dialog()
        .file()
        .set_title("Export Session")
        .add_filter(filter_name, &[filter_ext])
        .blocking_save_file()
        .ok_or_else(|| format!("Failed to save file"))?;

    let save_path = file_path.as_path().expect("Failed to get save file path");

    // Export to the selected format
    match format {
        ExportFormat::JSON => export_json(&session, &save_path),
        ExportFormat::CSV => export_csv(&session, &save_path),
        ExportFormat::Markdown => export_markdown(&session, &save_path),
    }
    .map_err(|e| e.to_string())?;

    Ok("Session exported successfully".to_string())
}

fn export_json(session: &Session, path: &Path) -> Result<(), io::Error> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &session)?;
    Ok(())
}

fn export_csv(session: &Session, path: &Path) -> Result<(), io::Error> {
    let mut file = File::create(path)?;

    // Write header
    writeln!(file, "Marker ID,Timestamp,Label,Notes")?;

    // Write markers
    for marker in &session.markers {
        writeln!(
            file,
            "{},{},\"{}\",\"{}\"",
            marker.id,
            marker.timestamp.to_rfc3339(),
            marker.label.replace("\"", "\"\""),
            marker.notes.as_deref().unwrap_or("").replace("\"", "\"\"")
        )?;
    }

    Ok(())
}

fn export_markdown(session: &Session, path: &Path) -> Result<(), io::Error> {
    let mut file = File::create(path)?;

    // Write header
    writeln!(file, "# Session Report: {}", session.id)?;
    writeln!(
        file,
        "- **Start Time**: {}",
        session.start_time.format("%Y-%m-%d %H:%M:%S")
    )?;
    writeln!(
        file,
        "- **End Time**: {}",
        session.end_time.format("%Y-%m-%d %H:%M:%S")
    )?;

    let duration = session.end_time - session.start_time;
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    writeln!(file, "- **Duration**: {}h {}m {}s", hours, minutes, seconds)?;

    writeln!(file, "\n## Markers\n")?;
    writeln!(file, "| Time | Timestamp | Label | Notes |")?;
    writeln!(file, "|------|-----------|-------|-------|")?;

    for marker in &session.markers {
        let relative_time = marker.timestamp - session.start_time;
        let rel_minutes = relative_time.num_minutes();
        let rel_seconds = relative_time.num_seconds() % 60;

        writeln!(
            file,
            "| {}m {}s | {} | {} | {} |",
            rel_minutes,
            rel_seconds,
            marker.timestamp.format("%H:%M:%S"),
            marker.label,
            marker.notes.as_deref().unwrap_or("-")
        )?;
    }

    Ok(())
}
