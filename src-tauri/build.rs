fn main() {
    #[cfg(windows)]
    {
        embed_resource::compile("app.manifest", embed_resource::NONE);
    }
    tauri_build::build()
}
