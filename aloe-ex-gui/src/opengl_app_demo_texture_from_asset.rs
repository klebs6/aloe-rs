crate::ix!();

pub struct OpenGLAppDemoTextureFromAsset<'a> {
    base:  OpenGLAppDemoTexture,
    image: Image,
    _0:    PhantomData<&'a ()>,
}

impl<'a> ApplyTo for OpenGLAppDemoTextureFromAsset<'a> {

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

impl<'a> OpenGLAppDemoTextureFromAsset<'a> {

    pub fn new(asset_name: *const u8) -> Self {
    
        todo!();
        /*


            name = assetName;
                image = resizeImageToPowerOfTwo (getImageFromAssets (assetName));
        */
    }
}
