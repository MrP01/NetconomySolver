use macroquad::prelude::*;
mod cube;
use cube::element::Drawable;

const CAMERA_DISTANCE: f32 = 8.0;

#[macroquad::main("Netconomy Cube Solver")]
async fn main() {
  let mut cube = cube::NetconomyCube::from_cube_string(String::from("scscscsccccscscccsccscccs"));
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
    cam_distance -= mouse_wheel().1;

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
    draw_text(
      format!("Bounding Cuboid Size: {:?}", cube.bounding_cuboid()).as_str(),
      10.0,
      20.0,
      30.0,
      BLACK,
    );
    if cube.check_overlaps() {
      draw_text("Has overlap!", 10.0, 50.0, 30.0, RED);
    }

    next_frame().await
  }
}
