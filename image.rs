use io::WriterUtil;

pub struct RGB { r: float, g: float, b: float }

pub struct Image {
  data: ~[RGB],
  iters: uint,
  w: uint,
  h: uint
}

pub impl Image {
  static fn new(w: uint, h: uint) -> Image {
    let mut i = Image { data: ~[], iters: 0, w: w, h: h };
    i.data.grow(w*h, &RGB{r: 0.0, g: 0.0, b: 0.0});
    i
  }

  fn to_ppm(&self) -> ~str {
    do io::with_str_writer |wr| {
      wr.write_line(fmt!("P3 %? %? 255", self.w, self.h));
      for uint::range(0, self.h) |y| {
        for uint::range(0, self.w) |x| {
          let color = &self.data[y*self.w+x];
          for [color.r, color.g, color.b].each |&v| {
            wr.write_str(fmt!("%? ", (v*255.0).to_int()));
          }
        }
      }
    }
  }

  fn each_coordinate(&const self, f: &fn(x: uint, y: uint) -> bool) {
    for uint::range(0, self.w) |x| {
      for uint::range(0, self.h) |y| {
        if !f(x, y) { return; }
      }
    }
  }

  fn blend_into(&self, other: &mut Image, count: uint) {
    assert self.w == other.w && self.h == other.h;

    let count = count as float;
    for self.each_coordinate |x, y| {
      let color = &self.data[y*self.w+x];
      let old = other.data[y*self.w+x]; //waiting for INHTWAMA
      other.data[y*self.w+x] = RGB {
        r: (old.r * count + color.r) / (count + 1.0),
        g: (old.g * count + color.g) / (count + 1.0),
        b: (old.b * count + color.b) / (count + 1.0)
      };
    }
  }

  fn width(&self) -> uint { self.w }
  fn height(&self) -> uint { self.h }

  fn set(&mut self, x: uint, y: uint, c: RGB) {
    self.data[y*self.w+x] = c;
  }
}
