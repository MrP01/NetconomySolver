use macroquad::prelude::{draw_cube, ivec2, ivec3, IVec2, IVec3, BLUE, BROWN};

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
  pub corner_orientation: Option<IVec2>,
  pub _position: Option<IVec3>,
  pub _direction: Option<IVec3>,
}
impl Drawable for Element {
  fn draw(&self) {
    if self._position.is_some() {
      match self.kind {
        ElementType::Straight => draw_cube(
          self._position.unwrap().as_f32(),
          ivec3(1, 1, 1).as_f32(),
          None,
          BROWN,
        ),
        ElementType::Corner => draw_cube(
          self._position.unwrap().as_f32(),
          ivec3(1, 1, 1).as_f32(),
          None,
          BLUE,
        ),
      }
    }
  }
}
impl Element {
  pub fn set(&mut self, pos: IVec3, dir: IVec3) {
    self._position = Some(pos);
    self._direction = Some(dir);
  }

  pub fn rotate_me(&mut self) {
    assert!(self.kind == ElementType::Corner);
    self.corner_orientation = Some(ivec2(
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
      corner_orientation: Some(ivec2(1, 0)),
      _position: None,
      _direction: None,
    };
  }
  pub fn first_element() -> Element {
    return Element {
      kind: ElementType::Straight,
      corner_orientation: None,
      _position: Some(ivec3(0, 0, 0)),
      _direction: Some(ivec3(1, 0, 0)),
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
