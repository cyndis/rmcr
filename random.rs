extern mod lmath;

pub fn random_vector() -> lmath::vec::Vec3<float> {
  let rng = rand::task_rng();
  let theta = rng.gen_float()*2.0*3.14;
  let z = -1.0 + 2.0 * rng.gen_float();
  let t = float::sqrt(1.0-z*z);
  lmath::vec::Vec3 { x: t * float::cos(theta),
                     y: t * float::sin(theta),
                     z: z }
}
