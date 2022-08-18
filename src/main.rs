use macroquad::prelude::*;

const CAMERA_DISTANCE: f32 = 8.0;

trait Drawable {
  fn draw(&self);
}

enum ElementType {
  Straight,
  Corner,
}

struct Element {
  position: Vec3,
  kind: ElementType,
}

impl Drawable for Element {
  fn draw(&self) {
    match self.kind {
      ElementType::Straight => draw_cube(self.position, vec3(1., 1., 1.), None, BLUE),
      ElementType::Corner => draw_cube(self.position, vec3(1., 1., 1.), None, GREEN),
    }
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

#[macroquad::main("Netconomy Cube Solver")]
async fn main() {
  let cube = NetconomyCube {
    elements: vec![
      Element {
        position: vec3(0., 0., 0.),
        kind: ElementType::Straight,
      },
      Element {
        position: vec3(1., 0., 0.),
        kind: ElementType::Corner,
      },
    ],
  };
  // let mut camera_phi: f32 = 0.0;
  // let mut camera_theta: f32 = 0.0;

  loop {
    if is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::W) {
      break;
    }
    let (mouse_x, mouse_y) = mouse_position();
    let camera_phi = mouse_x / 100.;
    let camera_theta = -mouse_y / 100.;

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
