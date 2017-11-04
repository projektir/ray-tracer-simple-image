#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    #[serde(default = "default_color_diffuse")]
    pub color_diffuse: [u8; 3]
}

fn default_color_diffuse() -> [u8; 3] {
    [255, 255, 255]
}
