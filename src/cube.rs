pub mod element;
pub mod utils;
use element::{Drawable, Element, ElementType};
use macroquad::prelude::{ivec3, IVec3};

const DOF_PER_CORNER: usize = 4;

pub struct AlgoState {
  pub cursor: usize,
}

pub struct NetconomyCube {
  elements: Vec<Element>,
  _corner_indices: Vec<usize>,
  pub _algo_state: AlgoState,
}
impl Drawable for NetconomyCube {
  fn draw(&self) {
    for element in &self.elements {
      element.draw();
    }
  }
}
impl NetconomyCube {
  pub fn from_cube_string(string: String) -> NetconomyCube {
    let mut elements = vec![Element::first_element()];
    let mut corner_indices: Vec<usize> = vec![];
    for (i, character) in string.chars().enumerate() {
      match character {
        's' => elements.push(Element::unknown_straight()),
        'c' => {
          elements.push(Element::unknown_corner());
          corner_indices.push(i + 1);
        }
        _ => unreachable!(),
      }
    }
    elements.push(Element::last_element());
    return NetconomyCube {
      elements,
      _corner_indices: corner_indices,
      _algo_state: AlgoState { cursor: 1 },
    };
  }

  pub fn corner(&mut self, cindex: usize) -> &mut Element {
    return &mut self.elements[self._corner_indices[cindex]];
  }

  pub fn compute_positions(&mut self) {
    let mut previous_corner_direction = ivec3(0, 1, 0);
    for i in 1..self.elements.len() {
      let previous = self.elements[i - 1].clone();
      let current = self.elements[i];
      assert!(previous._position.is_some());
      assert!(previous._direction.is_some());
      match current.kind {
        ElementType::Straight => {
          self.elements[i].set(
            previous._position.unwrap() + previous._direction.unwrap(),
            previous._direction.unwrap(),
          );
        }
        ElementType::Corner => {
          let d = previous._direction.unwrap();
          let e1 = previous_corner_direction;
          let e2 = d.cross(e1);
          assert!(current.corner_orientation.is_some());
          let co = current.corner_orientation.unwrap();
          self.elements[i].set(
            previous._position.unwrap() + previous._direction.unwrap(),
            co.x * e1 + co.y * e2,
          );
          previous_corner_direction = d;
        }
      }
    }
  }

  pub fn check_overlaps(&self) -> bool {
    return !utils::has_unique_elements(self.elements.iter().map(|x| x._position.unwrap()));
  }

  pub fn bounding_cuboid(&self) -> IVec3 {
    let x_iter = self.elements.iter().map(|x| x._position.unwrap().x);
    let min_x = x_iter.clone().min().unwrap();
    let max_x = x_iter.max().unwrap();
    let y_iter = self.elements.iter().map(|x| x._position.unwrap().y);
    let min_y = y_iter.clone().min().unwrap();
    let max_y = y_iter.max().unwrap();
    let z_iter = self.elements.iter().map(|x| x._position.unwrap().z);
    let min_z = z_iter.clone().min().unwrap();
    let max_z = z_iter.max().unwrap();
    return ivec3(max_x - min_x, max_y - min_y, max_z - min_z) + ivec3(1, 1, 1);
  }

  pub fn rotate_one(&mut self) {
    for cindex in 0..self._corner_indices.len() - 1 {
      if self._algo_state.cursor % DOF_PER_CORNER.pow(cindex as u32) == 0 {
        self
          .corner(self._corner_indices.len() - cindex - 1)
          .rotate_me();
      }
    }
    self._algo_state.cursor += 1;
    self.compute_positions();
  }

  // pub fn solve(&mut self) {}
}
