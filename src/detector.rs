use zed_extension_api::Worktree;

pub fn is_love_project(worktree: &Worktree) -> bool {
    worktree.read_text_file("main.lua").is_ok() || worktree.read_text_file("conf.lua").is_ok()
}
