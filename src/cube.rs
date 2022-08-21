pub mod element;
pub mod utils;
use element::{Drawable, Element, ElementType};
use macroquad::prelude::{ivec3, IVec3};

pub const DOF_PER_CORNER: usize = 4;

pub struct AlgoState {
  pub cursor: usize,
}

pub struct NetconomyCube {
  elements: Vec<Element>,
  _corner_indices: Vec<usize>,
  _moduli: Vec<usize>,
  pub _bounding_cuboid: IVec3,
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
      _corner_indices: corner_indices.clone(),
      _moduli: (0..corner_indices.len())
        .map(|x| DOF_PER_CORNER.pow(x as u32))
        .collect(),
      _bounding_cuboid: ivec3(1, 1, 1),
      _algo_state: AlgoState { cursor: 1 },
    };
  }

  pub fn corner_count(&self) -> usize {
    return self._corner_indices.len();
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
    self._bounding_cuboid = self.n_bounding_cuboid(self.elements.len());
  }

  pub fn check_overlaps(&self) -> bool {
    return !utils::has_unique_elements(self.elements.iter().map(|x| x._position.unwrap()));
  }

  pub fn n_bounding_cuboid(&self, n: usize) -> IVec3 {
    let x_iter = self.elements[..n].iter().map(|x| x._position.unwrap().x);
    let min_x = x_iter.clone().min().unwrap();
    let max_x = x_iter.max().unwrap();
    let y_iter = self.elements[..n].iter().map(|x| x._position.unwrap().y);
    let min_y = y_iter.clone().min().unwrap();
    let max_y = y_iter.max().unwrap();
    let z_iter = self.elements[..n].iter().map(|x| x._position.unwrap().z);
    let min_z = z_iter.clone().min().unwrap();
    let max_z = z_iter.max().unwrap();
    return ivec3(max_x - min_x, max_y - min_y, max_z - min_z) + ivec3(1, 1, 1);
  }

  pub fn rotate_one(&mut self) {
    let corners = self._corner_indices.len();
    for cindex in 0..corners - 1 {
      if self._algo_state.cursor % self._moduli[cindex] == 0 {
        self.corner(corners - cindex - 1).rotate_me();
      }
    }
    self._algo_state.cursor += 1;
    self.compute_positions();
  }

  pub fn fold_in(&mut self) {
    let corners = self._corner_indices.len();
    for cindex in 0..corners - 1 {
      let n_cuboid = self.n_bounding_cuboid(self._corner_indices[cindex + 1] + 1);
      println!("Corner {:?}, n_cuboid: {:?}", cindex, n_cuboid);
      if n_cuboid.x > 3 || n_cuboid.y > 3 || n_cuboid.z > 3 {
        for _i in 0..4 {
          self.corner(cindex).rotate_me();
          self.compute_positions();
          if !self.check_overlaps() {
            break;
          }
        }
      }
    }
  }
  // pub fn solve(&mut self) {}
}
