mod image;
mod scene;
extern mod lmath;
extern mod numeric;
extern mod std;

use io::WriterUtil;

fn main() {
  let mut scene = scene::LinearScene::new();
/*
  let sphere = scene::Object {
    shape: scene::Sphere({origin: lmath::vec::Vec3 { x: 0.0, y: 0.0, z: -5.0 }, radius: 1.0}),
    emits: false,
    color: scene::image::RGB { r: 0.8, g: 0.2, b: 0.2 },
    rotation: lmath::quat::Quat::identity()
  };
  scene.add(sphere);

  let light = scene::Object {
    shape: scene::Sphere({origin: lmath::vec::Vec3 { x: 2.0, y: 0.0, z: -3.0 }, radius: 0.6}),
    emits: true,
    color: scene::image::RGB { r: 1.0, g: 1.0, b: 1.0 },
    rotation:lmath::quat::Quat::identity()
  };
  scene.add(light);
*/
  let plane = scene::Object {
    shape: scene::AABB({min: lmath::vec::Vec3 { x: -5.0, y: -5.0, z: 1.0 },
                        max: lmath::vec::Vec3 { x:  5.0, y:  5.0, z: 1.0 }}),
    emits: true,
    color: scene::image::RGB { r: 1.0, g: 1.0, b: 1.0 },
    rotation: lmath::quat::Quat::identity()
  };
  scene.add(plane);

  let rad = numeric::types::angle::Radians(3.14/4.0);
  let quat = lmath::quat::Quat::from_angle_axis(rad, &lmath::vec::Vec3 { x: 0.3, y: 0.3, z: 0.0 });
  
  let box = scene::Object {
    shape: scene::AABB({min: lmath::vec::Vec3 { x: -0.5, y: -0.5, z: -4.0 },
                        max: lmath::vec::Vec3 { x:  0.5, y:  0.5, z: -3.5 }}),
    emits: false,
    color: scene::image::RGB { r: 0.0, g: 0.2, b: 1.0 },
    rotation: quat
  };
  scene.add(box);

  let camera = scene::Camera::new(1.57, 1.33, lmath::vec::Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                                  lmath::vec::Vec3 { x: 0.0, y: 0.0, z: -1.0 });

  let mut result = scene::image::Image::new(800, 600);
  
  io::println(fmt!("Start FRAMETHREADED at %?", std::time::now().ctime()));

  let mut futures = ~[];
  let ca = std::arc::ARC(camera);
  let sa = std::arc::ARC(scene);
  for uint::range(0,30) |_| {
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
