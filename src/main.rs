use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Ball {
    initial: Vec2,
    initial_velocity: Vec2,
    position: Vec2,
    velocity: Vec2,
    fuerzas: Vec<Vec2>,
    gravity: Vec2,
    elasticity : f32,
    color: Rgb<u8>,
}

struct Box {
    center: Vec2,
    width: f32,
    height: f32,
    elasticity: f32,
}


struct Model {
    ball: Ball,
    shape: Box,
    time: f32,

}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    let ball = Ball {
        initial: vec2(0.0,0.0),
        initial_velocity: vec2(20.0,0.0),
        position: vec2(0.0,0.0),
        velocity: vec2(0.0, 0.0),
        fuerzas: vec![vec2(0.0,0.0)],
        gravity: vec2(0.0, -9.81),
        elasticity: 0.9,
        color: WHITE,
    };

    let square = Box {
        center: vec2(0.0,0.0),
        width: 200.0,
        height: 200.0,
        elasticity: 1.0,
    };

    Model {
        ball,
        shape:square,
        time: 0.0,
    }

}

fn update(app: &App, model: &mut Model, update: Update) {
    model.time += 0.1;
    
    
    
    let mut ax = 0.0;
    let mut ay = 0.0;
    
    for fuerza in &model.ball.fuerzas {
        ax += fuerza.x;
        ay += fuerza.y;
    }

    ax += model.ball.gravity.x;
    ay += model.ball.gravity.y;


    if model.ball.position.x - 10.0 < model.shape.center.x - model.shape.width/2.0 {
        model.ball.initial_velocity.x = -model.ball.velocity.x*model.ball.elasticity*model.shape.elasticity;
        model.ball.initial_velocity.y = model.ball.velocity.y*model.ball.elasticity*model.shape.elasticity;
        model.ball.initial.x = model.shape.center.x - model.shape.width/2.0+10.0;
        model.ball.initial.y = model.ball.position.y;
        model.ball.color = rgb(120,12,200);
        model.time = 0.0; 
        
    }

    if model.ball.position.x + 10.0 > model.shape.center.x + model.shape.width/2.0{
        model.ball.initial_velocity.x = -model.ball.velocity.x*model.ball.elasticity*model.shape.elasticity;
        model.ball.initial_velocity.y = model.ball.velocity.y*model.ball.elasticity*model.shape.elasticity;
        model.ball.initial.x = model.shape.center.x + model.shape.width/2.0-10.0;
        model.ball.initial.y = model.ball.position.y;
        model.ball.color = rgb(120,12,200);

        model.time = 0.0;
    }

    if model.ball.position.y - 10.0 < model.shape.center.y - model.shape.height/2.0{
        model.ball.initial_velocity.y = -model.ball.velocity.y*model.ball.elasticity*model.shape.elasticity;
        model.ball.initial_velocity.x = model.ball.velocity.x*model.ball.elasticity*model.shape.elasticity;
        model.ball.initial.y = model.shape.center.y - model.shape.height/2.0+10.0;
        model.ball.initial.x = model.ball.position.x;
        model.ball.color = rgb(120,12,200);
        
        model.time = 0.0;
    }

    if model.ball.position.y +10.0 > model.shape.center.y + model.shape.height/2.0{
        model.ball.initial_velocity.y = -model.ball.velocity.y*model.ball.elasticity*model.shape.elasticity;
        model.ball.initial_velocity.x = model.ball.velocity.x*model.ball.elasticity*model.shape.elasticity;
        model.ball.initial.y = model.shape.center.y + model.shape.height/2.0-10.0;
        model.ball.initial.x = model.ball.position.x;
        model.ball.color = rgb(120,12,200);

        model.time = 0.0;
    }

    let x0 = model.ball.initial.x;
    let y0 = model.ball.initial.y;
    let vx0 = model.ball.initial_velocity.x;
    let vy0 = model.ball.initial_velocity.y;

    let x = x0 + vx0*model.time + ax*model.time*model.time/2.0;
    let y = y0 + vy0*model.time + ay*model.time*model.time/2.0;
    let vx = vx0 + ax*model.time;
    let vy = vy0 + ay*model.time;

    let pos = vec2(x,y);
    let vel = vec2(vx,vy);
    model.ball.position = pos;
    model.ball.velocity = vel;

}


fn view(app: &App, model: &Model, frame:Frame){
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.rect().x_y(model.shape.center.x,model.shape.center.y).w_h(model.shape.width, model.shape.height).color(BLACK).stroke_color(WHITE).stroke_weight(2.0);
    draw.ellipse().xy(model.ball.position).w_h(10.0,10.0).color(WHITE);
    draw.to_frame(app, &frame);
 }