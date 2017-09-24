use glium;
use cgm;

use color::Color;
use std::rc::Rc;
use glium::texture::Texture2d;
use std::iter::{Chain, Once, once};

mod translate;
mod rotate;
mod combine;

pub use self::translate::*;
pub use self::rotate::*;
pub use self::combine::*;

/// Trait for structs to be drawn with `Frame::draw`
pub trait Shape: IntoIterator<Item = RendTri> {
    /// Combine shapes together so they become one shape.
    fn combine<S: Shape>(self, rhs: S) -> Combine<Self, S> where Self: Sized {
        Combine(self, rhs)
    }

    /// Translate a shape using a `vector` which represents the direction and magnitude to translate it by.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5]).translate([0.1, 0.1]));
    /// ```
    fn translate<V: Into<cgm::Vector2<f32>>>(&self, vector: V) -> Translate<Self> where Self: Clone {
        Translate::new(self.clone(), vector.into())
    }

    /// Rotate a shape using an angle in radians.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// use std::f32::consts::PI;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5]).rotate(PI));
    /// ```
    fn rotate(&self, angle: f32) -> Rotate<Self> where Self: Clone {
        Rotate::new(self.clone(), angle)
    }
}

impl<S> Shape for S where S: IntoIterator<Item = RendTri> {}

/// Renderable triangle which includes color and texture information.
pub struct RendTri {
    pub(crate) tri: Tri,
    pub(crate) texture: Option<Rc<Texture2d>>,
}

impl RendTri {
    #[inline]
    fn map_pos<F: FnMut(cgm::Point2<f32>) -> cgm::Point2<f32>>(mut self, f: F) -> RendTri {
        self.tri.positions = self.tri.positions.map(f);
        self
    }

    #[inline]
    fn map_tex<F: FnMut(cgm::Point2<f32>) -> cgm::Point2<f32>>(mut self, f: F) -> RendTri {
        self.tri.texcoords = self.tri.texcoords.map(f);
        self
    }

    #[inline]
    fn map_color<F: FnMut([f32; 4]) -> [f32; 4]>(mut self, mut f: F) -> RendTri {
        self.tri.color = f(self.tri.color);
        self
    }

    #[inline]
    fn map_texture<T: Into<Option<Rc<Texture2d>>>>(mut self, t: T) -> RendTri {
        self.texture = t.into();
        self
    }
}

impl From<Tri> for RendTri {
    fn from(tri: Tri) -> Self {
        RendTri {
            tri: tri,
            texture: None,
        }
    }
}

/// Three positions which form a matrix for shader purposes
#[derive(Copy, Clone, Debug)]
pub struct Positions(pub [[f32; 2]; 3]);

impl Positions {
    fn map<F: FnMut(cgm::Point2<f32>) -> cgm::Point2<f32>>(self, mut f: F) -> Positions {
        Positions([f(self.0[0].into()).into(), f(self.0[1].into()).into(), f(self.0[2].into()).into()])
    }
}

/// A triangle primitive which enters the shader pipeline as a single vertex and is the only primitive in nest
#[derive(Copy, Clone, Debug)]
pub struct Tri {
    /// The three space vertices of the triangles
    pub positions: Positions,
    /// The three texture coordinates of the above vertices
    pub texcoords: Positions,
    /// The color of this triangle.
    pub color: [f32; 4],
}

impl Tri {
    /// Create a new triangle with points and tex coordinates specified
    #[inline]
    pub fn new<P: Into<cgm::Point2<f32>> + Copy, T: Into<cgm::Point2<f32>> + Copy, C: Into<Color>>(
        positions: [P; 3],
        texcoords: [T; 3],
        color: C,
    ) -> Tri {
        Tri {
            positions: Positions(
                [
                    positions[0].into().into(),
                    positions[1].into().into(),
                    positions[2].into().into(),
                ],
            ),
            texcoords: Positions(
                [
                    texcoords[0].into().into(),
                    texcoords[1].into().into(),
                    texcoords[2].into().into(),
                ],
            ),
            color: color.into().0,
        }
    }

    /// Create a new triangle with points with coordinates to create a single-color triangle
    #[inline]
    pub fn new_pos<P: Into<cgm::Point2<f32>> + Copy>(positions: [P; 3]) -> Tri {
        Tri::new(
            [
                positions[0].into(),
                positions[1].into(),
                positions[2].into(),
            ],
            [cgm::Point2::new(0.0, 0.0); 3],
            Color::WHITE,
        )
    }
}

implement_vertex!(Tri, positions, texcoords, color);

unsafe impl glium::vertex::Attribute for Positions {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x2x3
    }
}

/// Two points make a rectangle.
#[derive(Copy, Clone, Debug)]
pub struct Rect(pub [f32; 2], pub [f32; 2]);

impl IntoIterator for Rect {
    type IntoIter = Chain<Once<RendTri>, Once<RendTri>>;
    type Item = RendTri;
    fn into_iter(self) -> Self::IntoIter {
        Iterator::chain(once(Tri::new_pos(
            [
                [self.0[0], self.0[1]],
                [self.1[0], self.0[1]],
                [self.0[0], self.1[1]],
            ],
        ).into()), once(Tri::new_pos(
            [
                [self.1[0], self.1[1]],
                [self.0[0], self.1[1]],
                [self.1[0], self.0[1]],
            ],
        ).into()))
    }
}

/// Takes two points and creates a rectangle from those two points.
pub fn rect<A: Into<cgm::Point2<f32>>, B: Into<cgm::Point2<f32>>>(first: A, second: B) -> Rect {
    Rect(first.into().into(), second.into().into())
}