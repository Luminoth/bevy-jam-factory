pub mod camera;
pub mod input;
pub mod objects;

use std::collections::HashSet;

use bevy::{
    input::common_conditions::*, prelude::*, render::camera::ScalingMode, window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContexts};

use crate::assets::tiled::TiledMap;
use crate::cleanup_state;
use crate::data::inventory::InventoryData;
use crate::plugins::{TiledMapBundle, TiledMapTileLayer};
use crate::AppState;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, Reflect)]
#[source(AppState = AppState::InGame)]
pub enum IsPaused {
    #[default]
    Running,
    Paused,
}

#[derive(Debug, Component)]
pub struct OnInGame;

#[derive(Debug, Default, Reflect, Resource, Deref)]
pub struct Inventory(pub InventoryData);

#[derive(Debug, Default, Reflect, Resource)]
pub struct TileDrag {
    pub tiles: HashSet<Entity>,
}

impl TileDrag {
    pub fn new(start: Entity) -> Self {
        Self {
            tiles: HashSet::from([start]),
        }
    }
}

#[derive(Debug, Reflect, Resource, Deref)]
pub struct ObjectInfo(pub Entity);

#[derive(Debug, Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<IsPaused>()
            .enable_state_scoped_entities::<IsPaused>()
            .add_systems(OnEnter(AppState::LoadAssets), load_assets)
            .add_systems(
                Update,
                (wait_for_assets,).run_if(in_state(AppState::LoadAssets)),
            )
            .add_systems(OnEnter(AppState::InGame), enter)
            .add_systems(
                Update,
                (
                    camera::pan,
                    input::start_drag.run_if(input_just_pressed(MouseButton::Left)),
                    input::stop_drag.run_if(input_just_released(MouseButton::Left)),
                    // TODO: instead of "just_pressed" we should check for a Drag resource existing
                    // (eg. resource_exists::<DragOperation>)
                    input::drag.run_if(input_pressed(MouseButton::Left)),
                    pause_game.run_if(input_just_released(KeyCode::Escape)),
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(
                OnExit(AppState::InGame),
                (exit, cleanup_state::<OnInGame>, cleanup_state::<Node>),
            );
    }
}

// these should be less than (systems::tiled::MIN_TILEMAP_WIDTH / HEIGHT * systems::tiled::TILE_WIDTH / HEIGHT)
const VIEW_WIDTH: f32 = 800.0;
const VIEW_HEIGHT: f32 = 600.0;

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("map.tmx");

    commands.spawn((
        TiledMapBundle {
            tiled_map: map_handle,
            ..Default::default()
        },
        Name::new("Tiled Map"),
        OnInGame,
    ));

    info!("Waiting for assets ...");
}

fn wait_for_assets(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<AppState>>,
    tiledmap_query: Query<&TiledMapTileLayer>,
) {
    // TODO: other "systems" can load assets that we need to wait for
    // so this whole setup needs to be reworked

    egui::Window::new("Loading").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.label("Loading assets ...");
        });
    });

    if tiledmap_query.is_empty() {
        return;
    }

    info!("Assets loaded, starting game ...");
    game_state.set(AppState::InGame);
}

fn enter(mut commands: Commands, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    info!("entering InGame state");

    commands.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: VIEW_WIDTH,
        height: VIEW_HEIGHT,
    };
    commands.spawn((
        camera_bundle,
        Name::new("Main Camera"),
        camera::MainCamera,
        camera::UiCamera,
        IsDefaultUiCamera,
        OnInGame,
    ));

    // center the cursor so the camera doesn't start panning immediately
    let mut window = window_query.single_mut();
    let center_cursor_pos = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    window.set_cursor_position(Some(center_cursor_pos));

    commands.insert_resource(Inventory::default());
}

fn exit(mut commands: Commands) {
    info!("exiting InGame state");

    commands.remove_resource::<Inventory>();
    commands.remove_resource::<ClearColor>();
}

fn pause_game(mut pause_state: ResMut<NextState<IsPaused>>) {
    pause_state.set(IsPaused::Paused);
}
