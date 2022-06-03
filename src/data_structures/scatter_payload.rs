use crate::data_structures::Color;
use crate::traits::Sampler;

pub struct ScatterPayload {
    pub is_specular: bool,
    pub attenuation: Color,
    pub pdf: Box<dyn Sampler>,
}
