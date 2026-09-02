use crate::state::AppState;
use domain::Contact;
use tauri::State;

#[derive(serde::Serialize)]
pub struct ContactSearchResult {
    pub snapshot_id: String,
    pub contact: Contact,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn search_contacts(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<ContactSearchResult>, String> {
    let results = state
        .engine
        .search_contacts(&query)
        .map_err(|e| e.to_string())?;

    Ok(results
        .into_iter()
        .map(|(s_id, contact)| ContactSearchResult {
            snapshot_id: s_id.0,
            contact,
        })
        .collect())
}
