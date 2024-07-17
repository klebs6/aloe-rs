crate::ix!();

pub struct WavefrontObjFileMaterial {
    name:                  String,
    ambient:               WavefrontObjFileVertex,
    diffuse:               WavefrontObjFileVertex,
    specular:              WavefrontObjFileVertex,
    transmittance:         WavefrontObjFileVertex,
    emission:              WavefrontObjFileVertex,
    shininess:             f32, // default = 1.0f
    refractive_index:      f32, // default = 0.0f
    ambient_texture_name:  String,
    diffuse_texture_name:  String,
    specular_texture_name: String,
    normal_texture_name:   String,
    parameters:            StringPairArray,
}

impl Default for WavefrontObjFileMaterial {
    
    fn default() -> Self {
        todo!();
        /*
            zerostruct (ambient);
                zerostruct (diffuse);
                zerostruct (specular);
                zerostruct (transmittance);
                zerostruct (emission)
        */
    }
}


