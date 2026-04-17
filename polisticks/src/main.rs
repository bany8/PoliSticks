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
struct Name(String);

#[derive(Component)]
struct Functional(bool);

#[derive(Component)]
struct Condition(String);

fn add_cars(mut commands: Commands) {
    commands.spawn((Car, Name("BMW".to_string()), Functional(True)));
    commands.spawn((Car, Name("Izera".to_string()), Functional(False)));
    commands.spawn((Car, Name("Skoda".to_string()), Functional(True)));
}

pub struct Cars;

impl Plugin for Cars {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, add_cars);
    }
}
