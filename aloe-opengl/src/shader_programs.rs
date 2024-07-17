crate::ix!();

pub type ShaderProgramsPtr<'a> = ReferenceCountedObjectPtr<ShaderPrograms<'a>>;

pub struct ShaderPrograms<'a> {
    base:                    ReferenceCountedObject,
    solid_colour_program:    SolidColourProgram<'a>,
    solid_colour_masked:     SolidColourMaskedProgram<'a>,
    radial_gradient:         RadialGradientProgram<'a>,
    radial_gradient_masked:  RadialGradientMaskedProgram<'a>,
    linear_gradient1:        LinearGradient1Program<'a>,
    linear_gradient_1masked: LinearGradient1MaskedProgram<'a>,
    linear_gradient2:        LinearGradient2Program<'a>,
    linear_gradient_2masked: LinearGradient2MaskedProgram<'a>,
    image:                   ImageProgram<'a>,
    image_masked:            ImageMaskedProgram<'a>,
    tiled_image:             TiledImageProgram<'a>,
    tiled_image_masked:      TiledImageMaskedProgram<'a>,
    copy_texture:            CopyTextureProgram<'a>,
    mask_texture:            MaskTextureProgram<'a>,
}

impl<'a> ShaderPrograms<'a> {
    
    pub fn new(context: &mut OpenGLContext) -> Self {
    
        todo!();
        /*
        : solid_colour_program(context),
        : solid_colour_masked(context),
        : radial_gradient(context),
        : radial_gradient_masked(context),
        : linear_gradient1(context),
        : linear_gradient_1masked(context),
        : linear_gradient2(context),
        : linear_gradient_2masked(context),
        : image(context),
        : image_masked(context),
        : tiled_image(context),
        : tiled_image_masked(context),
        : copy_texture(context),
        : mask_texture(context),

        
        */
    }
}

