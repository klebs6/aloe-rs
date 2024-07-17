crate::ix!();

pub fn coords_to_rectangle<Type: Copy>(
    x: Type,
    y: Type,
    w: Type,
    h: Type) -> Rectangle<Type> {

    todo!();
    /*
        #if ALOE_DEBUG
            const int maxVal = 0x3fffffff;

            jassert ((int) x >= -maxVal && (int) x <= maxVal
                  && (int) y >= -maxVal && (int) y <= maxVal
                  && (int) w >= 0 && (int) w <= maxVal
                  && (int) h >= 0 && (int) h <= maxVal);
           #endif

            return { x, y, w, h };
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/contexts/aloe_GraphicsContext.h]

/**
  | Types of rendering quality that can
  | be specified when drawing images. @see
  | Graphics::setImageResamplingQuality
  |
  */
pub enum GraphicsResamplingQuality
{
    /**
      | Just uses a nearest-neighbour algorithm
      | for resampling.
      |
      */
    lowResamplingQuality     = 0,    

    /**
      | Uses bilinear interpolation for upsampling
      | and area-averaging for downsampling.
      |
      */
    mediumResamplingQuality  = 1,    

    /**
      | Uses bicubic interpolation for upsampling
      | and area-averaging for downsampling.
      |
      */
    highResamplingQuality    = 2,    
}

/**
  | A graphics context, used for drawing
  | a component or image.
  | 
  | When a Component needs painting, a Graphics
  | context is passed to its Component::paint()
  | method, and this you then call methods
  | within this object to actually draw
  | the component's content.
  | 
  | A Graphics can also be created from an
  | image, to allow drawing directly onto
  | that image.
  | 
  | @see Component::paint
  | 
  | @tags{Graphics}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Graphics<'a> {
    context_holder:     Box<dyn LowLevelGraphicsContext>,
    context:            &'a mut dyn LowLevelGraphicsContext,
    save_state_pending: bool, // default = false
}

/**
  | Uses RAII to save and restore the state
  | of a graphics context.
  | 
  | On construction, this calls Graphics::saveState(),
  | and on destruction it calls Graphics::restoreState()
  | on the Graphics object that you supply.
  |
  */
#[no_copy]
pub struct GraphicsScopedSaveState<'a>
{
    context: &'a mut Graphics<'a>,
}

impl<'a> Drop for GraphicsScopedSaveState<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
        context.restoreState();
 */
    }
}

impl<'a> GraphicsScopedSaveState<'a> {
    
    pub fn new(g: &mut Graphics) -> Self {
    
        todo!();
        /*
        : context(g),

            context.saveState();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/contexts/aloe_GraphicsContext.cpp]
impl<'a> Graphics<'a> {

    /**
      | @internal
      |
      */
    pub fn get_internal_context(&self) -> &mut dyn LowLevelGraphicsContext {
        
        todo!();
        /*
            return context;
        */
    }
    
    /**
      | Creates a Graphics object to draw directly
      | onto the given image.
      | 
      | The graphics object that is created
      | will be set up to draw onto the image,
      | with the context's clipping area being
      | the entire size of the image, and its
      | origin being the image's origin. To
      | draw into a subsection of an image, use
      | the reduceClipRegion() and setOrigin()
      | methods.
      | 
      | Obviously you shouldn't delete the
      | image before this context is deleted.
      |
      */
    pub fn new_with_image_to_draw_into(image_to_draw_onto: &Image) -> Self {
    
        todo!();
        /*


            : contextHolder (imageToDrawOnto.createLowLevelContext()),
          context (*contextHolder)
        jassert (imageToDrawOnto.isValid()); // Can't draw into a null image!
        */
    }
    
    /**
      | Create a graphics that draws with a given
      | low-level renderer.
      | 
      | This method is intended for use only
      | by people who know what they're doing.
      | 
      | -----------
      | @note
      | 
      | the LowLevelGraphicsContext will
      | NOT be deleted by this object.
      |
      */
    pub fn new(internal_context: &mut dyn LowLevelGraphicsContext) -> Self {
    
        todo!();
        /*
        : context(internalContext),
        */
    }
    
    /**
      | Resets the current colour, brush, and
      | font to default settings.
      |
      */
    pub fn reset_to_default_state(&mut self)  {
        
        todo!();
        /*
            saveStateIfPending();
        context.setFill (FillType());
        context.setFont (Font());
        context.setInterpolationQuality (Graphics::mediumResamplingQuality);
        */
    }
    
    /**
      | Returns true if this context is drawing
      | to a vector-based device, such as a printer.
      |
      */
    pub fn is_vector_device(&self) -> bool {
        
        todo!();
        /*
            return context.isVectorDevice();
        */
    }
    
    /**
      | Intersects the current clipping region
      | with another region.
      | 
      | -----------
      | @return
      | 
      | true if the resulting clipping region
      | is non-zero in size @see setOrigin,
      | clipRegionIntersects
      |
      */
    pub fn reduce_clip_region_with_area(&mut self, area: Rectangle<i32>) -> bool {
        
        todo!();
        /*
            saveStateIfPending();
        return context.clipToRectangle (area);
        */
    }
    
    /**
      | Intersects the current clipping region
      | with another region.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the resulting clipping region
      | is non-zero in size @see setOrigin,
      | clipRegionIntersects
      |
      */
    pub fn reduce_clip_region_with_xywh(
        &mut self, 
        x: i32,
        y: i32,
        w: i32,
        h: i32) -> bool {
        
        todo!();
        /*
            return reduceClipRegion (coordsToRectangle (x, y, w, h));
        */
    }
    
    /**
      | Intersects the current clipping region
      | with a rectangle list region.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the resulting clipping region
      | is non-zero in size @see setOrigin,
      | clipRegionIntersects
      |
      */
    pub fn reduce_clip_region_with_clip_region(&mut self, clip_region: &RectangleList<i32>) -> bool {
        
        todo!();
        /*
            saveStateIfPending();
        return context.clipToRectangleList (clipRegion);
        */
    }
    
    /**
      | Intersects the current clipping region
      | with a path.
      | 
      | -----------
      | @return
      | 
      | true if the resulting clipping region
      | is non-zero in size @see reduceClipRegion
      |
      */
    pub fn reduce_clip_region_with_path_and_transform(
        &mut self, 
        path:      &Path,
        transform: Option<&AffineTransform>

    ) -> bool {

        let transform: &AffineTransform = transform.unwrap_or(&AffineTransform::default());
        
        todo!();
        /*
            saveStateIfPending();
        context.clipToPath (path, transform);
        return ! context.isClipEmpty();
        */
    }
    
    /**
      | Intersects the current clipping region
      | with an image's alpha-channel.
      | 
      | The current clipping path is intersected
      | with the area covered by this image's
      | alpha-channel, after the image has
      | been transformed by the specified matrix.
      | 
      | -----------
      | @param image
      | 
      | the image whose alpha-channel should
      | be used. If the image doesn't have an
      | alpha-channel, it is treated as entirely
      | opaque.
      | ----------
      | @param transform
      | 
      | a matrix to apply to the image
      | 
      | -----------
      | @return
      | 
      | true if the resulting clipping region
      | is non-zero in size @see reduceClipRegion
      |
      */
    pub fn reduce_clip_region_with_image_and_transform(
        &mut self, 
        image:     &Image,
        transform: &AffineTransform) -> bool {
        
        todo!();
        /*
            saveStateIfPending();
        context.clipToImageAlpha (image, transform);
        return ! context.isClipEmpty();
        */
    }
    
    /**
      | Excludes a rectangle to stop it being
      | drawn into.
      |
      */
    pub fn exclude_clip_region(&mut self, rectangle_to_exclude: Rectangle<i32>)  {
        
        todo!();
        /*
            saveStateIfPending();
        context.excludeClipRectangle (rectangleToExclude);
        */
    }
    
    /**
      | Returns true if no drawing can be done
      | because the clip region is zero.
      |
      */
    pub fn is_clip_empty(&self) -> bool {
        
        todo!();
        /*
            return context.isClipEmpty();
        */
    }
    
    /**
      | Returns the position of the bounding
      | box for the current clipping region.
      | @see clipRegionIntersects
      |
      */
    pub fn get_clip_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return context.getClipBounds();
        */
    }
    
    /**
      | Saves the current graphics state on
      | an internal stack.
      | 
      | To restore the state, use restoreState().
      | @see ScopedSaveState
      |
      */
    pub fn save_state(&mut self)  {
        
        todo!();
        /*
            saveStateIfPending();
        saveStatePending = true;
        */
    }
    
    /**
      | Restores a graphics state that was previously
      | saved with saveState(). @see ScopedSaveState
      |
      */
    pub fn restore_state(&mut self)  {
        
        todo!();
        /*
            if (saveStatePending)
            saveStatePending = false;
        else
            context.restoreState();
        */
    }
    
    pub fn save_state_if_pending(&mut self)  {
        
        todo!();
        /*
            if (saveStatePending)
        {
            saveStatePending = false;
            context.saveState();
        }
        */
    }
    
    /**
      | Moves the position of the context's
      | origin.
      | 
      | This changes the position that the context
      | considers to be (0, 0) to the specified
      | position.
      | 
      | So if you call setOrigin with (100, 100),
      | then the position that was previously
      | referred to as (100, 100) will subsequently
      | be considered to be (0, 0).
      | 
      | @see reduceClipRegion, addTransform
      |
      */
    pub fn set_origin_point(&mut self, new_origin: Point<i32>)  {
        
        todo!();
        /*
            saveStateIfPending();
        context.setOrigin (newOrigin);
        */
    }
    
    /**
      | Moves the position of the context's
      | origin.
      | 
      | This changes the position that the context
      | considers to be (0, 0) to the specified
      | position.
      | 
      | So if you call setOrigin (100, 100),
      | then the position that was previously
      | referred to as (100, 100) will subsequently
      | be considered to be (0, 0).
      | 
      | @see reduceClipRegion, addTransform
      |
      */
    pub fn set_origin(&mut self, x: i32, y: i32)  {
        
        todo!();
        /*
            setOrigin ({ x, y });
        */
    }
    
    /**
      | Adds a transformation which will be
      | performed on all the graphics operations
      | that the context subsequently performs.
      | 
      | After calling this, all the coordinates
      | that are passed into the context will
      | be transformed by this matrix.
      | 
      | @see setOrigin
      |
      */
    pub fn add_transform(&mut self, transform: &AffineTransform)  {
        
        todo!();
        /*
            saveStateIfPending();
        context.addTransform (transform);
        */
    }
    
    /**
      | Checks whether a rectangle overlaps
      | the context's clipping region.
      | 
      | If this returns false, no part of the
      | given area can be drawn onto, so this
      | method can be used to optimise a component's
      | paint() method, by letting it avoid
      | drawing complex objects that aren't
      | within the region being repainted.
      |
      */
    pub fn clip_region_intersects(&self, area: Rectangle<i32>) -> bool {
        
        todo!();
        /*
            return context.clipRegionIntersects (area);
        */
    }
    
    /**
      | Begins rendering to an off-screen bitmap
      | which will later be flattened onto the
      | current context with the given opacity.
      | 
      | The context uses an internal stack of
      | temporary image layers to do this. When
      | you've finished drawing to the layer,
      | call endTransparencyLayer() to complete
      | the operation and composite the finished
      | layer. Every call to beginTransparencyLayer()
      | MUST be matched by a corresponding call
      | to endTransparencyLayer()!
      | 
      | This call also saves the current state,
      | and endTransparencyLayer() restores
      | it.
      |
      */
    pub fn begin_transparency_layer(&mut self, layer_opacity: f32)  {
        
        todo!();
        /*
            saveStateIfPending();
        context.beginTransparencyLayer (layerOpacity);
        */
    }
    
    /**
      | Completes a drawing operation to a temporary
      | semi-transparent buffer. See beginTransparencyLayer()
      | for more details.
      |
      */
    pub fn end_transparency_layer(&mut self)  {
        
        todo!();
        /*
            context.endTransparencyLayer();
        */
    }
    
    /**
      | Changes the current drawing colour.
      | 
      | This sets the colour that will now be
      | used for drawing operations - it also
      | sets the opacity to that of the colour
      | passed-in.
      | 
      | If a brush is being used when this method
      | is called, the brush will be deselected,
      | and any subsequent drawing will be done
      | with a solid colour brush instead.
      | 
      | @see setOpacity
      |
      */
    pub fn set_colour(&mut self, new_colour: Colour)  {
        
        todo!();
        /*
            saveStateIfPending();
        context.setFill (newColour);
        */
    }
    
    /**
      | Changes the opacity to use with the current
      | colour.
      | 
      | If a solid colour is being used for drawing,
      | this changes its opacity to this new
      | value (i.e. it doesn't multiply the
      | colour's opacity by this amount).
      | 
      | If a gradient is being used, this will
      | have no effect on it.
      | 
      | A value of 0.0 is completely transparent,
      | 1.0 is completely opaque.
      |
      */
    pub fn set_opacity(&mut self, new_opacity: f32)  {
        
        todo!();
        /*
            saveStateIfPending();
        context.setOpacity (newOpacity);
        */
    }
    
    /**
      | Sets the context to use a gradient for
      | its fill pattern.
      |
      */
    pub fn set_gradient_fill_with_gradient_ref(&mut self, gradient: &ColourGradient)  {
        
        todo!();
        /*
            setFillType (gradient);
        */
    }
    
    /**
      | Sets the context to use a gradient for
      | its fill pattern.
      |
      */
    pub fn set_gradient_fill(&mut self, gradient: ColourGradient)  {
        
        todo!();
        /*
            setFillType (std::move (gradient));
        */
    }
    
    /**
      | Sets the context to use a tiled image
      | pattern for filling.
      | 
      | Make sure that you don't delete this
      | image while it's still being used by
      | this context!
      |
      */
    pub fn set_tiled_image_fill(&mut self, 
        image_to_use: &Image,
        anchorx:      i32,
        anchory:      i32,
        opacity:      f32)  {
        
        todo!();
        /*
            saveStateIfPending();
        context.setFill (FillType (imageToUse, AffineTransform::translation ((float) anchorX, (float) anchorY)));
        context.setOpacity (opacity);
        */
    }
    
    /**
      | Changes the current fill settings.
      | @see setColour, setGradientFill,
      | setTiledImageFill
      |
      */
    pub fn set_fill_type(&mut self, new_fill: &FillType)  {
        
        todo!();
        /*
            saveStateIfPending();
        context.setFill (newFill);
        */
    }
    
    /**
      | Changes the font to use for subsequent
      | text-drawing functions. @see drawSingleLineText,
      | drawMultiLineText, drawText, drawFittedText
      |
      */
    pub fn set_font(&mut self, new_font: &Font)  {
        
        todo!();
        /*
            saveStateIfPending();
        context.setFont (newFont);
        */
    }
    
    /**
      | Changes the size of the currently-selected
      | font.
      | 
      | This is a convenient shortcut that changes
      | the context's current font to a different
      | size. The typeface won't be changed.
      | @see Font
      |
      */
    pub fn set_font_change_size(&mut self, new_font_height: f32)  {
        
        todo!();
        /*
            setFont (context.getFont().withHeight (newFontHeight));
        */
    }
    
    /**
      | Returns the currently selected font.
      |
      */
    pub fn get_current_font(&self) -> Font {
        
        todo!();
        /*
            return context.getFont();
        */
    }
    
    /**
      | Draws a one-line text string.
      | 
      | This will use the current colour (or
      | brush) to fill the text. The font is the
      | last one specified by setFont().
      | 
      | -----------
      | @param text
      | 
      | the string to draw
      | ----------
      | @param startX
      | 
      | the position to draw the left-hand edge
      | of the text
      | ----------
      | @param baselineY
      | 
      | the position of the text's baseline
      | ----------
      | @param justification
      | 
      | the horizontal flags indicate which
      | end of the text string is anchored at
      | the specified point.
      | 
      | @see drawMultiLineText, drawText,
      | drawFittedText, GlyphArrangement::addLineOfText
      |
      */
    pub fn draw_single_line_text(
        &self, 
        text:          &String,
        startx:        i32,
        baseliney:     i32,
        justification: Option<JustificationFlags>

    )  {

        let justification = justification.unwrap_or(JustificationFlags::left);
        
        todo!();
        /*
            if (text.isNotEmpty())
        {
            // Don't pass any vertical placement flags to this method - they'll be ignored.
            jassert (justification.getOnlyVerticalFlags() == 0);

            auto flags = justification.getOnlyHorizontalFlags();

            if (flags == Justification::right && startX < context.getClipBounds().getX())
                return;

            if (flags == Justification::left && startX > context.getClipBounds().getRight())
                return;

            GlyphArrangement arr;
            arr.addLineOfText (context.getFont(), text, (float) startX, (float) baselineY);

            if (flags != Justification::left)
            {
                auto w = arr.getBoundingBox (0, -1, true).getWidth();

                if ((flags & (Justification::horizontallyCentred | Justification::horizontallyJustified)) != 0)
                    w /= 2.0f;

                arr.draw (*this, AffineTransform::translation (-w, 0));
            }
            else
            {
                arr.draw (*this);
            }
        }
        */
    }
    
    /**
      | Draws text across multiple lines.
      | 
      | This will break the text onto a new line
      | where there's a new-line or carriage-return
      | character, or at a word-boundary when
      | the text becomes wider than the size
      | specified by the maximumLineWidth
      | parameter. New-lines will be vertically
      | separated by the specified leading.
      | 
      | @see setFont, drawSingleLineText,
      | drawFittedText, GlyphArrangement::addJustifiedText
      |
      */
    pub fn draw_multi_line_text(&self, 
        text:               &String,
        startx:             i32,
        baseliney:          i32,
        maximum_line_width: i32,
        justification:      Option<JustificationFlags>,
        leading:            Option<f32>

    ) {

        let justification = justification.unwrap_or(JustificationFlags::left);
        let leading       = leading.unwrap_or(0.0);
        
        todo!();
        /*
            if (text.isNotEmpty()
             && startX < context.getClipBounds().getRight())
        {
            GlyphArrangement arr;
            arr.addJustifiedText (context.getFont(), text,
                                  (float) startX, (float) baselineY, (float) maximumLineWidth,
                                  justification, leading);
            arr.draw (*this);
        }
        */
    }
    
    /**
      | Draws a line of text within a specified
      | rectangle.
      | 
      | The text will be positioned within the
      | rectangle based on the justification
      | flags passed-in. If the string is too
      | long to fit inside the rectangle, it
      | will either be truncated or will have
      | ellipsis added to its end (if the useEllipsesIfTooBig
      | flag is true).
      | 
      | @see drawSingleLineText, drawFittedText,
      | drawMultiLineText, GlyphArrangement::addJustifiedText
      |
      */
    pub fn draw_text_within_area(
        &self, 
        text:                    &String,
        area:                    Rectangle<f32>,
        justification_type:      Justification,
        use_ellipses_if_too_big: Option<bool>,

    )  {

        let use_ellipses_if_too_big: bool = use_ellipses_if_too_big.unwrap_or(true);
        
        todo!();
        /*
            if (text.isNotEmpty() && context.clipRegionIntersects (area.getSmallestIntegerContainer()))
        {
            GlyphArrangement arr;
            arr.addCurtailedLineOfText (context.getFont(), text, 0.0f, 0.0f,
                                        area.getWidth(), useEllipsesIfTooBig);

            arr.justifyGlyphs (0, arr.getNumGlyphs(),
                               area.getX(), area.getY(), area.getWidth(), area.getHeight(),
                               justificationType);
            arr.draw (*this);
        }
        */
    }
    
    /**
      | Draws a line of text within a specified
      | rectangle.
      | 
      | The text will be positioned within the
      | rectangle based on the justification
      | flags passed-in. If the string is too
      | long to fit inside the rectangle, it
      | will either be truncated or will have
      | ellipsis added to its end (if the useEllipsesIfTooBig
      | flag is true).
      | 
      | @see drawSingleLineText, drawFittedText,
      | drawMultiLineText, GlyphArrangement::addJustifiedText
      |
      */
    pub fn draw_text_within_area_i32(
        &self, 
        text:                    &String,
        area:                    Rectangle<i32>,
        justification_type:      Justification,
        use_ellipses_if_too_big: Option<bool>

    ) {

        let use_ellipses_if_too_big: bool = use_ellipses_if_too_big.unwrap_or(true);
        
        todo!();
        /*
            drawText (text, area.toFloat(), justificationType, useEllipsesIfTooBig);
        */
    }
    
    /**
      | Draws a line of text within a specified
      | rectangle.
      | 
      | The text will be positioned within the
      | rectangle based on the justification
      | flags passed-in. If the string is too
      | long to fit inside the rectangle, it
      | will either be truncated or will have
      | ellipsis added to its end (if the useEllipsesIfTooBig
      | flag is true).
      | 
      | @see drawSingleLineText, drawFittedText,
      | drawMultiLineText, GlyphArrangement::addJustifiedText
      |
      */
    pub fn draw_text(&self, 
        text:                    &String,
        x:                       i32,
        y:                       i32,
        width:                   i32,
        height:                  i32,
        justification_type:      Justification,
        use_ellipses_if_too_big: Option<bool>

    )  {

        let use_ellipses_if_too_big: bool = use_ellipses_if_too_big.unwrap_or(true);
        
        todo!();
        /*
            drawText (text, coordsToRectangle (x, y, width, height), justificationType, useEllipsesIfTooBig);
        */
    }
    
    /**
      | Tries to draw a text string inside a given
      | space.
      | 
      | This does its best to make the given text
      | readable within the specified rectangle,
      | so it's useful for labelling things.
      | 
      | If the text is too big, it'll be squashed
      | horizontally or broken over multiple
      | lines if the maximumLinesToUse value
      | allows this. If the text just won't fit
      | into the space, it'll cram as much as
      | possible in there, and put some ellipsis
      | at the end to show that it's been truncated.
      | 
      | A Justification parameter lets you
      | specify how the text is laid out within
      | the rectangle, both horizontally and
      | vertically.
      | 
      | The minimumHorizontalScale parameter
      | specifies how much the text can be squashed
      | horizontally to try to squeeze it into
      | the space. If you don't want any horizontal
      | scaling to occur, you can set this value
      | to 1.0f. Pass 0 if you want it to use a default
      | value.
      | 
      | @see GlyphArrangement::addFittedText
      |
      */
    pub fn draw_fitted_text_within_area(
        &self, 
        text:                     &String,
        area:                     Rectangle<i32>,
        justification:            Justification,
        maximum_number_of_lines:  i32,
        minimum_horizontal_scale: Option<f32>

    )  {

        let minimum_horizontal_scale: f32 = minimum_horizontal_scale.unwrap_or(0.0);
        
        todo!();
        /*
            if (text.isNotEmpty() && (! area.isEmpty()) && context.clipRegionIntersects (area))
        {
            GlyphArrangement arr;
            arr.addFittedText (context.getFont(), text,
                               (float) area.getX(), (float) area.getY(),
                               (float) area.getWidth(), (float) area.getHeight(),
                               justification,
                               maximumNumberOfLines,
                               minimumHorizontalScale);

            arr.draw (*this);
        }
        */
    }
    
    /**
      | Tries to draw a text string inside a given
      | space.
      | 
      | This does its best to make the given text
      | readable within the specified rectangle,
      | so it's useful for labelling things.
      | 
      | If the text is too big, it'll be squashed
      | horizontally or broken over multiple
      | lines if the maximumLinesToUse value
      | allows this. If the text just won't fit
      | into the space, it'll cram as much as
      | possible in there, and put some ellipsis
      | at the end to show that it's been truncated.
      | 
      | A Justification parameter lets you
      | specify how the text is laid out within
      | the rectangle, both horizontally and
      | vertically.
      | 
      | The minimumHorizontalScale parameter
      | specifies how much the text can be squashed
      | horizontally to try to squeeze it into
      | the space. If you don't want any horizontal
      | scaling to occur, you can set this value
      | to 1.0f. Pass 0 if you want it to use a default
      | value.
      | 
      | @see GlyphArrangement::addFittedText
      |
      */
    pub fn draw_fitted_text(&self, 
        text:                     &String,
        x:                        i32,
        y:                        i32,
        width:                    i32,
        height:                   i32,
        justification:            Justification,
        maximum_number_of_lines:  i32,
        minimum_horizontal_scale: Option<f32>

    )  {

        let minimum_horizontal_scale: f32 = minimum_horizontal_scale.unwrap_or(0.0);
        
        todo!();
        /*
            drawFittedText (text, coordsToRectangle (x, y, width, height),
                        justification, maximumNumberOfLines, minimumHorizontalScale);
        */
    }
    
    /**
      | Fills a rectangle with the current colour
      | or brush. @see drawRect, fillRoundedRectangle
      |
      */
    pub fn fill_rect_with_i32_rect(&self, r: Rectangle<i32>)  {
        
        todo!();
        /*
            context.fillRect (r, false);
        */
    }
    
    /**
      | Fills a rectangle with the current colour
      | or brush. @see drawRect, fillRoundedRectangle
      |
      */
    pub fn fill_rect_with_f32_rect(&self, r: Rectangle<f32>)  {
        
        todo!();
        /*
            context.fillRect (r);
        */
    }
    
    /**
      | Fills a rectangle with the current colour
      | or brush. @see drawRect, fillRoundedRectangle
      |
      */
    pub fn fill_rect_with_integral_params(&self, 
        x:      i32,
        y:      i32,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            context.fillRect (coordsToRectangle (x, y, width, height), false);
        */
    }
    
    /**
      | Fills a rectangle with the current colour
      | or brush. @see drawRect, fillRoundedRectangle
      |
      */
    pub fn fill_rect_with_float_params(&self, 
        x:      f32,
        y:      f32,
        width:  f32,
        height: f32)  {
        
        todo!();
        /*
            fillRect (coordsToRectangle (x, y, width, height));
        */
    }
    
    /**
      | Fills a set of rectangles using the current
      | colour or brush.
      | 
      | If you have a lot of rectangles to draw,
      | it may be more efficient to create a RectangleList
      | and use this method than to call fillRect()
      | multiple times.
      |
      */
    pub fn fill_rect_list_f32(&self, rectangles: &RectangleList<f32>)  {
        
        todo!();
        /*
            context.fillRectList (rectangles);
        */
    }
    
    /**
      | Fills a set of rectangles using the current
      | colour or brush. If you have a lot of rectangles
      | to draw, it may be more efficient to create
      | a RectangleList and use this method
      | than to call fillRect() multiple times.
      |
      */
    pub fn fill_rect_list(&self, rects: &RectangleList<i32>)  {
        
        todo!();
        /*
            for (auto& r : rects)
            context.fillRect (r, false);
        */
    }
    
    /**
      | Fills the context's entire clip region
      | with the current colour or brush.
      | 
      | (See also the fillAll (Colour) method
      | which is a quick way of filling it with
      | a given colour).
      |
      */
    pub fn fill_all(&self)  {
        
        todo!();
        /*
            fillRect (context.getClipBounds());
        */
    }
    
    /**
      | Fills the context's entire clip region
      | with a given colour.
      | 
      | This leaves the context's current colour
      | and brush unchanged, it just uses the
      | specified colour temporarily.
      |
      */
    pub fn fill_all_with_colour(&self, colour_to_use: Colour)  {
        
        todo!();
        /*
            if (! colourToUse.isTransparent())
        {
            auto clip = context.getClipBounds();

            context.saveState();
            context.setFill (colourToUse);
            context.fillRect (clip, false);
            context.restoreState();
        }
        */
    }
    
    /**
      | Fills a path using the currently selected
      | colour or brush.
      |
      */
    pub fn fill_path(&self, path: &Path)  {
        
        todo!();
        /*
            if (! (context.isClipEmpty() || path.isEmpty()))
            context.fillPath (path, AffineTransform());
        */
    }
    
    /**
      | Fills a path using the currently selected
      | colour or brush, and adds a transform.
      |
      */
    pub fn fill_path_with_transform(
        &self, 
        path:      &Path,
        transform: &AffineTransform)  {
        
        todo!();
        /*
            if (! (context.isClipEmpty() || path.isEmpty()))
            context.fillPath (path, transform);
        */
    }
    
    /**
      | Draws a path's outline using the currently
      | selected colour or brush.
      |
      */
    pub fn stroke_path(&self, 
        path:        &Path,
        stroke_type: &PathStrokeType,
        transform:   &AffineTransform)  {
        
        todo!();
        /*
            Path stroke;
        strokeType.createStrokedPath (stroke, path, transform, context.getPhysicalPixelScaleFactor());
        fillPath (stroke);
        */
    }
    
    /**
      | Draws a rectangular outline, using
      | the current colour or brush.
      | 
      | The lines are drawn inside the given
      | rectangle, and greater line thicknesses
      | extend inwards. @see fillRect
      |
      */
    pub fn draw_rect_with_xywh(
        &self, 
        x:              f32,
        y:              f32,
        width:          f32,
        height:         f32,
        line_thickness: Option<f32>

    ) {

        let line_thickness: f32 = line_thickness.unwrap_or(1.0);
        
        todo!();
        /*
            drawRect (coordsToRectangle (x, y, width, height), lineThickness);
        */
    }
    
    /**
      | Draws a rectangular outline, using
      | the current colour or brush.
      | 
      | The lines are drawn inside the given
      | rectangle, and greater line thicknesses
      | extend inwards. @see fillRect
      |
      */
    pub fn draw_rect_with_i32_xywh(
        &self, 
        x:              i32,
        y:              i32,
        width:          i32,
        height:         i32,
        line_thickness: Option<i32>

    ) {

        let line_thickness: i32 = line_thickness.unwrap_or(1);
        
        todo!();
        /*
            drawRect (coordsToRectangle (x, y, width, height), lineThickness);
        */
    }
    
    /**
      | Draws a rectangular outline, using
      | the current colour or brush.
      | 
      | The lines are drawn inside the given
      | rectangle, and greater line thicknesses
      | extend inwards. @see fillRect
      |
      */
    pub fn draw_rect_with_thickness(
        &self, 
        r:              Rectangle<i32>,
        line_thickness: Option<i32>

    )  {

        let line_thickness: i32 = line_thickness.unwrap_or(1);
        
        todo!();
        /*
            drawRect (r.toFloat(), (float) lineThickness);
        */
    }
    
    /**
      | Draws a rectangular outline, using
      | the current colour or brush.
      | 
      | The lines are drawn inside the given
      | rectangle, and greater line thicknesses
      | extend inwards. @see fillRect
      |
      */
    pub fn draw_rect_with_thickness_f32(
        &self, 
        r:              Rectangle<f32>,
        line_thickness: Option<f32>
    ) {

        let line_thickness: f32 = line_thickness.unwrap_or(1.0);
        
        todo!();
        /*
            jassert (r.getWidth() >= 0.0f && r.getHeight() >= 0.0f);

        RectangleList<float> rects;
        rects.addWithoutMerging (r.removeFromTop    (lineThickness));
        rects.addWithoutMerging (r.removeFromBottom (lineThickness));
        rects.addWithoutMerging (r.removeFromLeft   (lineThickness));
        rects.addWithoutMerging (r.removeFromRight  (lineThickness));
        context.fillRectList (rects);
        */
    }
    
    /**
      | Fills an ellipse with the current colour
      | or brush.
      | 
      | The ellipse is drawn to fit inside the
      | given rectangle. @see drawEllipse,
      | Path::addEllipse
      |
      */
    pub fn fill_ellipse_with_area(&self, area: Rectangle<f32>)  {
        
        todo!();
        /*
            Path p;
        p.addEllipse (area);
        fillPath (p);
        */
    }
    
    /**
      | Fills an ellipse with the current colour
      | or brush.
      | 
      | The ellipse is drawn to fit inside the
      | given rectangle. @see drawEllipse,
      | Path::addEllipse
      |
      */
    pub fn fill_ellipse(&self, 
        x: f32,
        y: f32,
        w: f32,
        h: f32)  {
        
        todo!();
        /*
            fillEllipse (coordsToRectangle (x, y, w, h));
        */
    }
    
    /**
      | Draws an elliptical stroke using the
      | current colour or brush. @see fillEllipse,
      | Path::addEllipse
      |
      */
    pub fn draw_ellipse(
        &self, 
        x:              f32,
        y:              f32,
        width:          f32,
        height:         f32,
        line_thickness: f32)  {
        
        todo!();
        /*
            drawEllipse (coordsToRectangle (x, y, width, height), lineThickness);
        */
    }
    
    /**
      | Draws an elliptical stroke using the
      | current colour or brush. @see fillEllipse,
      | Path::addEllipse
      |
      */
    pub fn draw_ellipse_with_area(
        &self, 
        area:           Rectangle<f32>,
        line_thickness: f32)  {
        
        todo!();
        /*
            Path p;

        if (area.getWidth() == area.getHeight())
        {
            // For a circle, we can avoid having to generate a stroke
            p.addEllipse (area.expanded (lineThickness * 0.5f));
            p.addEllipse (area.reduced  (lineThickness * 0.5f));
            p.setUsingNonZeroWinding (false);
            fillPath (p);
        }
        else
        {
            p.addEllipse (area);
            strokePath (p, PathStrokeType (lineThickness));
        }
        */
    }
    
    /**
      | Uses the current colour or brush to fill
      | a rectangle with rounded corners. @see
      | drawRoundedRectangle, Path::addRoundedRectangle
      |
      */
    pub fn fill_rounded_rectangle(
        &self, 
        x:           f32,
        y:           f32,
        width:       f32,
        height:      f32,
        corner_size: f32

    ) {
        
        todo!();
        /*
            fillRoundedRectangle (coordsToRectangle (x, y, width, height), cornerSize);
        */
    }
    
    /**
      | Uses the current colour or brush to fill
      | a rectangle with rounded corners. @see
      | drawRoundedRectangle, Path::addRoundedRectangle
      |
      */
    pub fn fill_rounded_rectangle_with_rect(
        &self, 
        r:           Rectangle<f32>,
        corner_size: f32

    ) {
        
        todo!();
        /*
            Path p;
        p.addRoundedRectangle (r, cornerSize);
        fillPath (p);
        */
    }
    
    /**
      | Uses the current colour or brush to draw
      | the outline of a rectangle with rounded
      | corners. @see fillRoundedRectangle,
      | Path::addRoundedRectangle
      |
      */
    pub fn draw_rounded_rectangle(
        &self, 
        x:              f32,
        y:              f32,
        width:          f32,
        height:         f32,
        corner_size:    f32,
        line_thickness: f32)  {
        
        todo!();
        /*
            drawRoundedRectangle (coordsToRectangle (x, y, width, height), cornerSize, lineThickness);
        */
    }
    
    /**
      | Uses the current colour or brush to draw
      | the outline of a rectangle with rounded
      | corners. @see fillRoundedRectangle,
      | Path::addRoundedRectangle
      |
      */
    pub fn draw_rounded_rectangle_with_rect(
        &self, 
        r:              Rectangle<f32>,
        corner_size:    f32,
        line_thickness: f32)  {
        
        todo!();
        /*
            Path p;
        p.addRoundedRectangle (r, cornerSize);
        strokePath (p, PathStrokeType (lineThickness));
        */
    }
    
    /**
      | Draws a line with an arrowhead at its
      | end.
      | 
      | -----------
      | @param line
      | 
      | the line to draw
      | ----------
      | @param lineThickness
      | 
      | the thickness of the line
      | ----------
      | @param arrowheadWidth
      | 
      | the width of the arrow head (perpendicular
      | to the line)
      | ----------
      | @param arrowheadLength
      | 
      | the length of the arrow head (along the
      | length of the line)
      |
      */
    pub fn draw_arrow(&self, 
        line:             Line<f32>,
        line_thickness:   f32,
        arrowhead_width:  f32,
        arrowhead_length: f32)  {
        
        todo!();
        /*
            Path p;
        p.addArrow (line, lineThickness, arrowheadWidth, arrowheadLength);
        fillPath (p);
        */
    }
    
    /**
      | Fills a rectangle with a checkerboard
      | pattern, alternating between two colours.
      |
      */
    pub fn fill_checker_board(&self, 
        area:         Rectangle<f32>,
        check_width:  f32,
        check_height: f32,
        colour1:      Colour,
        colour2:      Colour)  {
        
        todo!();
        /*
            jassert (checkWidth > 0 && checkHeight > 0); // can't be zero or less!

        if (checkWidth > 0 && checkHeight > 0)
        {
            context.saveState();

            if (colour1 == colour2)
            {
                context.setFill (colour1);
                context.fillRect (area);
            }
            else
            {
                auto clipped = context.getClipBounds().getIntersection (area.getSmallestIntegerContainer());

                if (! clipped.isEmpty())
                {
                    const int checkNumX = (int) (((float) clipped.getX() - area.getX()) / checkWidth);
                    const int checkNumY = (int) (((float) clipped.getY() - area.getY()) / checkHeight);
                    const float startX = area.getX() + (float) checkNumX * checkWidth;
                    const float startY = area.getY() + (float) checkNumY * checkHeight;
                    const float right  = (float) clipped.getRight();
                    const float bottom = (float) clipped.getBottom();

                    for (int i = 0; i < 2; ++i)
                    {
                        int cy = i;
                        RectangleList<float> checks;

                        for (float y = startY; y < bottom; y += checkHeight)
                            for (float x = startX + (cy++ & 1) * checkWidth; x < right; x += checkWidth * 2.0f)
                                checks.addWithoutMerging ({ x, y, checkWidth, checkHeight });

                        checks.clipTo (area);
                        context.setFill (i == ((checkNumX ^ checkNumY) & 1) ? colour1 : colour2);
                        context.fillRectList (checks);
                    }
                }
            }

            context.restoreState();
        }
        */
    }
    
    /**
      | Draws a vertical line of pixels at a given
      | x position.
      | 
      | The x position is an integer, but the
      | top and bottom of the line can be sub-pixel
      | positions, and these will be anti-aliased
      | if necessary.
      | 
      | The bottom parameter must be greater
      | than or equal to the top parameter.
      |
      */
    pub fn draw_vertical_line(&self, 
        x:      i32,
        top:    f32,
        bottom: f32)  {
        
        todo!();
        /*
            if (top < bottom)
            context.fillRect (Rectangle<float> ((float) x, top, 1.0f, bottom - top));
        */
    }
    
    /**
      | Draws a horizontal line of pixels at
      | a given y position.
      | 
      | The y position is an integer, but the
      | left and right ends of the line can be
      | sub-pixel positions, and these will
      | be anti-aliased if necessary.
      | 
      | The right parameter must be greater
      | than or equal to the left parameter.
      |
      */
    pub fn draw_horizontal_line(&self, 
        y:     i32,
        left:  f32,
        right: f32)  {
        
        todo!();
        /*
            if (left < right)
            context.fillRect (Rectangle<float> (left, (float) y, right - left, 1.0f));
        */
    }
    
    /**
      | Draws a line between two points.
      | 
      | The line is 1 pixel wide and drawn with
      | the current colour or brush.
      | 
      | TIP: If you're trying to draw horizontal
      | or vertical lines, don't use this - it's
      | better to use fillRect() instead unless
      | you really need an angled line.
      |
      */
    pub fn draw_line(&self, line: Line<f32>)  {
        
        todo!();
        /*
            context.drawLine (line);
        */
    }
    
    /**
      | Draws a line between two points.
      | 
      | The line is 1 pixel wide and drawn with
      | the current colour or brush.
      | 
      | TIP: If you're trying to draw horizontal
      | or vertical lines, don't use this - it's
      | better to use fillRect() instead unless
      | you really need an angled line.
      |
      */
    pub fn draw_line_between_two_points(
        &self, 
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32)  {
        
        todo!();
        /*
            context.drawLine (Line<float> (x1, y1, x2, y2));
        */
    }
    
    /**
      | Draws a line between two points with
      | a given thickness.
      | 
      | TIP: If you're trying to draw horizontal
      | or vertical lines, don't use this - it's
      | better to use fillRect() instead unless
      | you really need an angled line.
      | 
      | @see Path::addLineSegment
      |
      */
    pub fn draw_line_between_two_points_with_thickness(
        &self, 
        x1:             f32,
        y1:             f32,
        x2:             f32,
        y2:             f32,
        line_thickness: f32)  {
        
        todo!();
        /*
            drawLine (Line<float> (x1, y1, x2, y2), lineThickness);
        */
    }
    
    /**
      | Draws a line between two points with
      | a given thickness. @see Path::addLineSegment
      | 
      | TIP: If you're trying to draw horizontal
      | or vertical lines, don't use this - it's
      | better to use fillRect() instead unless
      | you really need an angled line.
      |
      */
    pub fn draw_line_with_thickness(
        &self, 
        line:           Line<f32>,
        line_thickness: f32)  {
        
        todo!();
        /*
            Path p;
        p.addLineSegment (line, lineThickness);
        fillPath (p);
        */
    }
    
    /**
      | Draws a dashed line using a custom set
      | of dash-lengths.
      | 
      | -----------
      | @param line
      | 
      | the line to draw
      | ----------
      | @param dashLengths
      | 
      | a series of lengths to specify the on/off
      | lengths - e.g. { 4, 5, 6, 7 } will draw a
      | line of 4 pixels, skip 5 pixels, draw
      | 6 pixels, skip 7 pixels, and then repeat.
      | ----------
      | @param numDashLengths
      | 
      | the number of elements in the array (this
      | must be an even number).
      | ----------
      | @param lineThickness
      | 
      | the thickness of the line to draw
      | ----------
      | @param dashIndexToStartFrom
      | 
      | the index in the dash-length array to
      | use for the first segment
      | 
      | @see PathStrokeType::createDashedStroke
      |
      */
    pub fn draw_dashed_line(
        &self, 
        line:                     Line<f32>,
        dash_lengths:             *const f32,
        num_dash_lengths:         i32,
        line_thickness:           Option<f32>,
        dash_index_to_start_from: Option<i32>,

    ) {

        let line_thickness:           f32 = line_thickness.unwrap_or(1.0);
        let dash_index_to_start_from: i32 = dash_index_to_start_from.unwrap_or(0);

        let n = dash_index_to_start_from;
        
        todo!();
        /*
            jassert (n >= 0 && n < numDashLengths); // your start index must be valid!

        const Point<double> delta ((line.getEnd() - line.getStart()).toDouble());
        const double totalLen = delta.getDistanceFromOrigin();

        if (totalLen >= 0.1)
        {
            const double onePixAlpha = 1.0 / totalLen;

            for (double alpha = 0.0; alpha < 1.0;)
            {
                jassert (dashLengths[n] > 0); // can't have zero-length dashes!

                const double lastAlpha = alpha;
                alpha += dashLengths [n] * onePixAlpha;
                n = (n + 1) % numDashLengths;

                if ((n & 1) != 0)
                {
                    const Line<float> segment (line.getStart() + (delta * lastAlpha).toFloat(),
                                               line.getStart() + (delta * jmin (1.0, alpha)).toFloat());

                    if (lineThickness != 1.0f)
                        drawLine (segment, lineThickness);
                    else
                        context.drawLine (segment);
                }
            }
        }
        */
    }
    
    /**
      | Changes the quality that will be used
      | when resampling images.
      | 
      | By default a Graphics object will be
      | set to mediumRenderingQuality. @see
      | Graphics::drawImage, Graphics::drawImageTransformed,
      | Graphics::drawImageWithin
      |
      */
    pub fn set_image_resampling_quality(&mut self, new_quality: GraphicsResamplingQuality)  {
        
        todo!();
        /*
            saveStateIfPending();
        context.setInterpolationQuality (newQuality);
        */
    }
    
    /**
      | Draws an image.
      | 
      | This will draw the whole of an image,
      | positioning its top-left corner at
      | the given coordinates, and keeping
      | its size the same. This is the simplest
      | image drawing method - the others give
      | more control over the scaling and clipping
      | of the images.
      | 
      | Images are composited using the context's
      | current opacity, so if you don't want
      | it to be drawn semi-transparently,
      | be sure to call setOpacity (1.0f) (or
      | setColour() with an opaque colour)
      | before drawing images.
      |
      */
    pub fn draw_image_at(
        &self, 
        image_to_draw:      &Image,
        x:                  i32,
        y:                  i32,
        fill_alpha_channel: Option<bool>

    )  {

        let fill_alpha_channel: bool = fill_alpha_channel.unwrap_or(false);
        
        todo!();
        /*
            drawImageTransformed (imageToDraw,
                              AffineTransform::translation ((float) x, (float) y),
                              fillAlphaChannel);
        */
    }
    
    /**
      | Draws an image to fit within a designated
      | rectangle.
      | 
      | -----------
      | @param imageToDraw
      | 
      | the source image to draw
      | ----------
      | @param targetArea
      | 
      | the target rectangle to fit it into
      | ----------
      | @param placementWithinTarget
      | 
      | this specifies how the image should
      | be positioned within the target rectangle
      | - see the RectanglePlacement class
      | for more details about this.
      | ----------
      | @param fillAlphaChannelWithCurrentBrush
      | 
      | if true, then instead of drawing the
      | image, just its alpha channel will be
      | used as a mask with which to draw with
      | the current brush or colour. This is
      | similar to fillAlphaMap(), and see
      | also drawImage() @see drawImage, drawImageTransformed,
      | drawImageAt, RectanglePlacement
      |
      */
    pub fn draw_image_with_area(
        &self, 
        image_to_draw:                         &Image,
        target_area:                           Rectangle<f32>,
        placement_within_target:               Option<RectanglePlacementFlags>,
        fill_alpha_channel_with_current_brush: Option<bool>

    ) {

        let placement_within_target               = placement_within_target.unwrap_or(RectanglePlacementFlags::stretchToFit);
        let fill_alpha_channel_with_current_brush = fill_alpha_channel_with_current_brush.unwrap_or(false);
        
        todo!();
        /*
            if (imageToDraw.isValid())
            drawImageTransformed (imageToDraw,
                                  placementWithinTarget.getTransformToFit (imageToDraw.getBounds().toFloat(), targetArea),
                                  fillAlphaChannelWithCurrentBrush);
        */
    }
    
    /**
      | Draws an image to fit within a designated
      | rectangle.
      | 
      | If the image is too big or too small for
      | the space, it will be rescaled to fit
      | as nicely as it can do without affecting
      | its aspect ratio. It will then be placed
      | within the target rectangle according
      | to the justification flags specified.
      | 
      | -----------
      | @param imageToDraw
      | 
      | the source image to draw
      | ----------
      | @param destX
      | 
      | top-left of the target rectangle to
      | fit it into
      | ----------
      | @param destY
      | 
      | top-left of the target rectangle to
      | fit it into
      | ----------
      | @param destWidth
      | 
      | size of the target rectangle to fit the
      | image into
      | ----------
      | @param destHeight
      | 
      | size of the target rectangle to fit the
      | image into
      | ----------
      | @param placementWithinTarget
      | 
      | this specifies how the image should
      | be positioned within the target rectangle
      | - see the RectanglePlacement class
      | for more details about this.
      | ----------
      | @param fillAlphaChannelWithCurrentBrush
      | 
      | if true, then instead of drawing the
      | image, just its alpha channel will be
      | used as a mask with which to draw with
      | the current brush or colour. This is
      | similar to fillAlphaMap(), and see
      | also drawImage() @see setImageResamplingQuality,
      | drawImage, drawImageTransformed,
      | drawImageAt, RectanglePlacement
      |
      */
    pub fn draw_image_within(
        &self, 
        image_to_draw:                         &Image,
        dx:                                    i32,
        dy:                                    i32,
        dw:                                    i32,
        dh:                                    i32,
        placement_within_target:               RectanglePlacement,
        fill_alpha_channel_with_current_brush: Option<bool>

    ) {

        let fill_alpha_channel_with_current_brush = fill_alpha_channel_with_current_brush.unwrap_or(false);
        
        todo!();
        /*
            drawImage (imageToDraw, coordsToRectangle (dx, dy, dw, dh).toFloat(),
                   placementWithinTarget, fillAlphaChannelWithCurrentBrush);
        */
    }

    /**
      | Draws part of an image, rescaling it
      | to fit in a given target region.
      | 
      | The specified area of the source image
      | is rescaled and drawn to fill the specified
      | destination rectangle.
      | 
      | Images are composited using the context's
      | current opacity, so if you don't want
      | it to be drawn semi-transparently,
      | be sure to call setOpacity (1.0f) (or
      | setColour() with an opaque colour)
      | before drawing images.
      | 
      | -----------
      | @param imageToDraw
      | 
      | the image to overlay
      | ----------
      | @param destX
      | 
      | the left of the destination rectangle
      | ----------
      | @param destY
      | 
      | the top of the destination rectangle
      | ----------
      | @param destWidth
      | 
      | the width of the destination rectangle
      | ----------
      | @param destHeight
      | 
      | the height of the destination rectangle
      | ----------
      | @param sourceX
      | 
      | the left of the rectangle to copy from
      | the source image
      | ----------
      | @param sourceY
      | 
      | the top of the rectangle to copy from
      | the source image
      | ----------
      | @param sourceWidth
      | 
      | the width of the rectangle to copy from
      | the source image
      | ----------
      | @param sourceHeight
      | 
      | the height of the rectangle to copy from
      | the source image
      | ----------
      | @param fillAlphaChannelWithCurrentBrush
      | 
      | if true, then instead of drawing the
      | source image's pixels, the source image's
      | alpha channel is used as a mask with which
      | to fill the destination using the current
      | colour or brush. (If the source is has
      | no alpha channel, then it will just fill
      | the target with a solid rectangle) @see
      | setImageResamplingQuality, drawImageAt,
      | drawImageWithin, fillAlphaMap
      |
      */
    pub fn draw_image(
        &self, 
        image_to_draw:                         &Image,
        dx:                                    i32,
        dy:                                    i32,
        dw:                                    i32,
        dh:                                    i32,
        sx:                                    i32,
        sy:                                    i32,
        sw:                                    i32,
        sh:                                    i32,
        fill_alpha_channel_with_current_brush: Option<bool>

    )  {

        let fill_alpha_channel_with_current_brush: bool = fill_alpha_channel_with_current_brush.unwrap_or(false);
        
        todo!();
        /*
            if (imageToDraw.isValid() && context.clipRegionIntersects (coordsToRectangle (dx, dy, dw, dh)))
            drawImageTransformed (imageToDraw.getClippedImage (coordsToRectangle (sx, sy, sw, sh)),
                                  AffineTransform::scale ((float) dw / (float) sw, (float) dh / (float) sh)
                                                  .translated ((float) dx, (float) dy),
                                  fillAlphaChannelWithCurrentBrush);
        */
    }
    
    /**
      | Draws an image, having applied an affine
      | transform to it.
      | 
      | This lets you throw the image around
      | in some wacky ways, rotate it, shear,
      | scale it, etc.
      | 
      | Images are composited using the context's
      | current opacity, so if you don't want
      | it to be drawn semi-transparently,
      | be sure to call setOpacity (1.0f) (or
      | setColour() with an opaque colour)
      | before drawing images.
      | 
      | If fillAlphaChannelWithCurrentBrush
      | is set to true, then the image's RGB channels
      | are ignored and it is filled with the
      | current brush, masked by its alpha channel.
      | 
      | If you want to render only a subsection
      | of an image, use Image::getClippedImage()
      | to create the section that you need.
      | 
      | @see setImageResamplingQuality,
      | drawImage
      |
      */
    pub fn draw_image_transformed(
        &self, 
        image_to_draw:                         &Image,
        transform:                             &AffineTransform,
        fill_alpha_channel_with_current_brush: Option<bool>

    ) {

        let fill_alpha_channel_with_current_brush: bool = fill_alpha_channel_with_current_brush.unwrap_or(false);
        
        todo!();
        /*
            if (imageToDraw.isValid() && ! context.isClipEmpty())
        {
            if (fillAlphaChannelWithCurrentBrush)
            {
                context.saveState();
                context.clipToImageAlpha (imageToDraw, transform);
                fillAll();
                context.restoreState();
            }
            else
            {
                context.drawImage (imageToDraw, transform);
            }
        }
        */
    }
}
