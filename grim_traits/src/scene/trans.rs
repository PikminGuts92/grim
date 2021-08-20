use super::{Matrix, MiloObject};

#[allow(non_camel_case_types)]
pub enum TransConstraint {
    kConstraintNone,
    kConstraintLocalRotate,
    kConstraintParentWorld,
    kConstraintLookAtTarget,
    kConstraintShadowTarget,
    kConstraintBillboardZ,
    kConstraintBillboardXZ,
    kConstraintBillboardXYZ,
    kConstraintFastBillboardXYZ
}

pub trait Trans : MiloObject {
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