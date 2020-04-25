#![allow(dead_code)]

use nannou::prelude::*;
use nannou::app::Draw;

const AXIS_LEN: i32 = 25;
const SCALE: f32 = 20.0;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    points: Vec<Point2>,
    polynom: Polynom,
}

struct Polynom {
    parts: Vec<f32>
}

impl Polynom {
    fn constant(a: f32) -> Self {
        Polynom {
            parts: vec![a]
        }
    }

    fn mathlike(mut parts: Vec<f32>) -> Self {
        parts.reverse();
        Polynom {
            parts
        }
    }

    fn zero_at(a: f32) -> Self {
        Polynom {
            parts: vec![-a, 1.0]
        }
    }

    fn grade(&self) -> usize {
        return self.parts.len() - 1;
    }

    fn nth(&self, n: usize) -> f32 {
        self.parts.get(n).cloned().unwrap_or(0.0)
    }

    fn scaled(&self, s: f32) -> Self {
        Polynom {
            parts: self.parts.iter()
                .map(|p| s * p).collect()
        }
    }

    fn added(&self, other: Polynom) -> Self {
        let len = self.grade().max(other.grade()) + 1;
        let mut parts = Vec::with_capacity(len);
        for i in 0..len {
            parts.push(self.nth(i) + other.nth(i));
        }
        Polynom { parts }
    }

    fn extended(&self, other: Polynom) -> Self {
        let max = self.grade() + other.grade() + 1;
        let mut parts = Vec::with_capacity(max);
        for _ in 0..=max { parts.push(0.0); }
        for (i1, p1) in self.parts.iter().enumerate() {
            for (i2, p2) in other.parts.iter().enumerate() {
                let grade = i1 + i2;
                let factor = p1 * p2;
                parts[grade] += factor;
            }
        }
        Polynom { parts }
    }

    fn interpolate(points: &Vec<Point2>) -> Self {
        let mut g = Polynom::constant(points[0].y);
        let mut h = Polynom::zero_at(points[0].x);
        let mut k = 1;

        while k < points.len() {
            let a = points[k].x;
            let b = points[k].y;
            g = g.added(h.scaled((b - g.eval(a)) / h.eval(a)));
            h = h.extended(Polynom::zero_at(a));
            k += 1;
        }

        return g;
    }

    fn eval(&self, x: f32) -> f32 {
        let mut result = 0.0;
        let mut xp = 0.0;
        for f in self.parts.iter().rev() {
            result = f + xp;
            xp = x * result;
        }
        return result;
    }

    fn draw(&self, draw: &Draw) {
        let vertecies = (-AXIS_LEN..=AXIS_LEN)
            .map(|i| {
                let x = i as f32;
                pt2(x * SCALE, self.eval(x) * SCALE)
            });
        draw.polyline()
            .weight(2.0)
            .join_round()
            .color(BLACK)
            .points(vertecies);
    }
}

fn model(_app: &App) -> Model {
    Model {
        points: Vec::new(),
        polynom: Polynom::constant(0.0),
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    let event: WindowEvent = match event {
        Event::WindowEvent { simple, .. } => match simple {
            Some(event) => event,
            None => return,
        }
        _ => return,
    };

    match event {
        WindowEvent::MouseReleased(MouseButton::Left) => {
            let x = app.mouse.x / SCALE;
            let y = ( app.mouse.y - 120.0 ) / SCALE;
            let point = pt2(x, y);
            model.points.push(point);
            model.polynom = Polynom::interpolate(&model.points);
        }
        _ => (),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: &Frame){
    let win = app.window_rect();
    let draw = app.draw();

    draw.background().color(WHITE);

    for x in 0..AXIS_LEN {
        let x = x as f32 * SCALE;
        draw.line()
            .start(pt2(x, win.top()))
            .end(pt2(x, win.bottom()))
            .color(LIGHTBLUE);
        draw.line()
            .start(pt2(-x, win.top()))
            .end(pt2(-x, win.bottom()))
            .color(LIGHTBLUE);
    }
    for y in 0..AXIS_LEN {
        let y = y as f32 * SCALE;
        draw.line()
            .start(pt2(win.left(), y))
            .end(pt2(win.right(), y))
            .color(LIGHTBLUE);
        draw.line()
            .start(pt2(win.left(), -y))
            .end(pt2(win.right(), -y))
            .color(LIGHTBLUE);
    }

    model.polynom.draw(&draw);

    draw.ellipse().color(BLUE).radius(5.0).xy(pt2(0.0, 0.0));

    for point in &model.points {
        draw.ellipse().color(PURPLE).radius(5.0).xy(*point * SCALE);
    }

    draw.to_frame(app, &frame).unwrap();
}
