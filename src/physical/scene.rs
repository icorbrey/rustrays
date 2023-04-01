use std::marker::PhantomData;

use crate::math::{color::Color, vector3::Vector3};

use super::{canvas::Canvas, light::Light, object::Object};

pub struct Scene {
    pub background_color: Color,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub viewport: Vector3,
    pub origin: Vector3,
    pub canvas: Canvas,
}

impl Scene {
    pub fn new() -> UnresolvedScene<No, No, No> {
        UnresolvedScene {
            background_color: None,
            marker: PhantomData,
            objects: vec![],
            lights: vec![],
            viewport: None,
            origin: None,
            canvas: None,
        }
    }
}

pub struct UnresolvedScene<HasCanvas, HasViewport, HasBackgroundColor>
where
    HasCanvas: SceneRequirement,
    HasViewport: SceneRequirement,
    HasBackgroundColor: SceneRequirement,
{
    pub marker: PhantomData<(HasCanvas, HasViewport, HasBackgroundColor)>,
    pub background_color: Option<Color>,
    pub viewport: Option<Vector3>,
    pub origin: Option<Vector3>,
    pub canvas: Option<Canvas>,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl<A, B> UnresolvedScene<No, A, B>
where
    A: SceneRequirement,
    B: SceneRequirement,
{
    pub fn add_canvas(self, canvas: Canvas) -> UnresolvedScene<Yes, A, B> {
        UnresolvedScene {
            background_color: self.background_color,
            viewport: self.viewport,
            objects: self.objects,
            canvas: Some(canvas),
            lights: self.lights,
            origin: self.origin,
            marker: PhantomData,
        }
    }
}

impl<A, B> UnresolvedScene<A, No, B>
where
    A: SceneRequirement,
    B: SceneRequirement,
{
    pub fn add_viewport(self, viewport: Vector3, origin: Vector3) -> UnresolvedScene<A, Yes, B> {
        UnresolvedScene {
            background_color: self.background_color,
            viewport: Some(viewport),
            objects: self.objects,
            origin: Some(origin),
            canvas: self.canvas,
            lights: self.lights,
            marker: PhantomData,
        }
    }
}

impl<A, B> UnresolvedScene<A, B, No>
where
    A: SceneRequirement,
    B: SceneRequirement,
{
    pub fn add_background_color(self, color: Color) -> UnresolvedScene<A, B, Yes> {
        UnresolvedScene {
            background_color: Some(color),
            viewport: self.viewport,
            objects: self.objects,
            canvas: self.canvas,
            lights: self.lights,
            origin: self.origin,
            marker: PhantomData,
        }
    }
}

impl<A, B, C> UnresolvedScene<A, B, C>
where
    A: SceneRequirement,
    B: SceneRequirement,
    C: SceneRequirement,
{
    pub fn add_object(mut self, object: Object) -> UnresolvedScene<A, B, C> {
        self.objects.push(object);
        self
    }

    pub fn add_light(mut self, light: Light) -> UnresolvedScene<A, B, C> {
        self.lights.push(light);
        self
    }
}

impl UnresolvedScene<Yes, Yes, Yes> {
    pub fn crystalize(self) -> Scene {
        Scene {
            background_color: self.background_color.unwrap(),
            viewport: self.viewport.unwrap(),
            origin: self.origin.unwrap(),
            canvas: self.canvas.unwrap(),
            objects: self.objects,
            lights: self.lights,
        }
    }
}

pub trait SceneRequirement {}

pub struct Yes;
pub struct No;

impl SceneRequirement for Yes {}
impl SceneRequirement for No {}
