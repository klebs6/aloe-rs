crate::ix!();

pub struct OpenGLAppDemoBuiltInTexture<'a> {
    base:  OpenGLAppDemoTexture,
    image: Image,
    _0:    PhantomData<&'a ()>,
}

pub trait ApplyTo {

    type Target;
    type Output;

    fn apply_to(&mut self, texture: &mut Self::Target) -> Self::Output;
}

impl<'a> ApplyTo for OpenGLAppDemoBuiltInTexture<'a> {

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

impl<'a> OpenGLAppDemoBuiltInTexture<'a> {

    pub fn new(
        nm:         *const u8,
        image_data: *const c_void,
        image_size: usize) -> Self {
    
        todo!();
        /*


            : image (resizeImageToPowerOfTwo (ImageFileFormat::loadFrom (imageData, imageSize)))

                name = nm;
        */
    }
}

