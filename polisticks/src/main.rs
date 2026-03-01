use bevy::prelude::*;

fn main() {
    App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(Cars)
            .run();
}

#[derive(Component)]
struct Car;

#[derive(Component)]
struct Condition(bool);

#[derive(Component)]
struct Condition(String);

fn add_cars(mut commands: Commands) {
    commands.spawn((Car, Condition));
}

pub struct Cars;

impl Plugin for Cars {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, add_cars);
    }
}
