use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use three_d::{
    EffectMaterialId, Gm, InnerSpace, Material, Mesh, RenderStates,
    Srgba, Vec3, Program, Viewer, Light, MaterialType,
};

// =====================================================
// Material
// =====================================================

#[derive(Clone)]
pub struct CustomMaterial {
    pub color: [f32; 3],
}

impl Material for CustomMaterial {
    fn render_states(&self) -> RenderStates {
        let mut renderstates = RenderStates::default();
        renderstates.depth_test = three_d::DepthTest::Less;
        renderstates.cull = three_d::Cull::Back;
        renderstates
    }

    fn id(&self) -> EffectMaterialId {
        EffectMaterialId(1)
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _viewer: &dyn Viewer,
        _lights: &[&dyn Light],
    ) {
        program.use_uniform("lightDirection", three_d::vec3(0.5, 0.5, 0.5).normalize());
        program.use_uniform("lightPos", three_d::vec3(0.5, 0.5, 0.5));
        program.use_uniform("baseColor", three_d::vec3(
            self.color[0],
            self.color[1],
            self.color[2],
        ));
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("Shaders/defaultfragment.glsl").to_owned()
    }
}

// =====================================================
// Type Aliases
// =====================================================

pub type InstanceRef = Rc<RefCell<dyn Instance>>;
pub type WeakInstanceRef = Weak<RefCell<dyn Instance>>;

// =====================================================
// Instance Trait (Scene Graph)
// =====================================================

pub trait Instance {
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);

    fn get_children(&self) -> &Vec<InstanceRef>;
    fn get_children_mut(&mut self) -> &mut Vec<InstanceRef>;

    fn get_parent(&self) -> Option<WeakInstanceRef>;
    fn set_parent(&mut self, parent: Option<WeakInstanceRef>);

    fn add_child(&mut self, child: &InstanceRef);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    // Recursive traversal
    fn get_descendants(&self) -> Vec<InstanceRef> {
        let mut result = Vec::new();

        for child in self.get_children() {
            result.push(child.clone());
            result.extend(child.borrow().get_descendants());
        }

        result
    }
}

// =====================================================
// Object Trait (Renderable objects)
// =====================================================

pub trait Object: Instance {
    fn get_renderable(&self) -> &Gm<Mesh, CustomMaterial>;
    fn get_renderable_mut(&mut self) -> &mut Gm<Mesh, CustomMaterial>;
}

// =====================================================
// Part (Owns the GPU object)
// =====================================================

pub struct Part {
    name: String,
    children: Vec<InstanceRef>,
    parent: Option<WeakInstanceRef>,

    renderable: Gm<Mesh, CustomMaterial>,

    pub position: Vec3,
    pub rotation: Vec3,
    pub color: Srgba,
}

impl Part {
    pub fn new(name: &str, mesh: Mesh) -> Self {
        Self {
            name: name.to_string(),
            children: vec![],
            parent: None,
            renderable: Gm::new(
                mesh,
                CustomMaterial {
                    color: [1.0, 0.0, 0.0],
                },
            ),
            position: three_d::vec3(0.0, 0.0, 0.0),
            rotation: three_d::vec3(0.0, 0.0, 0.0),
            color: Srgba::RED
        }
    }
}

// ---------------- Instance Impl ----------------

impl Instance for Part {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_children(&self) -> &Vec<InstanceRef> {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut Vec<InstanceRef> {
        &mut self.children
    }

    fn get_parent(&self) -> Option<WeakInstanceRef> {
        self.parent.clone()
    }

    fn set_parent(&mut self, parent: Option<WeakInstanceRef>) {
        self.parent = parent;
    }

    fn add_child(&mut self, child: &InstanceRef) {
        self.children.push(child.clone());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// ---------------- Object Impl ----------------

impl Object for Part {
    fn get_renderable(&self) -> &Gm<Mesh, CustomMaterial> {
        &self.renderable
    }

    fn get_renderable_mut(&mut self) -> &mut Gm<Mesh, CustomMaterial> {
        &mut self.renderable
    }
}

// =====================================================
// Workspace (Root Node — NO renderable storage)
// =====================================================

pub struct Workspace {
    children: Vec<InstanceRef>,
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            children: vec![],
        }
    }
}

impl Instance for Workspace {
    fn get_name(&self) -> String {
        "Workspace".into()
    }

    fn set_name(&mut self, _name: String) {}

    fn get_children(&self) -> &Vec<InstanceRef> {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut Vec<InstanceRef> {
        &mut self.children
    }

    fn get_parent(&self) -> Option<WeakInstanceRef> {
        None
    }

    fn set_parent(&mut self, _parent: Option<WeakInstanceRef>) {}

    fn add_child(&mut self, child: &InstanceRef) {
        self.children.push(child.clone());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}