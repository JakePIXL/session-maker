use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub label: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub markers: Vec<Marker>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSession {
    pub id: String,
    pub start_time: DateTime<Utc>,
    pub markers: Vec<Marker>,
}

impl ActiveSession {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            start_time: Utc::now(),
            markers: Vec::new(),
        }
    }

    pub fn add_marker(&mut self, label: &str) -> &Marker {
        let marker = Marker {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            label: label.to_string(),
            notes: None,
        };

        self.markers.push(marker);
        self.markers.last().unwrap()
    }

    pub fn to_session(&self) -> Session {
        Session {
            id: self.id.clone(),
            start_time: self.start_time,
            end_time: Utc::now(),
            markers: self.markers.clone(),
            name: None,
        }
    }
}

// Tauri command functions
#[tauri::command]
pub async fn start_session(app_state: tauri::State<'_, crate::AppState>) -> Result<String, String> {
    let mut session = app_state.active_session.lock().map_err(|e| e.to_string())?;

    if session.is_some() {
        return Err("A session is already in progress".to_string());
    }

    let new_session = ActiveSession::new();
    let id = new_session.id.clone();
    *session = Some(new_session);

    Ok(id)
}

#[tauri::command]
pub async fn stop_session(app_state: tauri::State<'_, crate::AppState>) -> Result<Session, String> {
    let mut session_guard = app_state.active_session.lock().map_err(|e| e.to_string())?;

    match session_guard.take() {
        Some(active) => {
            let completed = active.to_session();

            // Save to storage
            let mut storage = app_state.storage.lock().map_err(|e| e.to_string())?;
            storage
                .save_session(&completed)
                .map_err(|e| e.to_string())?;

            Ok(completed)
        }
        None => Err("No active session".to_string()),
    }
}

#[tauri::command]
pub async fn add_marker(
    label: String,
    app_state: tauri::State<'_, crate::AppState>,
) -> Result<Marker, String> {
    let mut session = app_state.active_session.lock().map_err(|e| e.to_string())?;

    match *session {
        Some(ref mut active) => {
            let marker = active.add_marker(&label);
            Ok(marker.clone())
        }
        None => Err("No active session".to_string()),
    }
}

#[tauri::command]
pub async fn get_active_session(
    app_state: tauri::State<'_, crate::AppState>,
) -> Result<Option<ActiveSession>, String> {
    let session = app_state.active_session.lock().map_err(|e| e.to_string())?;
    Ok(session.clone())
}

#[tauri::command]
pub async fn get_sessions(
    app_state: tauri::State<'_, crate::AppState>,
) -> Result<Vec<Session>, String> {
    let storage = app_state.storage.lock().map_err(|e| e.to_string())?;
    storage.list_sessions().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_session_by_id(
    id: String,
    app_state: tauri::State<'_, crate::AppState>,
) -> Result<Option<Session>, String> {
    let storage = app_state.storage.lock().map_err(|e| e.to_string())?;
    storage.get_session(&id).map_err(|e| e.to_string())
}
