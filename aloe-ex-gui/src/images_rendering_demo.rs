crate::ix!();

pub struct ImagesRenderingDemo<'a> {
    base:       GraphicsDemoBase<'a>,
    is_argb:    bool,
    is_tiled:   bool,
    rgb_image:  Image,
    argb_image: Image,
}

impl<'a> ImagesRenderingDemo<'a> {

    pub fn new(
        cc:    &mut ControllersComponent,
        argb:  bool,
        tiled: bool) -> Self {
    
        todo!();
        /*


            : GraphicsDemoBase (cc, String ("Images") + (argb ? ": ARGB" : ": RGB") + (tiled ? " Tiled" : String() )),
              isArgb (argb), isTiled (tiled)

            argbImage = getImageFromAssets ("aloe_icon.png");
            rgbImage  = getImageFromAssets ("portmeirion.jpg");
        */
    }
    
    pub fn draw_demo(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto image = isArgb ? argbImage : rgbImage;

            AffineTransform transform (AffineTransform::translation ((float) (image.getWidth()  / -2),
                                                                     (float) (image.getHeight() / -2))
                                       .followedBy (getTransform()));

            if (isTiled)
            {
                FillType fill (image, transform);
                fill.setOpacity (getAlpha());
                g.setFillType (fill);
                g.fillAll();
            }
            else
            {
                g.setOpacity (getAlpha());
                g.drawImageTransformed (image, transform, false);
            }
        */
    }
}
