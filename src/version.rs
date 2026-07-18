use std::path::{Path, PathBuf};

use zed_extension_api::Worktree;

/// List of LÖVE version supported by the extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoveVersion {
    V11_5,
    V11_4,
    V11_3,
}

impl LoveVersion {
    /// Converting a version string into a LoveVersion.
    pub fn from_str(version: &str) -> Option<Self> {
        match version {
            "11.5" => Some(Self::V11_5),
            "11.4" => Some(Self::V11_4),
            "11.3" => Some(Self::V11_3),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::V11_5 => "11.5",
            Self::V11_4 => "11.4",
            Self::V11_3 => "11.3",
        }
    }

    /// Directory inside assets/library/.
    pub fn directory(self) -> &'static str {
        self.name()
    }

    /// Lua runtime used by LuaLS.
    pub fn runtime(self) -> &'static str {
        match self {
            Self::V11_5 | Self::V11_4 | Self::V11_3 => "LuaJIT",
        }
    }
}

/// Detect the LÖVE version used by a project.
pub fn detect(worktree: &Worktree) -> LoveVersion {
    if let Ok(conf) = worktree.read_text_file("conf.lua") {
        if let Some(version) = parse_conf(&conf) {
            return version;
        }
    }

    // Default to the latest bundled version.
    LoveVersion::V11_5
}

/// Build the path to the LuaCATS library for the selected version.
pub fn library_path(root: &Path, version: LoveVersion) -> PathBuf {
    root.join(version.directory())
}

/// Parse the LÖVE version from conf.lua.
fn parse_conf(conf: &str) -> Option<LoveVersion> {
    for version in ["11.5", "11.4", "11.3"] {
        if conf.contains(&format!("\"{version}\"")) || conf.contains(&format!("'{version}'")) {
            return LoveVersion::from_str(version);
        }
    }

    None
}
