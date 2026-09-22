fn main() {
    #[cfg(windows)]
    {
        embed_resource::compile("app.rc", embed_resource::NONE);
    }
    tauri_build::build()
}
