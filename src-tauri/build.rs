extern crate embed_resource;

fn main() {
    embed_resource::compile("program.rc");
    tauri_build::build()
}
