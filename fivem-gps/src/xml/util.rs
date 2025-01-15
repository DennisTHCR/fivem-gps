use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Scene {
    pub objects: Objects,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@filename")]
    pub filename: String,
    #[serde(rename = "@timestamp")]
    pub timestamp: String,
    #[serde(rename = "@user")]
    pub user: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Objects {
    pub object: Vec<TopLevelObject>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TopLevelObject {
    #[serde(rename = "@guid")]
    pub guid: String,
    #[serde(rename = "@class")]
    pub class: String,
    pub transform: Transform,
    pub references: Option<References>,
    pub attributes: Option<Attributes>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Transform {
    pub object: InnerObject,
    pub node: Node,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InnerObject {
    pub position: Position,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Node {
    pub position: Position,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Position {
    #[serde(rename = "@x")]
    pub x: f32,
    #[serde(rename = "@y")]
    pub y: f32,
    #[serde(rename = "@z")]
    pub z: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct References {
    #[serde(rename = "ref")]
    pub reference: Vec<Ref>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ref {
    #[serde(rename = "@guid")]
    pub guid: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Attributes {
    pub attribute: Option<Vec<Attribute>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Attribute {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

pub fn parse_scene(xml_data: &str) -> Result<Scene, Box<dyn std::error::Error>> {
    let scene: Scene = from_str(xml_data)?;
    Ok(scene)
}
