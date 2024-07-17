crate::ix!();

pub struct OpenGLAppDemoVertex
{
    position:  [f32; 3],
    normal:    [f32; 3],
    colour:    [f32; 4],
    tex_coord: [f32; 2],
}

/**
  | Vertex data to be passed to the shaders.
  | 
  | For the purposes of this demo, each vertex
  | will have a 3D position, a colour and
  | a 2D texture co-ordinate. Of course
  | you can ignore these or manipulate them
  | in the shader programs but are some useful
  | defaults to work from.
  |
  */
pub struct Vertex
{
    position:  [f32; 3],
    normal:    [f32; 3],
    colour:    [f32; 4],
    tex_coord: [f32; 2],
}
