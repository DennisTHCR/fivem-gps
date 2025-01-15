pub mod util;

use std::io::Write;

use std::{collections::HashMap, fs::{self, File}};

use bevy::prelude::*;
use util::parse_scene;

use crate::pathfinding::{guid_to_node, node_to_links, pathfind, VehicleLink, VehicleNode};

#[derive(Resource)]
pub struct XMLAssets {
    pub circle: Mesh2d,
    pub blue: MeshMaterial2d<ColorMaterial>,
    pub red: MeshMaterial2d<ColorMaterial>,
    pub yellow: MeshMaterial2d<ColorMaterial>,
    pub white: MeshMaterial2d<ColorMaterial>,
    pub orange: MeshMaterial2d<ColorMaterial>,
}

pub struct XMLPlugin;

impl Plugin for XMLPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_assets, read).chain());
    }
}

fn init_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(XMLAssets {
        circle: Mesh2d(meshes.add(Circle::new(5.))),
        blue: MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 0.0, 1.0))),
        red: MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
        yellow: MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 1.0, 0.0))),
        white: MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 1.0, 1.0))),
        orange: MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.5, 0.0))),
    });
}

/* fn read(mut commands: Commands, assets: Res<XMLAssets>) {
    let mut buffer = Vec::new();
    let mut reader = Reader::from_file("paths.xml").unwrap();
    let mut depth = 0;
    let mut current = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Err(e) => panic!("{:?}", e),
            Ok(Event::Eof) => {break}
            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"object" {
                    if depth == 0 {
                        e.attributes().for_each(|atr| {
                            let attribute = atr.unwrap();
                            if attribute.key.as_ref() == b"class" {
                                current = attribute.value.to_vec();
                            }
                        });
                    }
                    depth += 1;
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"object" {
                    depth -= 1;
                }
            }
            Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"position" {
                    let mut x = default();
                    let mut y = default();
                    let mut z = default();
                    e.attributes().for_each(|atr| {
                        let attribute = atr.unwrap();
                        match attribute.key.as_ref() {
                            b"x" => {
                                x = String::from_utf8(
                                    attribute.value.iter().map(|&n| n).collect::<Vec<_>>(),
                                )
                                .unwrap()
                                .parse::<f32>()
                                .unwrap();
                            }
                            b"y" => {
                                y = String::from_utf8(
                                    attribute.value.iter().map(|&n| n).collect::<Vec<_>>(),
                                )
                                .unwrap()
                                .parse::<f32>()
                                .unwrap();
                            }
                            b"z" => {
                                z = String::from_utf8(
                                    attribute.value.iter().map(|&n| n).collect::<Vec<_>>(),
                                )
                                .unwrap()
                                .parse::<f32>()
                                .unwrap();
                            }
                            _ => (),
                        }
                    });
                    commands.spawn((
                        if current == b"vehiclenode".to_vec() {
                        assets.blue.clone()
                    } else {
                        assets.red.clone()
                    },
                        assets.circle.clone(),
                        Transform::from_xyz(x, y, z),
                    ));
                }
            }
            _ => {}
        };
    }
}
*/

#[derive(Component)]
pub struct NodesParentMarker;

#[derive(Resource)]
pub struct PathfindingData {
    pub nodes: HashMap<String, VehicleNode>,
    pub links: HashMap<String, Vec<VehicleLink>>,
    pub start_node: String,
    pub end_node: String,
}

pub fn serialize_to_lua(
    nodes: &HashMap<String, VehicleNode>,
    links: &HashMap<String, Vec<VehicleLink>>,
) -> std::io::Result<()> {
    let mut file1 = File::create("nodes.lua")?;

    writeln!(file1, "nodes = {{")?;
    for (guid, node) in nodes {
        let attributes = node
            .attributes
            .iter()
            .map(|(key, value)| format!("[{:?}] = {:?}", key, value))
            .collect::<Vec<String>>()
            .join(", ");
        writeln!(
            file1,
            "    [{:?}] = {{ position = {{ x = {}, y = {}, z = {} }}, attributes = {{ {} }} }},",
            guid, node.position.x, node.position.y, node.position.z, attributes
        )?;
    }
    writeln!(file1, "}}")?;

    let mut file2 = File::create("links.lua")?;
    // Write the links
    writeln!(file2, "links = {{")?;
    for (guid, link_vec) in links {
        writeln!(file2, "    [{:?}] = {{", guid)?;
        for link in link_vec {
            let attributes = link
                .attributes
                .iter()
                .map(|(key, value)| format!("[{:?}] = {:?}", key, value))
                .collect::<Vec<String>>()
                .join(", ");
            let refs = link
                .refs
                .iter()
                .map(|r| format!("{:?}", r))
                .collect::<Vec<String>>()
                .join(", ");
            writeln!(
                file2,
                "        {{ position = {{ x = {}, y = {}, z = {} }}, refs = {{ {} }}, attributes = {{ {} }} }},",
                link.position.x, link.position.y, link.position.z, refs, attributes
            )?;
        }
        writeln!(file2, "    }},")?;
    }
    writeln!(file2, "}}")?;

    let mut file3 = File::create("guids.lua")?;
    writeln!(file3, "guids = {{")?;
    nodes.iter().for_each(|node| {
        let pos = node.1.position;
        let _ = writeln!(file3, "[\"{} {} {}\"] = {:?},", pos.x, pos.y, pos.z, node.0);
    });
    writeln!(file3, "}}")?;

    Ok(())
}

fn read(mut commands: Commands, assets: Res<XMLAssets>) {
    let data = fs::read_to_string("paths.xml").unwrap();
    let scene = parse_scene(data.as_str()).unwrap();
    let objects = scene.clone().objects.object;
    let nodes = guid_to_node(objects.clone());
    let links = node_to_links(objects.clone());
    // let _ = serialize_to_lua(&nodes, &links);
    let path = pathfind(
        &nodes,
        &links,
        objects[0].guid.clone(),
        objects[1001].guid.clone(),
    )
    .unwrap();
    println!("{} {}", String::from("6EBB61B9-2B35-4D22-B9E2-48D9FB0A3BDD"), String::from("32EB89C5-3597-441B-9E54-28341024C1C2"));
    commands
        .spawn((
            Name::from("NodesParent"),
            Transform::from_xyz(0., 0., 0.),
            Visibility::Visible,
            NodesParentMarker,
        ))
        .with_children(|parent| {
            path.0.iter().for_each(|guid| {
                let node = nodes.get(guid).unwrap();
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
    commands.insert_resource(PathfindingData {
        nodes,
        links,
        start_node: String::from("6EBB61B9-2B35-4D22-B9E2-48D9FB0A3BDD"),
        end_node: String::from("32EB89C5-3597-441B-9E54-28341024C1C2"),
    });
}
