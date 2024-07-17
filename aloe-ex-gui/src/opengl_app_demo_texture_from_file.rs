crate::ix!();

pub struct OpenGLAppDemoTextureFromFile<'a> {
    base:  OpenGLAppDemoTexture,
    image: Image,
    _0:    PhantomData<&'a ()>,
}

impl<'a> ApplyTo for OpenGLAppDemoTextureFromFile<'a> {

    type Target = OpenGLTexture<'a>;
    type Output = bool;

    fn apply_to(&mut self, texture: &mut Self::Target) -> Self::Output {
        
        todo!();
        /*
            texture.loadImage (image);
                return false;
        */
    }
}

impl<'a> OpenGLAppDemoTextureFromFile<'a> {

    pub fn new(file: &File) -> Self {
    
        todo!();
        /*


            name = file.getFileName();
                image = resizeImageToPowerOfTwo (ImageFileFormat::loadFrom (file));
        */
    }
}
