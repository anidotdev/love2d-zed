use std::env;
use std::path::{Path, PathBuf};

use zed_extension_api::serde_json::{Value, json};

const LOVE_RUNTIME: &str = "LuaJIT";
const LOVE_GLOBAL: &str = "love";

pub fn library_path() -> Result<PathBuf, String> {
    let extension_dir =
        env::current_dir().map_err(|e| format!("failed to determine extension directory: {e}"))?;

    Ok(extension_dir.join("assets").join("library"))
}

pub fn apply_love_defaults(settings: &mut Value, library: &Path) {
    let lua = ensure_object(settings, "Lua");

    // Runtime
    let runtime = ensure_object(lua, "runtime");
    runtime["version"] = json!(LOVE_RUNTIME);

    // Diagnostics
    let diagnostics = ensure_object(lua, "diagnostics");
    let globals = ensure_array(diagnostics, "globals");

    if !globals.iter().any(|v| v == LOVE_GLOBAL) {
        globals.push(json!(LOVE_GLOBAL));
    }

    // Workspace
    let workspace = ensure_object(lua, "workspace");
    let libraries = ensure_array(workspace, "library");

    let library = library.to_string_lossy().to_string();

    if !libraries.iter().any(|v| v == &json!(library)) {
        libraries.push(json!(library));
    }
}

fn ensure_object<'a>(parent: &'a mut Value, key: &str) -> &'a mut Value {
    if !parent[key].is_object() {
        parent[key] = json!({});
    }

    &mut parent[key]
}

fn ensure_array<'a>(parent: &'a mut Value, key: &str) -> &'a mut Vec<Value> {
    if !parent[key].is_array() {
        parent[key] = json!([]);
    }

    parent[key].as_array_mut().expect("expected JSON array")
}
