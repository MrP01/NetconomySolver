use macroquad::prelude::*;

const CAMERA_DISTANCE: f32 = 8.0;

trait Drawable {
  fn draw(&self);
}

#[derive(Debug, Clone, Copy)]
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
}
impl Drawable for NetconomyCube {
  fn draw(&self) {
    for element in &self.elements {
      element.draw();
    }
  }
}
impl NetconomyCube {
  fn compute_positions(&mut self) {
    for i in 1..self.elements.len() {
      let previous = self.elements[i - 1].clone();
      println!("{:?}", previous);
      assert!(previous._position.is_some());
      assert!(previous._direction.is_some());
      match previous.kind {
        ElementType::Straight => {
          self.elements[i].set(
            previous._position.unwrap() + previous._direction.unwrap(),
            previous._direction.unwrap(),
          );
        }
        ElementType::Corner => {
          let d = previous._direction.unwrap();
          let e1 = vec3(d.z, d.y, -d.x);
          let e2 = vec3(d.x, d.z, -d.y);
          assert!(previous.corner_orientation.is_some());
          let co = previous.corner_orientation.unwrap();
          self.elements[i].set(
            previous._position.unwrap() + previous._direction.unwrap(),
            co.x * e1 + co.y * e2,
          );
        }
      }
    }
  }
}

#[macroquad::main("Netconomy Cube Solver")]
async fn main() {
  let cube_string = String::from("scscscsccccscscccsccscccs");
  let mut elements = vec![Element::first_element()];
  for character in cube_string.chars() {
    match character {
      's' => elements.push(Element::unknown_straight()),
      'c' => elements.push(Element::unknown_corner()),
      _ => unreachable!(),
    }
  }
  elements.push(Element::last_element());
  let mut cube = NetconomyCube { elements };
  cube.compute_positions();

  loop {
    if is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::W) {
      break;
    }
    let (mouse_x, mouse_y) = mouse_position();
    let camera_phi = mouse_x / 100.;
    let camera_theta = -mouse_y / 200.;

    clear_background(LIGHTGRAY);

    // Going 3d!
    set_camera(&Camera3D {
      position: vec3(
        camera_theta.sin() * camera_phi.cos() * CAMERA_DISTANCE,
        camera_theta.sin() * camera_phi.sin() * CAMERA_DISTANCE,
        camera_theta.cos() * CAMERA_DISTANCE,
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
