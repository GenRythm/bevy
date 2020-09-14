
use bevy::prelude::*;

mod env;
mod camera;
mod pickup;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_plugin(env::EnvironmentPlugin)
        .add_plugin(pickup::PickingPlugin)
        // .add_plugin(pickup::DebugPickingPlugin)
        .run();
}

