use torophy::{Space, RigidBody, Circle, Vec2};
use std::time::Duration;

fn main() {
    let shape1 = Circle::new(Vec2::xy(200.0, 200.0), 30.0);
    let mut body1 = RigidBody::new(shape1);
    body1.set_mass(3.0);
    body1.set_velocity(Vec2::xy(7.0, 4.0));

    let shape2 = Circle::new(Vec2::xy(200.0, 200.0), 30.0);
    let mut body2 = RigidBody::new(shape2);
    body2.set_mass(1.0);
    body2.set_velocity(Vec2::xy(-3.0, -1.0));

    let mut space = Space::new(600, 400);
    space.add(body1);
    space.add(body2);

    loop {
        space.update(Duration::from_millis(1));
    }
}

