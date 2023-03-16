use std::marker::PhantomData;

use crate::math::vector3::Vector3;

use super::{canvas::Canvas, light::Light, object::Object};

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub viewport: Vector3,
    pub origin: Vector3,
    pub canvas: Canvas,
}

impl Scene {
    pub fn new() -> UnresolvedScene<Unfulfilled, Unfulfilled> {
        UnresolvedScene {
            marker: PhantomData,
            objects: vec![],
            lights: vec![],
            viewport: None,
            origin: None,
            canvas: None,
        }
    }
}

pub struct UnresolvedScene<HasCanvas, HasViewport>
where
    HasCanvas: SceneRequirement,
    HasViewport: SceneRequirement,
{
    pub marker: PhantomData<(HasCanvas, HasViewport)>,
    pub viewport: Option<Vector3>,
    pub origin: Option<Vector3>,
    pub canvas: Option<Canvas>,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl<A> UnresolvedScene<Unfulfilled, A>
where
    A: SceneRequirement,
{
    pub fn add_canvas(self, canvas: Canvas) -> UnresolvedScene<Fulfilled, A> {
        UnresolvedScene {
            viewport: self.viewport,
            objects: self.objects,
            canvas: Some(canvas),
            origin: self.origin,
            lights: self.lights,
            marker: PhantomData,
        }
    }
}

impl<A> UnresolvedScene<A, Unfulfilled>
where
    A: SceneRequirement,
{
    pub fn add_viewport(self, viewport: Vector3, origin: Vector3) -> UnresolvedScene<A, Fulfilled> {
        UnresolvedScene {
            viewport: Some(viewport),
            objects: self.objects,
            origin: Some(origin),
            canvas: self.canvas,
            lights: self.lights,
            marker: PhantomData,
        }
    }
}

impl<A, B> UnresolvedScene<A, B>
where
    A: SceneRequirement,
    B: SceneRequirement,
{
    pub fn add_object(mut self, object: Object) -> UnresolvedScene<A, B> {
        self.objects.push(object);
        self
    }

    pub fn add_light(mut self, light: Light) -> UnresolvedScene<A, B> {
        self.lights.push(light);
        self
    }
}

impl UnresolvedScene<Fulfilled, Fulfilled> {
    pub fn crystalize(self) -> Scene {
        Scene {
            viewport: self.viewport.unwrap(),
            origin: self.origin.unwrap(),
            canvas: self.canvas.unwrap(),
            objects: self.objects,
            lights: self.lights,
        }
    }
}

pub trait SceneRequirement {}

pub struct Fulfilled;
pub struct Unfulfilled;

impl SceneRequirement for Fulfilled {}
impl SceneRequirement for Unfulfilled {}
