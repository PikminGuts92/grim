#![allow(unused_imports)]

mod apps;
use apps::SceneTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scene = SceneTool::new();
    scene.run()
}