#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    #[serde(default = "default_color_diffuse")]
    pub color_diffuse: [u8; 3],
}

impl Default for Material {
    fn default() -> Material {
        Material {
            color_diffuse: default_color_diffuse(),
        }
    }
}

fn default_color_diffuse() -> [u8; 3] {
    [255, 255, 255]
}
