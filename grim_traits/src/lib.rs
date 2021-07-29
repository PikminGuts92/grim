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

    fn get_rate(&self) -> AnimRate;
    fn set_rate(&mut self, rate: AnimRate);
}

pub struct Sphere {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}

pub trait Draw {
    fn get_showing(&self) -> bool;
    fn set_showing(&mut self, showing: bool);

    fn get_bounding(&self) -> Sphere;
    fn set_bounding(&mut self, sphere: Sphere);

    fn get_draw_order(&self) -> f32;
    fn set_draw_order(&mut self, draw_order: f32);
}

pub trait Poll {
    fn get_target_1(&self) -> &str;
    fn set_target_1(&mut self, target: &str);

    fn get_target_2(&self) -> &str;
    fn set_target_2(&mut self, target: &str);
}