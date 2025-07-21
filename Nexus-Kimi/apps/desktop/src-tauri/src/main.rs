use threadspace_memory::Memory;
use tauri::State;

struct AppState {
    memory: Memory,
}

#[tauri::command]
async fn query_memory(id: String, state: State<'_, AppState>) -> Result<Option<String>, String> {
    state
        .memory
        .get(&id)
        .map(|r| r.map(|rec| rec.data))
        .map_err(|e| e.to_string())
}

fn main() {
    let memory = Memory::new("threadspace.db").expect("open memory");
    tauri::Builder::default()
        .manage(AppState { memory })
        .invoke_handler(tauri::generate_handler![query_memory])
        .run(tauri::generate_context!())
        .expect("error while running tauri");
}