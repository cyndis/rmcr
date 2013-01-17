mod image;
mod scene;
extern mod lmath;
extern mod numeric;
extern mod std;

use io::WriterUtil;

fn main() {
  let mut scene = scene::LinearScene::new();

  let sky = scene::Object {
    shape: scene::AABB({min: lmath::vec::Vec3 { x: -10.0, y: 6.0, z: -10.0 },
                        max: lmath::vec::Vec3 { x:  10.0, y: 6.1, z:   0.0 }}),
    emits: true,
    color: scene::image::RGB { r: 1.0, g: 1.0, b: 1.0 },
    rotation: lmath::quat::Quat::identity()
  };
  scene.add(sky);

  let floor = scene::Object {
    shape: scene::AABB({min: lmath::vec::Vec3 { x: -10.0, y: -0.1, z: -10.0 },
                        max: lmath::vec::Vec3 { x:  10.0, y:  0.0, z:   0.0 }}),
    emits: false,
    color: scene::image::RGB { r: 0.6, g: 0.6, b: 0.6 },
    rotation: lmath::quat::Quat::identity()
  };
  scene.add(floor);

  let ball = scene::Object {
    shape: scene::Sphere({origin: lmath::vec::Vec3 { x: -2.0, y: 1.0, z: -4.0 },
                          radius: 1.0}),
    emits: false,
    color: scene::image::RGB { r: 0.8, g: 0.3, b: 0.3 },
    rotation: lmath::quat::Quat::identity()
  };
  scene.add(ball);

  let box = scene::Object {
    shape: scene::AABB({min: lmath::vec::Vec3 { x: 1.0, y: 0.0, z: -7.0 },
                        max: lmath::vec::Vec3 { x: 2.0, y: 1.0, z: -6.0 }}),
    emits: false,
    color: scene::image::RGB { r: 0.3, g: 0.8, b: 0.8 },
    rotation: lmath::quat::Quat::from_angle_axis(numeric::types::angle::Degrees(45.0), &lmath::vec::Vec3 { x: 0.0, y: 1.0, z: 0.0 })
  };
  scene.add(box);

  let camera = scene::Camera::new(1.57, 1.33, lmath::vec::Vec3 { x: 0.0, y: 2.0, z: 0.0 },
                                  lmath::vec::Vec3 { x: 0.0, y: 0.0, z: -5.0 });

  let mut result = scene::image::Image::new(800, 600);
  
  io::println(fmt!("Start FRAMETHREADED at %?", std::time::now().ctime()));

  let mut futures = ~[];
  let ca = std::arc::ARC(camera);
  let sa = std::arc::ARC(scene);
  for uint::range(0,150) |_| {
    let ca = ca.clone(); let sa = sa.clone();
    futures.push(do std::future::spawn |move ca, move sa| {
      scene::render_singlethread(std::arc::get(&ca), std::arc::get(&sa), 800, 600)
    });
  }

  let mut i = 0;
  for futures.each |&future| {
    future.get_ref().blend_into(&mut result, i);
    io::println(fmt!("Frame %? completed.", i+1));
    i += 1;
  }

  io::println(fmt!("End FRAMETHREADED at %?", std::time::now().ctime()));
  let wr = io::file_writer(&path::Path("framethreaded.ppm"), &[io::Create, io::Truncate]).get();
  wr.write_str(result.to_ppm());
}
