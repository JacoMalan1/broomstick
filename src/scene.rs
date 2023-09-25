use crate::graphics::{camera::Camera, drawable::Draw};
use glium::Frame;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub trait SceneObject: Draw {}

pub struct Scene {
    objects: HashMap<Uuid, Arc<dyn SceneObject>>,
    camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            objects: HashMap::new(),
        }
    }

    pub fn add_object(&mut self, obj: Arc<dyn SceneObject>) -> Uuid {
        let uuid = Uuid::new_v4();
        self.objects.insert(uuid, obj);
        uuid
    }

    pub fn remove_object(&mut self, uuid: &Uuid) -> Option<Arc<dyn SceneObject>> {
        self.objects.remove(uuid)
    }

    pub fn draw_all(&self, f: &mut Frame) -> anyhow::Result<()> {
        self.objects.iter().for_each(|x| x.1.draw(f, &self.camera));
        Ok(())
    }

    pub fn draw_uuid(&self, uuid: &Uuid, f: &mut Frame) {
        if let Some(obj) = self.objects.get(uuid) {
            obj.draw(f, &self.camera);
        }
    }
}
