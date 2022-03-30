#![allow(unused_imports)]

mod apps;
use apps::MeshTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scene = MeshTool::new();
    scene.run()
}