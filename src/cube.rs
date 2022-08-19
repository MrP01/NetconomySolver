pub mod element;
pub mod utils;
use element::{Drawable, Element, ElementType};
use macroquad::prelude::vec3;

pub struct NetconomyCube {
  elements: Vec<Element>,
  corner_indices: Vec<usize>,
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
      corner_indices,
    };
  }

  pub fn corner(&mut self, cindex: usize) -> &mut Element {
    return &mut self.elements[self.corner_indices[cindex]];
  }

  pub fn compute_positions(&mut self) {
    let mut previous_corner_direction = vec3(0., 1., 0.);
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
    return !utils::has_unique_elements(
      self.elements.iter().map(|x| x._position.unwrap().as_i32()),
    );
  }
}
