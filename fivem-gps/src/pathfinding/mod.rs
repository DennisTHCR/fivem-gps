use pathfinding::directed::astar::astar;
use std::collections::HashMap;

use bevy::math::Vec3;

use crate::xml::util::TopLevelObject;

pub struct VehicleNode {
    pub position: Vec3,
    pub guid: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Clone)]
pub struct VehicleLink {
    pub position: Vec3,
    pub guid: String,
    pub refs: Vec<String>,
    pub attributes: HashMap<String, String>,
}

pub fn guid_to_node(objects: Vec<TopLevelObject>) -> HashMap<String, VehicleNode> {
    let mut map = HashMap::new();
    for object in objects {
        if object.class == "vehiclenode" {
            let position = &object.transform.node.position;
            let mut attribute_map = HashMap::new();
            if object.attributes.is_some() {
                if object.attributes.as_ref().unwrap().attribute.is_some() {
                    object.attributes.as_ref().unwrap().attribute.as_ref().unwrap().iter().for_each(|attr| {attribute_map.insert(attr.name.clone(), attr.value.clone());});
                }
            }
            map.insert(
                object.guid.clone(),
                VehicleNode {
                    position: Vec3::new(position.x, position.y, position.z),
                    guid: object.guid.clone(),
                    attributes: attribute_map
                },
            );
        }
    }
    map
}

pub fn node_to_links(objects: Vec<TopLevelObject>) -> HashMap<String, Vec<VehicleLink>> {
    let mut links_by_ref: HashMap<String, Vec<VehicleLink>> = HashMap::new();
    let mut nodes = Vec::new();

    for object in objects {
        let mut attribute_map = HashMap::new();
            if object.attributes.is_some() {
                if object.attributes.as_ref().unwrap().attribute.is_some() {
                    object.attributes.as_ref().unwrap().attribute.as_ref().unwrap().iter().for_each(|attr| {attribute_map.insert(attr.name.clone(), attr.value.clone());});
                }
            }
        let position = &object.transform.node.position;
        if object.class == "vehiclelink" {
            let vehicle_link = VehicleLink {
                position: Vec3::new(position.x, position.y, position.z),
                guid: object.guid.clone(),
                refs: object
                    .references
                    .as_ref()
                    .unwrap()
                    .reference
                    .iter()
                    .map(|reference| reference.guid.clone())
                    .collect(),
                attributes: attribute_map,
            };

            for reference in &vehicle_link.refs {
                links_by_ref
                    .entry(reference.clone())
                    .or_insert_with(Vec::new)
                    .push(vehicle_link.clone());
            }
        } else if object.class == "vehiclenode" {
            nodes.push(VehicleNode {
                position: Vec3::new(position.x, position.y, position.z),
                guid: object.guid.clone(),
                attributes: attribute_map,
            });
        }
    }

    let mut map = HashMap::new();
    for node in nodes {
        let guid = &node.guid;
        let link_vec = links_by_ref.remove(guid).unwrap_or_default();
        map.insert(node.guid.clone(), link_vec);
    }

    map
}

fn get_next(
    guid: &String,
    links: &HashMap<String, Vec<VehicleLink>>,
    nodes: &HashMap<String, VehicleNode>,
) -> Vec<(String, i32)> {
    links
        .get(guid)
        .unwrap()
        .iter()
        .flat_map(|link| {
            link.refs.iter().map(|reference| {
                let node = nodes.get(reference).clone().unwrap();
                (
                    node.guid.clone(),
                    (node.position - nodes.get(guid).clone().unwrap().position.clone()).length()
                        as i32,
                )
            })
        })
        .collect()
}

pub fn pathfind(
    nodes: &HashMap<String, VehicleNode>,
    links: &HashMap<String, Vec<VehicleLink>>,
    goal: String,
    start: String,
) -> Option<(Vec<String>, i32)> {
    astar(
        &start,
        |guid| get_next(guid, &links, &nodes),
        |guid| {
            let pos = nodes.get(&goal).unwrap().position.clone();
            (nodes.get(guid).unwrap().position - Vec3::new(pos.x, pos.y, pos.z)).length() as i32
        },
        |guid| guid == &goal,
    )
}

pub fn get_closest_vehicle_node(
    position: Vec3,
    nodes: &HashMap<String, VehicleNode>,
) -> Option<(String, f32)> {
    nodes
        .iter()
        .map(|(guid, node)| {
            let distance = (node.position - position).length();
            (guid.clone(), distance)
        })
        .min_by(|(_, dist_a), (_, dist_b)| {
            dist_a
                .partial_cmp(dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}
