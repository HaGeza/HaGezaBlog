use bevy::{
    DefaultPlugins,
    app::{Plugin, Startup},
    ecs::{
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    prelude::{App, Component, Update},
    time::{Time, Timer, TimerMode},
};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Jacky Chen".to_string())));
    commands.spawn((Person, Name("Zdeve Hobs".to_string())));
    commands.spawn((Person, Name("Fernando Rodriguez Salamanca Vespucci".to_string())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Zdeve Hobs" {
            name.0 = "Djevry Abszdeen".to_string();
            break;
        }
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people).chain());
    }
}

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugins(HelloPlugin).run();
}
