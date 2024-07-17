crate::ix!();

pub struct OpenGLAppDemoShaderPreset {
    name:            *const u8,
    vertex_shader:   *const u8,
    fragment_shader: *const u8,
}

pub trait OpenGLAppDemoTextureInterface {
    fn apply_to(&mut self, _0: &mut OpenGLTexture) -> bool;
}

/**
  | These classes are used to load textures
  | from the various sources that the demo
  | uses..
  |
  */
pub struct OpenGLAppDemoTexture {
    name: String,
}

///--------------------------
pub struct OpenGLAppDemoDynamicTexture<'a> {
    base:  OpenGLAppDemoTexture,
    image: Image,
    x:     BouncingNumber,
    y:     BouncingNumber,
    _0:    PhantomData<&'a ()>,
}

impl<'a> Default for OpenGLAppDemoDynamicTexture<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            name = "Dynamically-generated texture"
        */
    }
}

impl<'a> ApplyTo for OpenGLAppDemoDynamicTexture<'a> {

    type Target = OpenGLTexture<'a>;
    type Output = bool;

    fn apply_to(&mut self, texture: &mut Self::Target) -> Self::Output {
        
        todo!();
        /*
            int size = 128;

                if (! image.isValid())
                    image = Image (Image::ARGB, size, size, true);

                {
                    Graphics g (image);
                    g.fillAll (Colours::lightcyan);

                    g.setColour (Colours::darkred);
                    g.drawRect (0, 0, size, size, 2);

                    g.setColour (Colours::green);
                    g.fillEllipse (x.getValue() * (float) size * 0.9f,
                                   y.getValue() * (float) size * 0.9f,
                                   (float) size * 0.1f,
                                   (float) size * 0.1f);

                    g.setColour (Colours::black);
                    g.setFont (40);

                    const MessageManagerLock mml (ThreadPoolJob::getCurrentThreadPoolJob());
                    if (! mml.lockWasGained())
                        return false;

                    g.drawFittedText (String (Time::getCurrentTime().getMilliseconds()), image.getBounds(), Justification::centred, 1);
                }

                texture.loadImage (image);
                return true;
        */
    }
}

