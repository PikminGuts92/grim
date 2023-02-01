#![allow(unused_imports)]

mod apps;
use apps::AudioTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tool = AudioTool::new();
    tool.run()
}