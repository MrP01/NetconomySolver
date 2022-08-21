use macroquad::prelude::{
  clear_background, draw_text, is_key_down, is_key_pressed, ivec3, mouse_position, mouse_wheel,
  next_frame, set_camera, set_default_camera, vec3, Camera3D, KeyCode, BLACK, LIGHTGRAY, RED,
};
mod cube;
use cube::element::Drawable;

const CAMERA_DISTANCE: f32 = 8.0;

#[macroquad::main("Netconomy Cube Solver")]
async fn main() {
  let mut cube = cube::NetconomyCube::from_cube_string(String::from("scscscsccccscscccsccscccs"));
  // let mut cube = cube::NetconomyCube::from_cube_string(String::from("scscscscccc"));
  cube.compute_positions();

  let mut cam_distance = CAMERA_DISTANCE;
  let mut solved: bool = false;
  let mut solving: bool = false;
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
    if is_key_pressed(KeyCode::Key5) {
      cube.corner(4).rotate_me();
      cube.compute_positions();
    }
    if is_key_pressed(KeyCode::Key6) {
      cube.corner(5).rotate_me();
      cube.compute_positions();
    }
    if is_key_pressed(KeyCode::Key7) {
      cube.corner(6).rotate_me();
      cube.compute_positions();
    }
    if is_key_pressed(KeyCode::Key8) {
      cube.corner(7).rotate_me();
      cube.compute_positions();
    }
    if is_key_pressed(KeyCode::Key9) {
      cube.corner(8).rotate_me();
      cube.compute_positions();
    }
    if is_key_pressed(KeyCode::F) {
      cube.fold_in();
    }
    if is_key_pressed(KeyCode::Space) && !solved {
      cube.rotate_one();
    }
    if is_key_pressed(KeyCode::S) {
      solving = !solving;
    }
    if (solving || is_key_down(KeyCode::Enter)) && !solved {
      for _i in 0..999 {
        cube.rotate_one();
        if cube._bounding_cuboid == ivec3(3, 3, 3) && !cube.check_overlaps() {
          solved = true;
          println!("Solved!!");
          break;
        }
      }
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

    cube.draw();

    // Back to screen space, render some text
    set_default_camera();
    draw_text(
      format!(
        "Bounding Cuboid Size: {:?}, it: {:?} / {:?}",
        cube._bounding_cuboid,
        cube._algo_state.cursor,
        cube::DOF_PER_CORNER.pow(cube.corner_count() as u32)
      )
      .as_str(),
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
