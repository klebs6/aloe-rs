crate::ix!();

pub type WavefrontObjFileIndex = u32;

pub struct WavefrontObjFileVertex { 
    x: f32,
    y: f32,
    z: f32,
}

pub struct WavefrontObjFileTextureCoord { 
    x: f32,
    y: f32,
}

pub struct WavefrontObjFileMesh
{
    vertices:       Vec<WavefrontObjFileVertex>,
    normals:        Vec<WavefrontObjFileVertex>,
    texture_coords: Vec<WavefrontObjFileTextureCoord>,
    indices:        Vec<WavefrontObjFileIndex>,
}
