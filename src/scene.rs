use image::ImageResult;

use crate::canvas::Canvas;
use crate::light::Light;
use crate::math::ray::Ray;
use crate::math::trace::{get_raycast_direction, trace_ray};
use crate::math::vector3::Vector3;
use crate::object::Object;

pub fn create_scene() -> Scene<NeedsCanvas> {
    Scene { state: NeedsCanvas }
}

pub struct Scene<S: SceneState> {
    state: S,
}

pub struct NeedsCanvas;

impl Scene<NeedsCanvas> {
    pub fn add_canvas(self, canvas: Canvas) -> Scene<NeedsViewport> {
        Scene {
            state: NeedsViewport { canvas },
        }
    }
}

pub struct NeedsViewport {
    canvas: Canvas,
}

impl Scene<NeedsViewport> {
    pub fn add_viewport(self, origin: Vector3, viewport: Vector3) -> Scene<NeedsLights> {
        Scene {
            state: NeedsLights {
                canvas: self.state.canvas,
                origin,
                viewport,
            },
        }
    }
}

pub struct NeedsLights {
    canvas: Canvas,
    origin: Vector3,
    viewport: Vector3,
}

impl Scene<NeedsLights> {
    pub fn add_lights(self, lights: Vec<Light>) -> Scene<NeedsObjects> {
        Scene {
            state: NeedsObjects {
                canvas: self.state.canvas,
                origin: self.state.origin,
                viewport: self.state.viewport,
                lights,
            },
        }
    }
}

pub struct NeedsObjects {
    canvas: Canvas,
    origin: Vector3,
    viewport: Vector3,
    lights: Vec<Light>,
}

impl Scene<NeedsObjects> {
    pub fn add_objects(self, objects: Vec<Object>) -> Scene<ReadyToRender> {
        Scene {
            state: ReadyToRender {
                canvas: self.state.canvas,
                origin: self.state.origin,
                viewport: self.state.viewport,
                lights: self.state.lights,
                objects,
            },
        }
    }
}

pub struct ReadyToRender {
    canvas: Canvas,
    origin: Vector3,
    viewport: Vector3,
    lights: Vec<Light>,
    objects: Vec<Object>,
}

impl Scene<ReadyToRender> {
    pub fn render(mut self) -> Scene<ReadyToSave> {
        let (width, height) = self.state.canvas.size;
        for x in (-1 * width as i32 / 2)..(width as i32 / 2) {
            for y in (-1 * height as i32 / 2)..(height as i32 / 2) {
                let direction =
                    get_raycast_direction(x, y, self.state.viewport, &self.state.canvas);
                let color = trace_ray(
                    self.state.objects.clone(),
                    self.state.lights.clone(),
                    Ray::new(self.state.origin, direction),
                    (1.0, f64::INFINITY),
                );
                self.state.canvas.write_pixel(x, y, color);
            }
        }

        Scene {
            state: ReadyToSave {
                canvas: self.state.canvas,
            },
        }
    }
}

pub struct ReadyToSave {
    canvas: Canvas,
}

impl Scene<ReadyToSave> {
    pub fn save(self, path: &str) -> ImageResult<()> {
        self.state.canvas.save(path)
    }
}

pub trait SceneState {}
impl SceneState for NeedsCanvas {}
impl SceneState for NeedsViewport {}
impl SceneState for NeedsLights {}
impl SceneState for NeedsObjects {}
impl SceneState for ReadyToRender {}
impl SceneState for ReadyToSave {}
