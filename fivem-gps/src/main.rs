mod camera;
mod input;
mod pathfinding;
mod xml;

use bevy::prelude::*;
use camera::CameraManagementPlugin;
use input::{InputPlugin, ParsedInput};
use pathfinding::{get_closest_vehicle_node, pathfind};
use xml::{NodesParentMarker, PathfindingData, XMLAssets, XMLPlugin};

fn main() {
    App::new()
        .add_systems(Startup, load_background)
        .add_systems(Update, update_pathfinding)
        .add_plugins(DefaultPlugins)
        .add_plugins((InputPlugin, CameraManagementPlugin, XMLPlugin))
        .run();
}

#[derive(Component)]
struct MapMarker;

fn load_background(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Sprite::from_image(asset_server.load("map.png")),
        Transform::from_translation(Vec3::new(358.957, 1647.58, 0.))
            .with_scale(Vec3::new(1.23, 1.23, 1.0)),
        MapMarker,
        Name::new("Background"),
    ));
}

fn update_pathfinding(
    input: Res<ParsedInput>,
    mut pathfinding_data: ResMut<PathfindingData>,
    query: Query<(&Children, Entity), With<NodesParentMarker>>,
    mut commands: Commands,
    assets: Res<XMLAssets>,
) {
    if input.left_click {
        pathfinding_data.start_node = get_closest_vehicle_node(input.cursor_position.extend(0.), &pathfinding_data.nodes).unwrap().0;
    }
    if input.right_click {
        pathfinding_data.end_node = get_closest_vehicle_node(input.cursor_position.extend(0.), &pathfinding_data.nodes).unwrap().0;
    }
    if input.left_click || input.right_click {
        if !query.is_empty() {
            commands.entity(query.single().1).despawn_recursive();
        }
        let opt = pathfind(
            &pathfinding_data.nodes,
            &pathfinding_data.links,
            pathfinding_data.start_node.clone(),
            pathfinding_data.end_node.clone(),
        );
        if opt.is_none() {return}
        let path = opt.unwrap();
        commands
        .spawn((
            Name::from("NodesParent"),
            Transform::from_xyz(0., 0., 0.),
            Visibility::Visible,
            NodesParentMarker,
        ))
            .with_children(|parent| {
                path.0.iter().for_each(|guid| {
                    let node = pathfinding_data.nodes.get(guid).unwrap();
                    parent.spawn((
                        if node.attributes.get("Speed").is_none() {
                            assets.yellow.clone()
                        } else {
                            let speed = node.attributes.get("Speed").unwrap();
                            if speed == "0" {
                                assets.white.clone()
                            } else if speed == "2" {
                                assets.orange.clone()
                            } else {
                                assets.red.clone()
                            }
                        },
                        assets.circle.clone(),
                        Transform::from_translation(node.position)
                            .with_scale(Vec3::splat(5.)),
                    ));
                });
            });
    }
}
