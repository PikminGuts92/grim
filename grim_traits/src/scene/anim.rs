pub enum AnimRate {
    K30Fps,
    K480Fpb,
    K30FpsUi,
    K1Fpb,
    K30FpsTutorial,
}

pub trait Anim {
    fn get_frame(&self) -> f32;
    fn set_frame(&mut self, frame: f32);

    fn get_rate(&self) -> &AnimRate;
    fn get_rate_mut(&mut self) -> &mut AnimRate;
    fn set_rate(&mut self, rate: AnimRate);
}