#[allow(non_camel_case_types)]
pub enum AnimRate {
    k30_fps,
    k480_fpb,
    k30_fps_ui,
    k1_fpb,
    k30_fps_tutorial
}

pub trait Anim {
    fn get_frame(&self) -> f32;
    fn set_frame(&mut self, frame: f32);

    fn get_rate(&self) -> &AnimRate;
    fn get_rate_mut(&mut self) -> &mut AnimRate;
    fn set_rate(&mut self, rate: AnimRate);
}