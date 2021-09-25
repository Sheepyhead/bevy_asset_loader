use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_loading::ProgressCounter;

/// This example shows how to track the loading progress of your collections
///
/// Requires the feature 'progress_tracking'
fn main() {
    let mut app = App::build();
    app.add_plugin(bevy_loading::LoadingPlugin {
        loading_state: MyStates::AssetLoading,
        next_state: MyStates::Wrong,
    });
    AssetLoader::new(MyStates::AssetLoading, MyStates::Next)
        .with_collection::<TextureAssets>()
        .with_collection::<AudioAssets>()
        .build(&mut app);
    app.add_state(MyStates::AssetLoading)
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(MyStates::Next).with_system(quit.system()))
        .add_system_set(
            SystemSet::on_enter(MyStates::Wrong).with_system((|| print!("WTF")).system()),
        )
        .add_system_set(
            SystemSet::on_update(MyStates::AssetLoading).with_system(print_progress.system()),
        )
        .run();
}

#[derive(AssetCollection)]
struct AudioAssets {
    #[asset(path = "audio/background.ogg")]
    _background: Handle<AudioSource>,
    #[asset(path = "audio/plop.ogg")]
    _plop: Handle<AudioSource>,
}

#[derive(AssetCollection)]
struct TextureAssets {
    #[asset(path = "textures/player.png")]
    _player: Handle<Texture>,
    #[asset(path = "textures/tree.png")]
    _tree: Handle<Texture>,
    #[asset(path = "textures/female_adventurer.png")]
    _female_adventurer: Handle<Texture>,
    #[asset(path = "textures/red_square_large.png")]
    _red_square: Handle<Texture>,
    #[asset(path = "textures/green_square_large.png")]
    _green_square: Handle<Texture>,
    #[asset(path = "textures/black_square_large.png")]
    _black_square: Handle<Texture>,
}

fn quit(mut quit: EventWriter<AppExit>) {
    quit.send(AppExit);
}

fn print_progress(progress: Res<ProgressCounter>) {
    println!("Current progress: {:?}", progress.progress());
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum MyStates {
    AssetLoading,
    Next,
    Wrong,
}
