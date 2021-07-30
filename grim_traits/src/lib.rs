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

pub struct Sphere {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}

pub trait Draw {
    fn get_showing(&self) -> bool;
    fn set_showing(&mut self, showing: bool);

    fn get_bounding(&self) -> &Sphere;
    fn get_bounding_mut(&mut self) -> &mut Sphere;
    fn set_bounding(&mut self, sphere: Sphere);

    fn get_draw_order(&self) -> f32;
    fn set_draw_order(&mut self, draw_order: f32);
}

pub struct Color4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub struct BoneTrans {
    pub name: String,
    pub trans: Matrix,
}

pub struct UV {
    pub u: f32,
    pub v: f32,
}

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub struct Vertex {
    pub pos: Vector4,
    pub normals: Vector4,
    pub color: Color4,
    pub uv: UV,
}

pub trait Mesh : Draw + Trans {
    fn get_material(&self) -> &String;
    fn get_material_mut(&mut self) -> &mut String;
    fn set_material(&mut self, mat: String);

    fn get_mesh_name(&self) -> &String;
    fn get_mesh_name_mut(&mut self) -> &mut String;
    fn set_mesh_name(&mut self, mesh: String);

    fn get_vertices(&self) -> &Vec<Vertex>;
    fn get_vertices_mut(&mut self) -> &mut Vec<Vertex>;
    fn set_vertices(&mut self, vertices: Vec<Vertex>);

    fn get_faces(&self) -> &Vec<[u16; 3]>;
    fn get_faces_mut(&mut self) -> &mut Vec<[u16; 3]>;
    fn set_faces(&mut self, faces: Vec<[u16; 3]>);

    fn get_face_groups(&self) -> &Vec<u8>;
    fn get_face_groups_mut(&mut self) -> &mut Vec<u8>;
    fn set_face_groups(&mut self, face_groups: Vec<u8>);

    fn get_bones(&self) -> &Vec<BoneTrans>;
    fn get_bones_mut(&mut self) -> &mut Vec<BoneTrans>;
    fn set_bones(&mut self, bones: Vec<BoneTrans>);
}

pub trait Poll {
    fn get_target_1(&self) -> &String;
    fn get_target_1_mut(&mut self) -> &mut String;
    fn set_target_1(&mut self, target: String);

    fn get_target_2(&self) -> &String;
    fn get_target_2_mut(&mut self) -> &mut String;
    fn set_target_2(&mut self, target: String);
}

pub struct Matrix {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
}

pub enum TransConstraint {
    KConstraintNone,
    KConstraintLocalRotate,
    KConstraintParentWorld,
    KConstraintLookAtTarget,
    KConstraintShadowTarget,
    KConstraintBillboardZ,
    KConstraintBillboardXZ,
    KConstraintBillboardXYZ,
    KConstraintFastBillboardXYZ
}

pub trait Trans {
    fn get_local_transform(&self) -> &Matrix;
    fn get_local_transform_mut(&mut self) -> &mut Matrix;
    fn set_local_transform(&mut self, transform: Matrix);

    fn get_world_transform(&self) -> &Matrix;
    fn get_world_transform_mut(&mut self) -> &mut Matrix;
    fn set_world_transform(&mut self, transform: Matrix);

    fn get_constraint(&self) -> &TransConstraint;
    fn get_constraint_mut(&mut self) -> &mut TransConstraint;
    fn set_constraint(&mut self, constraint: TransConstraint);

    fn get_target(&self) -> &String;
    fn get_target_mut(&mut self) -> &mut String;
    fn set_target(&mut self, target: String);

    fn get_preserve_scale(&self) -> bool;
    fn set_preserve_scale(&mut self, preserve_scale: bool);

    fn get_parent(&self) -> &String;
    fn get_parent_mut(&mut self) -> &mut String;
    fn set_parent(&mut self, parent: String);
}