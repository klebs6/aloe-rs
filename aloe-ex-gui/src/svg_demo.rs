crate::ix!();

pub struct SVGDemo<'a> {
    base: GraphicsDemoBase<'a>,
    last_svg_load_time: Time,
    svg_drawable:       Box<Drawable<'a>>,
}

impl<'a> SVGDemo<'a> {

    pub fn new(cc: &mut ControllersComponent) -> Self {
    
        todo!();
        /*


            : GraphicsDemoBase (cc, "SVG")

            createSVGDrawable();
        */
    }
    
    pub fn draw_demo(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (Time::getCurrentTime().toMilliseconds() > lastSVGLoadTime.toMilliseconds() + 2000)
                createSVGDrawable();

            svgDrawable->draw (g, getAlpha(), getTransform());
        */
    }
    
    pub fn create_svg_drawable(&mut self)  {
        
        todo!();
        /*
            lastSVGLoadTime = Time::getCurrentTime();

            ZipFile icons (createAssetInputStream ("icons.zip").release(), true);

            // Load a random SVG file from our embedded icons.zip file.
            const std::unique_ptr<InputStream> svgFileStream (icons.createStreamForEntry (Random::getSystemRandom().nextInt (icons.getNumEntries())));

            if (svgFileStream.get() != nullptr)
            {
                svgDrawable = Drawable::createFromImageDataStream (*svgFileStream);

                if (svgDrawable != nullptr)
                {
                    // to make our icon the right size, we'll set its bounding box to the size and position that we want.

                    if (auto comp = dynamic_cast<DrawableComposite*> (svgDrawable.get()))
                        comp->setBoundingBox ({ -100.0f, -100.0f, 200.0f, 200.0f });
                }
            }
        */
    }
}
