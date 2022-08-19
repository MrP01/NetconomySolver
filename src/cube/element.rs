use macroquad::prelude::{draw_cube, vec2, vec3, Vec2, Vec3, BLUE, BROWN};

pub trait Drawable {
  fn draw(&self);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElementType {
  Straight,
  Corner,
}

#[derive(Debug, Clone, Copy)]
pub struct Element {
  pub kind: ElementType,
  pub corner_orientation: Option<Vec2>,
  pub _position: Option<Vec3>,
  pub _direction: Option<Vec3>,
}
impl Drawable for Element {
  fn draw(&self) {
    if self._position.is_some() {
      match self.kind {
        ElementType::Straight => draw_cube(self._position.unwrap(), vec3(1., 1., 1.), None, BROWN),
        ElementType::Corner => draw_cube(self._position.unwrap(), vec3(1., 1., 1.), None, BLUE),
      }
    }
  }
}
impl Element {
  pub fn set(&mut self, pos: Vec3, dir: Vec3) {
    self._position = Some(pos);
    self._direction = Some(dir);
  }

  pub fn rotate_me(&mut self) {
    assert!(self.kind == ElementType::Corner);
    self.corner_orientation = Some(vec2(
      self.corner_orientation.unwrap().y,
      -self.corner_orientation.unwrap().x,
    ));
  }

  pub fn unknown_straight() -> Element {
    return Element {
      kind: ElementType::Straight,
      corner_orientation: None,
      _position: None,
      _direction: None,
    };
  }
  pub fn unknown_corner() -> Element {
    return Element {
      kind: ElementType::Corner,
      corner_orientation: Some(vec2(1., 0.)),
      _position: None,
      _direction: None,
    };
  }
  pub fn first_element() -> Element {
    return Element {
      kind: ElementType::Straight,
      corner_orientation: None,
      _position: Some(vec3(0., 0., 0.)),
      _direction: Some(vec3(1., 0., 0.)),
    };
  }
  pub fn last_element() -> Element {
    return Element {
      kind: ElementType::Straight,
      corner_orientation: None,
      _position: None,
      _direction: None,
    };
  }
}
