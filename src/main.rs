use macroquad::prelude::*;

const CAMERA_DISTANCE: f32 = 8.0;

trait Drawable {
  fn draw(&self);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ElementType {
  Straight,
  Corner,
}

#[derive(Debug, Clone, Copy)]
struct Element {
  kind: ElementType,
  corner_orientation: Option<Vec2>,
  _position: Option<Vec3>,
  _direction: Option<Vec3>,
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
  fn set(&mut self, pos: Vec3, dir: Vec3) {
    self._position = Some(pos);
    self._direction = Some(dir);
  }

  fn rotate_me(&mut self) {
    assert!(self.kind == ElementType::Corner);
    self.corner_orientation = Some(vec2(
      self.corner_orientation.unwrap().y,
      -self.corner_orientation.unwrap().x,
    ));
    println!("I was rotated.");
  }

  fn unknown_straight() -> Element {
    return Element {
      kind: ElementType::Straight,
      corner_orientation: None,
      _position: None,
      _direction: None,
    };
  }
  fn unknown_corner() -> Element {
    return Element {
      kind: ElementType::Corner,
      corner_orientation: Some(vec2(1., 0.)),
      _position: None,
      _direction: None,
    };
  }
  fn first_element() -> Element {
    return Element {
      kind: ElementType::Straight,
      corner_orientation: None,
      _position: Some(vec3(0., 0., 0.)),
      _direction: Some(vec3(1., 0., 0.)),
    };
  }
  fn last_element() -> Element {
    return Element {
      kind: ElementType::Straight,
      corner_orientation: None,
      _position: None,
      _direction: None,
    };
  }
}

struct NetconomyCube {
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
  fn from_cube_string(string: String) -> NetconomyCube {
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

  fn corner(&mut self, cindex: usize) -> &mut Element {
    return &mut self.elements[self.corner_indices[cindex]];
  }

  fn compute_positions(&mut self) {
    let mut previous_corner_direction = vec3(0., 1., 0.);
    for i in 1..self.elements.len() {
      let previous = self.elements[i - 1].clone();
      let current = self.elements[i];
      println!("{:?}", previous);
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
          // let e1 = vec3(d.z, d.y, -d.x);
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
}

#[macroquad::main("Netconomy Cube Solver")]
async fn main() {
  let mut cube = NetconomyCube::from_cube_string(String::from("scscscsccccscscccsccscccs"));
  cube.compute_positions();

  let mut cam_distance = CAMERA_DISTANCE;
  loop {
    if is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::W) {
      break;
    }
    if is_key_pressed(KeyCode::Key1) {
      cube.corner(0).rotate_me();
      cube.compute_positions();
    }
    if is_key_pressed(KeyCode::Key2) {
      cube.corner(1).rotate_me();
      cube.compute_positions();
    }
    if is_key_pressed(KeyCode::Key3) {
      cube.corner(2).rotate_me();
      cube.compute_positions();
    }
    if is_key_pressed(KeyCode::Key4) {
      cube.corner(3).rotate_me();
      cube.compute_positions();
    }
    let (mouse_x, mouse_y) = mouse_position();
    let camera_phi = mouse_x / 100.;
    let camera_theta = -mouse_y / 200.;
    cam_distance += mouse_wheel().1;

    clear_background(LIGHTGRAY);

    // Going 3d!
    set_camera(&Camera3D {
      position: vec3(
        camera_theta.sin() * camera_phi.cos() * cam_distance,
        camera_theta.sin() * camera_phi.sin() * cam_distance,
        camera_theta.cos() * cam_distance,
      ),
      up: vec3(0., 0., 1.),
      target: vec3(0., 0., 0.),
      ..Default::default()
    });

    // draw_grid(20, 1., BLACK, GRAY);
    // draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);
    cube.draw();

    // Back to screen space, render some text
    set_default_camera();
    draw_text("Solve the cube:", 10.0, 20.0, 30.0, BLACK);

    next_frame().await
  }
}
