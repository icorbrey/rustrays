use std::marker::PhantomData;

use crate::{canvas::Canvas, light::Light, math::vector3::Vector3, object::Object};

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub viewport: Vector3,
    pub origin: Vector3,
    pub canvas: Canvas,
}

pub struct UnresolvedScene {
    pub viewport: Option<Vector3>,
    pub origin: Option<Vector3>,
    pub canvas: Option<Canvas>,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

pub fn build_scene() -> SceneBuilder<Unfulfilled, Unfulfilled> {
    SceneBuilder {
        scene: UnresolvedScene {
            viewport: None,
            origin: None,
            canvas: None,
            objects: vec![],
            lights: vec![],
        },
        marker: PhantomData,
    }
}

pub struct SceneBuilder<HasCanvas, HasViewport>
where
    HasCanvas: SceneState,
    HasViewport: SceneState,
{
    scene: UnresolvedScene,
    marker: PhantomData<(HasCanvas, HasViewport)>,
}

impl<A> SceneBuilder<Unfulfilled, A>
where
    A: SceneState,
{
    pub fn add_canvas(mut self, canvas: Canvas) -> SceneBuilder<Fulfilled, A> {
        self.scene.canvas = Some(canvas);
        SceneBuilder {
            scene: self.scene,
            marker: PhantomData,
        }
    }
}

impl<A> SceneBuilder<A, Unfulfilled>
where
    A: SceneState,
{
    pub fn add_viewport(
        mut self,
        viewport: Vector3,
        origin: Vector3,
    ) -> SceneBuilder<A, Fulfilled> {
        self.scene.viewport = Some(viewport);
        self.scene.origin = Some(origin);
        SceneBuilder {
            scene: self.scene,
            marker: PhantomData,
        }
    }
}

impl<A, B> SceneBuilder<A, B>
where
    A: SceneState,
    B: SceneState,
{
    pub fn add_object(mut self, object: Object) -> SceneBuilder<A, B> {
        self.scene.objects.push(object);
        self
    }

    pub fn add_light(mut self, light: Light) -> SceneBuilder<A, B> {
        self.scene.lights.push(light);
        self
    }
}

impl SceneBuilder<Fulfilled, Fulfilled> {
    pub fn crystalize(self) -> Scene {
        Scene {
            viewport: self.scene.viewport.unwrap(),
            origin: self.scene.origin.unwrap(),
            canvas: self.scene.canvas.unwrap(),
            objects: self.scene.objects,
            lights: self.scene.lights,
        }
    }
}

pub trait SceneState {}

pub struct Fulfilled;
pub struct Unfulfilled;

impl SceneState for Fulfilled {}
impl SceneState for Unfulfilled {}
