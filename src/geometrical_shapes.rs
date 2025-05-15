use raster::{Color, Image};
use rand::Rng;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, img : &mut Image);
    fn color(&self) -> Color {
        let mut rng = rand::rng();
        let r = rng.random_range(1..255);
        let g = rng.random_range(1..255);
        let b = rng.random_range(1..255);
        Color::rgb(r,g,b)
    }
}

#[derive(Debug,Copy,Clone)]
pub struct Point {
    x : i32,
    y : i32
}

impl Point {
    pub fn new(x : i32,y : i32) -> Self {
        Self {x,y}
    }

    pub fn random(w : i32, h : i32) -> Self {
        let mut rng = rand::rng();

        let x = rng.random_range(0..w);
        let y = rng.random_range(0..h);

        Self::new(x,y)
    }
}

impl Drawable for Point {
    fn draw(&self, img : &mut Image) {
        let color = self.color();
        img.display(self.x,self.y, color)
    }
}

#[derive(Debug)]
pub struct Line {
    p1 : Point,
    p2 : Point
}

impl Line {
    fn new(p1 : &Point, p2 : &Point) -> Self {
        Self {p1:*p1,p2:*p2}
    }

    pub fn random(w : i32, h : i32) -> Self {
        let p1 = Point::random(w,h);
        let p2 = Point::random(w,h);

        Self::new(&p1,&p2)
    }
}

impl Drawable for Line {
    fn draw(&self, img : &mut Image) {
        let x1 = self.p1.x;
        let y1 = self.p1.y;
        let x2 = self.p2.x;
        let y2 = self.p2.y;

        let dx = x2-x1;
        let dy = y2-y1;

        let steps = i32::max(dx.abs(),dy.abs());

        let x_inc = dx as f32 /steps as f32;
        let y_inc = dy as f32 /steps as f32;

        let mut x = x1 as f32;
        let mut y = y1 as f32;

        for _ in 0..=steps {
            img.display(x.round() as i32, y.round() as i32, self.color());
            x += x_inc;
            y += y_inc;
        }
    }
}

pub struct Rectangle {
    p1 : Point,
    p2 : Point
}

impl Rectangle {
    pub fn new(p1 : &Point, p2 : &Point) -> Self {
        Self {p1 : *p1, p2 : *p2}
    }
}

impl Drawable for Rectangle {
    fn draw(&self, img : &mut Image) {
        let b = &self.p1;
        let d = &self.p2;
        let a = &Point::new(d.x, b.y);
        let c = &Point::new(b.x, d.y);

        Line::new(a,b).draw(img);
        Line::new(b,c).draw(img);
        Line::new(c,d).draw(img);
        Line::new(d,a).draw(img);
     }
}

pub struct Triangle {
    a : Point,
    b : Point,
    c : Point,
}

impl Triangle {
    pub fn new(a : &Point, b : &Point,c : &Point) -> Self {
        Self {a : *a, b : *b, c : *c}
    }
}

impl Drawable for Triangle {
    fn draw(&self, img : &mut Image) {
        Line::new(&self.a, &self.b).draw(img);
        Line::new(&self.b, &self.c).draw(img);
        Line::new(&self.c, &self.a).draw(img);
    }
}

pub struct Circle {
    center : Point,
    radius : i32
}

impl Circle {
    fn new(center : &Point, radius : i32) -> Self {
        Self {center : *center, radius}
    }

    pub fn random(w : i32, h : i32) -> Self {
        let center = &Point::random(w,h);
        let radius = rand::rng().random_range(5..w-5);

        Self::new(center,radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, image : &mut Image) {
        let color = self.color();
        let cx = self.center.x;
        let cy = self.center.y;
        let r = self.radius;

        let mut x : i32 = 0;
        let mut y = -r;

        while x < -y {
            let y_mid = y as f64 + 0.5;
            if (x.pow(2)) as f64 + y_mid.powf(2.0) > (r * r) as f64 {
                y += 1;
            }

            image.display(cx + x, cy + y, color.clone());
            image.display(cx - x, cy + y, color.clone());
            image.display(cx + x, cy - y, color.clone());
            image.display(cx - x, cy - y, color.clone());
            image.display(cx + y, cy + x, color.clone());
            image.display(cx - y, cy + x, color.clone());
            image.display(cx + y, cy - x, color.clone());
            image.display(cx - y, cy - x, color.clone());

            x += 1;
        }
    }
}