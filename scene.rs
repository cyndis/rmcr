extern mod lmath;
extern mod std;

mod image;
mod random;

pub trait Scene {
  fn intersection_candidates(&self, ray: &Ray) -> ~[Object];
}

pub fn render_singlethread<T: Scene>(camera: &Camera, scene: &T, w: uint, h: uint) -> image::Image {
  let mut image = image::Image::new(w, h);
  let w = w as float;
  let h = h as float;
  
  for image.each_coordinate |x, y| {
    let fx = x as float / w; let fy = y as float / h;

    // antialiasing
    let fx = fx + rand::task_rng().gen_float() / w;
    let fy = fy + rand::task_rng().gen_float() / h;

    let ray = camera.ray(fx, fy);
    let color = trace_ray(scene, &ray, 0);
    image.set(x, y, color)
  }

  image
}

fn mul_color(a: &image::RGB, b: &image::RGB) -> image::RGB {
  image::RGB { r: a.r * b.r, g: a.g * b.g, b: a.b * b.b }
}

fn trace_ray<T: Scene>(scene: &T, ray: &Ray, iter: uint) -> image::RGB {
  let candidates = scene.intersection_candidates(ray);

  // boohoo we copy the object here orz
  let mut best_intersection: Option<(scene::Intersection, scene::Object, lmath::vec::Vec3<float>)> = None;
  for candidates.each |&obj| {
    let origin_objspace = ray.origin.sub_v(&obj.origin());
    let tf_origin_objspace = obj.rotation.mul_v(&origin_objspace);
    let transformed_ray = Ray {
      origin: tf_origin_objspace,
      direction: obj.rotation.mul_v(&ray.direction.neg()).neg()
    };
    match (best_intersection, obj.intersect_ray(&transformed_ray)) {
      (_, None) => (),
      (None, Some(is)) => { best_intersection = Some((is, obj, tf_origin_objspace)); },
      (Some((prev,_,_)), Some(is)) => { if is.position < prev.position { best_intersection = Some((is, obj, tf_origin_objspace)); } }
    };
  };

  match best_intersection {
    Some((intersection, obj, tf_origin_objspace)) => {
      if obj.emits {
        return obj.color;
      } else {
        let mut random_dir = random::random_vector();
        if random_dir.dot(&intersection.normal) < 0.0 {
          random_dir.neg_self();
        }

        let new_ray_origin = tf_origin_objspace.add_v(&ray.direction.mul_t(intersection.position+0.01));
        let tf_new_ray_origin = obj.origin().add_v(&obj.rotation.inverse().mul_v(&new_ray_origin));

        let new_ray = Ray { origin: tf_new_ray_origin,
                            direction: obj.rotation.inverse().mul_v(&random_dir) };
        return mul_color(&obj.color, &trace_ray(scene, &new_ray, iter+1));
      }
    },
    None => ()
  }

  image::RGB { r: 0.0, g: 0.0, b: 0.0 }
}

struct Camera {
  origin: lmath::vec::Vec3<float>,

  priv at: lmath::vec::Vec3<float>,
  priv hori: lmath::vec::Vec3<float>,
  priv vert: lmath::vec::Vec3<float>
}

impl Camera {
  static fn new(fov: float, aspect: float, origin: lmath::vec::Vec3<float>,
                lookat: lmath::vec::Vec3<float>) -> Camera
  {
    let at = lookat.sub_v(&origin).normalize();
    let up = lmath::vec::Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    let hori = at.cross(&up);
    let vert = at.cross(&hori);
    let H = float::tan(0.5 * fov);
    let hori = hori.mul_t(H);
    let vert = vert.mul_t((1.0 / aspect) * H);

    Camera { origin: origin, at: at, hori: hori, vert: vert }
  }

  fn ray(&self, x: float, y: float) -> Ray {
    Ray {
      origin: self.origin,
      direction: self.at.add_v(&self.hori.mul_t(2.0*x-1.0)).add_v(&self.vert.mul_t(2.0*y-1.0)).normalize()
    }
  }
}

pub struct LinearScene {
  objects: ~[Object]
}

impl LinearScene {
  static fn new() -> LinearScene {
    LinearScene { objects: ~[] }
  }

  fn add(&mut self, obj: Object) {
    self.objects.push(obj);
  }
}

impl LinearScene: Scene {
  fn intersection_candidates(&self, _: &Ray) -> ~[Object] {
    copy self.objects
  }
}

pub struct Ray {
  origin: lmath::vec::Vec3<float>,
  direction: lmath::vec::Vec3<float>
}

pub struct Intersection {
  position: float,
  normal: lmath::vec::Vec3<float>
}

pub enum Shape {
  Sphere({ origin: lmath::vec::Vec3<float>, radius: float }),
  AABB({ min: lmath::vec::Vec3<float>, max: lmath::vec::Vec3<float> })
}

pub struct Object {
  shape: Shape,
  emits: bool,
  color: image::RGB,
  rotation: lmath::quat::Quat<float>
}

pub impl Object {
  fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
    match self.shape {
      Sphere({origin, radius}) => {
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&ray.origin);
        let c = ray.origin.dot(&ray.origin) - radius * radius;

        let dcrim = b*b-4.0*a*c;
        if dcrim < 0.0 { return None }

        let a2inv =  1.0 / (2.0 * a);
        let t0 = (-b + float::sqrt(dcrim)) * a2inv;
        let t1 = (-b - float::sqrt(dcrim)) * a2inv;

        if t1 < 0.0 || t0 < 0.0 { return None }

        let t = [t0, t1].min();

        Some(Intersection { position: t,
                            normal: ray.origin.add_v(&ray.direction.mul_t(t)).sub_v(&origin).div_t(radius) })
      },
      AABB({min, max}) => {
        let ray_dir_inv = lmath::vec::Vec3 { x: 1.0, y: 1.0, z: 1.0 }.div_v(&ray.direction);

        let min = min.sub_v(&self.origin());
        let max = max.sub_v(&self.origin());

        let t1 = min.sub_v(&ray.origin).mul_v(&ray_dir_inv);
        let t2 = max.sub_v(&ray.origin).mul_v(&ray_dir_inv);

        let tmin = [[t1.z, t2.z].min(), [t1.y, t2.y].min(), [t1.x, t2.x].min()].max();
        let tmax = [[t1.z, t2.z].max(), [t1.y, t2.y].max(), [t1.x, t2.x].max()].min();

        if tmin < 0.0 || tmax < [0.0, tmin].max() { return None }

        let ip = ray.origin.add_v(&ray.direction.mul_t(tmin));
        let c1 = ip.sub_v(&min);
        let c2 = ip.sub_v(&max);

        let EPSILON = 0.000000001;

        let normal =
          if float::abs(c1.x) < EPSILON { lmath::vec::Vec3 { x: -1.0, y: 0.0, z: 0.0 } }
          else if float::abs(c1.y) < EPSILON { lmath::vec::Vec3 { x: 0.0, y: -1.0, z: 0.0 } }
          else if float::abs(c1.z) < EPSILON { lmath::vec::Vec3 { x: 0.0, y: 0.0, z: -1.0 } }
          else if float::abs(c2.x) < EPSILON { lmath::vec::Vec3 { x: 1.0, y: 0.0, z: 0.0 } }
          else if float::abs(c2.y) < EPSILON { lmath::vec::Vec3 { x: 0.0, y: 1.0, z: 0.0 } }
          else if float::abs(c2.z) < EPSILON { lmath::vec::Vec3 { x: 0.0, y: 0.0, z: 1.0 } }
          else { fail ~"unreachable" };

        // FIXME sometimes rays seem to be able to end up below the world

        Some(Intersection { position: tmin, normal: normal })
      }
    }
  }

  fn origin(&self) -> lmath::vec::Vec3<float> {
    match self.shape {
      Sphere({origin, _}) => origin,
      AABB({min, max}) => {
        lmath::vec::Vec3 { x: (min.x+max.x)/2.0, y: (min.y+max.y)/2.0, z: (min.z+max.z)/2.0 }
      }
    }
  }
}
