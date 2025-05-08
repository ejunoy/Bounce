use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

fn generate_ngon(sides: u8, radius: f32, center: Point2, rotation: f32, elasticity: f32) -> Shape {
    let step = 2.0 * PI / (sides as f32);
    let mut vertices = Vec::new();
    for i in 0..sides {
        let angle = rotation+ step * i as f32;
        let x = angle.cos() * radius + center.x;
        let y = angle.sin() * radius + center.y;
        vertices.push(pt2(x, y));
    }
    vertices.push(vertices[0]); 
    Shape {
        vertices,
        sides,
        center,
        elasticity,
        rotation,
    }
}

fn orthogonal_vector(v: &Vec2) -> Vec2 {
    let ortho = vec2(-v.y, v.x);
    ortho.normalize()
}

fn distance_to_line(p: &Point2, a: &Point2, b: &Point2) -> f32 {
    let u = *p - *a;
    let edge = *b - *a;
    let normal = orthogonal_vector(&edge);
    u.dot(normal)
}

fn reflect_along_line(velocity: &Vec2, a: &Point2, b: &Point2) -> Vec2 {
    let edge = *b - *a;
    let normal = orthogonal_vector(&edge);
    *velocity - 2.0 * velocity.dot(normal) * normal
}

struct Ball {
    initial_position: Point2,
    initial_velocity: Vec2,
    position: Point2,
    velocity: Vec2,
    gravity: Vec2,
    elasticity: f32,
    color: Rgb<u8>,
    size: f32,
}

struct Shape {
    vertices: Vec<Point2>,
    sides: u8,
    center: Point2,
    rotation: f32,
    elasticity: f32,
}

struct Model {
    ball: Ball,
    shape: Shape,
    time: f32,
    collided: bool,
}

fn model(app: &App) -> Model {
    app.new_window().view(view).key_pressed(key_pressed).build().unwrap();

    let ball = Ball {
        initial_position: pt2(0.0, 0.0),
        initial_velocity: vec2(100.0, 100.0),
        position: pt2(0.0, 0.0),
        velocity: vec2(100.0, 100.0),
        gravity: vec2(0.0, -90.81),
        elasticity: 0.9,
        color: WHITE,
        size: 10.0,
    };

    let shape = generate_ngon(7, 200.0, pt2(0.0, 0.0), 0.0, 0.9);

    Model {
        ball,
        shape,
        time: 0.0,
        collided: false,
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    let step: f32 = 0.05;
    if key.eq(&Key::Right) {
        model.shape.rotation += step;
    } else if key.eq(&Key::Left) {
        model.shape.rotation -= step;
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.shape = generate_ngon(model.shape.sides, 200.0, pt2(0.0, 0.0), model.shape.rotation, 0.9);

    let ball = &mut model.ball;
    let shape = &model.shape;
    let dt = 1.0 / 60.0;


    model.time += dt;
    ball.position = ball.initial_position
        + ball.initial_velocity * model.time
        + 0.5 * ball.gravity * model.time * model.time;
    ball.velocity = ball.initial_velocity + ball.gravity * model.time;

    for i in 0..shape.vertices.len() - 1 {
        let a = shape.vertices[i];
        let b = shape.vertices[i + 1];
        let edge = b - a;
        let normal = orthogonal_vector(&edge);

        let distance = distance_to_line(&ball.position, &a, &b);
        let approaching = ball.velocity.dot(normal) < 0.0;

        if distance < ball.size && approaching {
            ball.velocity = reflect_along_line(&ball.velocity, &a, &b)
                * ball.elasticity
                * shape.elasticity;

            ball.position = ball.position + (ball.size - distance) * normal;

            ball.initial_position = ball.position;
            ball.initial_velocity = ball.velocity;
            model.time = 0.0;

            break;
        }
    }
}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.polyline()
        .points(model.shape.vertices.iter().copied())
        .color(WHITE);

    draw.ellipse()
        .xy(model.ball.position)
        .w_h(model.ball.size, model.ball.size)
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
