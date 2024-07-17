crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_Drawable.h]

/**
  | The base class for objects which can
  | draw themselves, e.g. polygons, images,
  | etc.
  | 
  | @see DrawableComposite, DrawableImage,
  | DrawablePath, DrawableText
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct Drawable<'a> {
    base:                         Component<'a>,
    origin_relative_to_component: Point<i32>,
    drawable_clip_path:           Box<Drawable<'a>>,
}

impl<'a> Default for Drawable<'a> {

    /**
      | The base class can't be instantiated
      | directly.
      | 
      | @see DrawableComposite, DrawableImage,
      | DrawablePath, DrawableText
      |
      */
    fn default() -> Self {
    
        todo!();
        /*


            setInterceptsMouseClicks (false, false);
        setPaintingIsUnclipped (true);
        setAccessible (false);
        */
    }
    
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_Drawable.cpp]
impl<'a> Drawable<'a> {
    
    /**
      | Attempts to parse an SVG (Scalable Vector
      | Graphics) document, and to turn this
      | into a Drawable tree.
      | 
      | If something goes wrong while parsing,
      | it may return nullptr.
      | 
      | SVG is a pretty large and complex spec,
      | and this doesn't aim to be a full implementation,
      | but it can return the basic vector objects.
      |
      */
    pub fn create_fromsvg(svg_document: &XmlElement) -> Box<Drawable> {
        
        todo!();
        /*
        
        */
    }

    /**
      | Attempts to parse an SVG (Scalable Vector
      | Graphics) document from a file, and
      | to turn this into a Drawable tree.
      | 
      | If something goes wrong while parsing,
      | it may return nullptr.
      | 
      | SVG is a pretty large and complex spec,
      | and this doesn't aim to be a full implementation,
      | but it can return the basic vector objects.
      | 
      | Any references to references to external
      | image files will be relative to the parent
      | directory of the file passed.
      |
      */
    pub fn create_from_svg_file(svg_file: &File) -> Box<Drawable> {
        
        todo!();
        /*
        
        */
    }

    /**
      | Parses an SVG path string and returns
      | it.
      |
      */
    pub fn parse_svg_path(svg_path: &String) -> PathBuf {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new(other: &Drawable) -> Self {
    
        todo!();
        /*
        : component(other.getName()),

            setInterceptsMouseClicks (false, false);
        setPaintingIsUnclipped (true);
        setAccessible (false);

        setComponentID (other.getComponentID());
        setTransform (other.getTransform());

        if (auto* clipPath = other.drawableClipPath.get())
            setClipPath (clipPath->createCopy());
        */
    }
    
    pub fn apply_drawable_clip_path(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (drawableClipPath != nullptr)
        {
            auto clipPath = drawableClipPath->getOutlineAsPath();

            if (! clipPath.isEmpty())
                g.getInternalContext().clipToPath (clipPath, {});
        }
        */
    }
    
    /**
      | Renders this Drawable object.
      | 
      | -----------
      | @note
      | 
      | the preferred way to render a drawable
      | in future is by using it as a component
      | and adding it to a parent, so you might
      | want to consider that before using this
      | method.
      | 
      | @see drawWithin
      |
      */
    pub fn draw(
        &self, 
        g:         &mut Graphics,
        opacity:   f32,
        transform: Option<&AffineTransform>

    ) {

        let transform: &AffineTransform = transform.unwrap_or(&AffineTransform::default());
        
        todo!();
        /*
            const_cast<Drawable*> (this)->nonConstDraw (g, opacity, transform);
        */
    }
    
    pub fn non_const_draw(
        &mut self, 
        g:         &mut Graphics,
        opacity:   f32,
        transform: &AffineTransform

    ) {
        
        todo!();
        /*
            Graphics::ScopedSaveState ss (g);

        g.addTransform (AffineTransform::translation ((float) -(originRelativeToComponent.x),
                                                      (float) -(originRelativeToComponent.y))
                            .followedBy (getTransform())
                            .followedBy (transform));

        applyDrawableClipPath (g);

        if (! g.isClipEmpty())
        {
            if (opacity < 1.0f)
            {
                g.beginTransparencyLayer (opacity);
                paintEntireComponent (g, true);
                g.endTransparencyLayer();
            }
            else
            {
                paintEntireComponent (g, true);
            }
        }
        */
    }
    
    /**
      | Renders the Drawable at a given offset
      | within the Graphics context.
      | 
      | The coordinates passed-in are used
      | to translate the object relative to
      | its own origin before drawing it - this
      | is basically a quick way of saying:
      | 
      | -----------
      | @note
      | 
      | the preferred way to render a drawable
      | in future is by using it as a component
      | and adding it to a parent, so you might
      | want to consider that before using this
      | method.
      | 
      | -----------
      | @code
      | 
      | draw (g, AffineTransform::translation (x, y)).
      |
      */
    pub fn draw_at(&self, 
        g:       &mut Graphics,
        x:       f32,
        y:       f32,
        opacity: f32)  {
        
        todo!();
        /*
            draw (g, opacity, AffineTransform::translation (x, y));
        */
    }
    
    /**
      | Renders the Drawable within a rectangle,
      | scaling it to fit neatly inside without
      | changing its aspect-ratio.
      | 
      | The object can placed arbitrarily within
      | the rectangle based on a Justification
      | type, and can either be made as big as
      | possible, or just reduced to fit.
      | 
      | -----------
      | @note
      | 
      | the preferred way to render a drawable
      | in future is by using it as a component
      | and adding it to a parent, so you might
      | want to consider that before using this
      | method.
      | 
      | -----------
      | @param g
      | 
      | the graphics context to render onto
      | ----------
      | @param destArea
      | 
      | the target rectangle to fit the drawable
      | into
      | ----------
      | @param placement
      | 
      | defines the alignment and rescaling
      | to use to fit this object within the target
      | rectangle.
      | ----------
      | @param opacity
      | 
      | the opacity to use, in the range 0 to 1.0
      |
      */
    pub fn draw_within(&self, 
        g:         &mut Graphics,
        dest_area: Rectangle<f32>,
        placement: RectanglePlacement,
        opacity:   f32)  {
        
        todo!();
        /*
            draw (g, opacity, placement.getTransformToFit (getDrawableBounds(), destArea));
        */
    }
    
    /**
      | Returns the DrawableComposite that
      | contains this object, if there is one.
      |
      */
    pub fn get_parent(&self) -> *mut DrawableComposite {
        
        todo!();
        /*
            return dynamic_cast<DrawableComposite*> (getParentComponent());
        */
    }
    
    /**
      | Sets a the clipping region of this drawable
      | using another drawable.
      | 
      | The drawable passed in will be deleted
      | when no longer needed.
      |
      */
    pub fn set_clip_path(&mut self, clip_path: Box<Drawable>)  {
        
        todo!();
        /*
            if (drawableClipPath != clipPath)
        {
            drawableClipPath = std::move (clipPath);
            repaint();
        }
        */
    }
    
    pub fn transform_context_to_correct_origin(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setOrigin (originRelativeToComponent);
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            setBoundsToEnclose (getDrawableBounds());
        */
    }
    
    pub fn set_bounds_to_enclose(&mut self, area: Rectangle<f32>)  {
        
        todo!();
        /*
            Point<int> parentOrigin;

        if (auto* parent = getParent())
            parentOrigin = parent->originRelativeToComponent;

        auto newBounds = area.getSmallestIntegerContainer() + parentOrigin;
        originRelativeToComponent = parentOrigin - newBounds.getPosition();
        setBounds (newBounds);
        */
    }
    
    pub fn replace_colour(&mut self, 
        original:    Colour,
        replacement: Colour) -> bool {
        
        todo!();
        /*
            bool changed = false;

        for (auto* c : getChildren())
            if (auto* d = dynamic_cast<Drawable*> (c))
                changed = d->replaceColour (original, replacement) || changed;

        return changed;
        */
    }
    
    /**
      | Resets any transformations on this
      | drawable, and positions its origin
      | within its parent component.
      |
      */
    pub fn set_origin_with_original_size(&mut self, origin_within_parent: Point<f32>)  {
        
        todo!();
        /*
            setTransform (AffineTransform::translation (originWithinParent.x, originWithinParent.y));
        */
    }
    
    /**
      | Sets a transform for this drawable that
      | will position it within the specified
      | area of its parent component.
      |
      */
    pub fn set_transform_to_fit(&mut self, 
        area:      &Rectangle<f32>,
        placement: RectanglePlacement)  {
        
        todo!();
        /*
            if (! area.isEmpty())
            setTransform (placement.getTransformToFit (getDrawableBounds(), area));
        */
    }
    
    /**
      | Tries to turn some kind of image file
      | into a drawable.
      | 
      | The data could be an image that the ImageFileFormat
      | class understands, or it could be SVG.
      |
      */
    pub fn create_from_image_data(&mut self, 
        data:      *const c_void,
        num_bytes: usize) -> Box<Drawable> {
        
        todo!();
        /*
            auto image = ImageFileFormat::loadFrom (data, numBytes);

        if (image.isValid())
            return std::make_unique<DrawableImage> (image);

        if (auto svg = parseXMLIfTagMatches (String::createStringFromData (data, (int) numBytes), "svg"))
            return Drawable::createFromSVG (*svg);

        return {};
        */
    }
    
    /**
      | Tries to turn a stream containing some
      | kind of image data into a drawable.
      | 
      | The data could be an image that the ImageFileFormat
      | class understands, or it could be SVG.
      |
      */
    pub fn create_from_image_data_stream<R: Read>(
        &mut self, 
        data_source: &mut R
    ) -> Box<Drawable> {
        
        todo!();
        /*
            MemoryOutputStream mo;
        mo << dataSource;

        return createFromImageData (mo.getData(), mo.getDataSize());
        */
    }
    
    /**
      | Tries to turn a file containing some
      | kind of image data into a drawable.
      | 
      | The data could be an image that the ImageFileFormat
      | class understands, or it could be SVG.
      |
      */
    pub fn create_from_image_file(&mut self, file: &File) -> Box<Drawable> {
        
        todo!();
        /*
            FileInputStream fin (file);

        if (fin.openedOk())
            return createFromImageDataStream (fin);

        return {};
        */
    }
}
