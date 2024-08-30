pub mod camera;
pub mod input;
pub mod inventory;
pub mod items;
pub mod objects;

use std::collections::HashSet;

use bevy::{
    input::common_conditions::*, prelude::*, render::camera::ScalingMode, window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContexts};

use crate::assets::tiled::TiledMap;
use crate::audio::start_music;
use crate::cleanup_state;
use crate::plugins::{audio::AudioAssets, tiled::TiledMapBundle};
use crate::AppState;

/// Pause game sub-state
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, Reflect)]
#[source(AppState = AppState::InGame)]
pub enum IsPaused {
    #[default]
    Running,
    Paused,
}

/// InGame state tag
#[derive(Debug, Component)]
pub struct OnInGame;

/// Game asset container resource
#[derive(Debug, Default, Reflect, Resource)]
pub struct GameAssets {
    pub map: Handle<TiledMap>,
    pub resources_sheet: Handle<Image>,
    pub harvester_sheet: Handle<Image>,
}

impl GameAssets {
    pub fn is_loaded(&self, map_assets: &Res<Assets<TiledMap>>) -> bool {
        map_assets.contains(&self.map)
    }
}

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

/// Tracks what Object is being viewed in the ObjectInfo window
#[derive(Debug, Reflect, Resource, Deref)]
pub struct ObjectInfo(pub Entity);

// these should be less than (systems::tiled::MIN_TILEMAP_WIDTH / HEIGHT * systems::tiled::TILE_WIDTH / HEIGHT)
const VIEW_WIDTH: f32 = 800.0;
const VIEW_HEIGHT: f32 = 600.0;

#[derive(Debug, Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<IsPaused>()
            .enable_state_scoped_entities::<IsPaused>()
            .add_event::<items::ItemDragEvent>()
            .add_event::<items::ItemDropEvent>()
            .add_event::<inventory::InventoryUpdatedEvent>()
            .add_systems(OnEnter(AppState::LoadAssets), load_assets)
            .add_systems(
                Update,
                (wait_for_assets,).run_if(in_state(AppState::LoadAssets)),
            )
            .add_systems(OnEnter(AppState::InGame), enter)
            .add_systems(
                Update,
                (
                    pause_game.run_if(input_just_released(KeyCode::Escape)),
                    camera::pan,
                    input::start_drag.run_if(input_just_pressed(MouseButton::Left)),
                    input::stop_drag.run_if(input_just_released(MouseButton::Left)),
                    // TODO: instead of "just_pressed" we should check for a Drag resource existing
                    // (eg. resource_exists::<DragOperation>)
                    input::drag
                        .run_if(input_pressed(MouseButton::Left))
                        .after(input::start_drag),
                    items::item_drag_event_handler,
                    items::item_drop_event_handler,
                    objects::object_click_event_handler,
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(
                OnExit(AppState::InGame),
                (exit, cleanup_state::<OnInGame>, cleanup_state::<Node>),
            );
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        // TODO: is there a way to make this streaming?
        music: asset_server.load("music/Windless Slopes.ogg"),
    });

    commands.insert_resource(GameAssets {
        map: asset_server.load("map.tmx"),
        resources_sheet: asset_server.load("resources.png"),
        harvester_sheet: asset_server.load("harvester.png"),
    });

    info!("Waiting for assets ...");
}

fn wait_for_assets(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<AppState>>,
    game_assets: Res<GameAssets>,
    map_assets: Res<Assets<TiledMap>>,
    game_audio_assets: Res<AudioAssets>,
    audio_assets: Res<Assets<AudioSource>>,
) {
    // TODO: other "systems" can load assets that we need to wait for
    // so this whole setup needs to be reworked

    egui::Window::new("Loading").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.label("Loading assets ...");
        });
    });

    if !game_assets.is_loaded(&map_assets) || !game_audio_assets.is_loaded(&audio_assets) {
        return;
    }

    info!("Assets loaded, starting game ...");
    game_state.set(AppState::InGame);
}

fn enter(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    game_assets: Res<GameAssets>,
    mut inventory_update_events: EventWriter<inventory::InventoryUpdatedEvent>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
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

    start_music(&mut commands, audio_assets.music.clone());

    commands.insert_resource(
        //inventory::Inventory::default()
        inventory::Inventory(crate::data::inventory::InventoryData::new_test()),
    );
    inventory_update_events.send_default();

    commands.spawn((
        TiledMapBundle {
            tiled_map: game_assets.map.clone(),
            ..Default::default()
        },
        Name::new("Tiled Map"),
        OnInGame,
    ));
}

fn exit(mut commands: Commands) {
    info!("exiting InGame state");

    commands.remove_resource::<GameAssets>();
    commands.remove_resource::<AudioAssets>();
    commands.remove_resource::<ObjectInfo>();
    commands.remove_resource::<TileDrag>();
    commands.remove_resource::<inventory::Inventory>();
    commands.remove_resource::<ClearColor>();
}

fn pause_game(mut pause_state: ResMut<NextState<IsPaused>>) {
    pause_state.set(IsPaused::Paused);
}
