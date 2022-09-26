use super::{Matrix, Object, ObjectId};

pub trait ObjectDir : Object {
    fn get_entries_ids(&self) -> &Vec<ObjectId>;
    fn get_entries_ids_mut(&mut self) -> &mut Vec<ObjectId>;
    fn set_entries_ids(&mut self, ids: Vec<ObjectId>);

    fn get_subdirs_ids(&self) -> &Vec<ObjectId>;
    fn get_subdirs_ids_mut(&mut self) -> &mut Vec<ObjectId>;
    fn set_subdirs_ids(&mut self, ids: Vec<ObjectId>);

    // TODO: Add proxy_file + inline_subdir + path_name

    fn get_viewports(&self) -> &[Matrix; 7];
    fn get_viewports_mut(&mut self) -> &mut [Matrix; 7];
    fn set_viewports(&mut self, viewports: [Matrix; 7]);

    fn get_curr_viewport_index(&self) -> u32;
    fn set_curr_viewport_index(&mut self, index: u32);
}