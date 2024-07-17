crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/colour/aloe_ColourGradient.h]

pub struct ColourPoint
{
    position: f64,
    colour:   Colour,
}

impl PartialEq<ColourPoint> for ColourPoint {
    
    #[inline] fn eq(&self, other: &ColourPoint) -> bool {
        todo!();
        /*
           return position == other.position && colour == other.colour;
        */
    }
}

impl Eq for ColourPoint {}

/**
  | Describes the layout and colours that
  | should be used to paint a colour gradient.
  | 
  | @see Graphics::setGradientFill
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct ColourGradient {
    point1:    Point<f32>,
    point2:    Point<f32>,

    /**
      | If true, the gradient should be filled
      | circularly, centred around point1, with
      | point2 defining a point on the
      | circumference. 
      |
      | If false, the gradient is linear between
      | the two points.
      */
    is_radial: bool,

    colours:   Vec<ColourPoint>,
}

impl PartialEq<ColourGradient> for ColourGradient {
    
    #[inline] fn eq(&self, other: &ColourGradient) -> bool {
        todo!();
        /*
            return point1 == other.point1 && point2 == other.point2
                && isRadial == other.isRadial
                && colours == other.colours;
        */
    }
}

impl Eq for ColourGradient {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/colour/aloe_ColourGradient.cpp]
impl Default for ColourGradient {

    /**
      | Creates an uninitialised gradient.
      | 
      | If you use this constructor instead
      | of the other one, be sure to set all the
      | object's public member variables before
      | using it!
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : is_radial(false),

            #if ALOE_DEBUG
        point1.setX (987654.0f);
        #define ALOE_COLOURGRADIENT_CHECK_COORDS_INITIALISED   jassert (point1.x != 987654.0f);
       #else
        #define ALOE_COLOURGRADIENT_CHECK_COORDS_INITIALISED
       #endif
        */
    }
}
    
impl ColourGradient {

    /**
      | Creates a vertical linear gradient
      | from top to bottom in a rectangle
      |
      */
    pub fn vertical<Type: Copy + Clone>(
        colour_top:    Colour,
        colour_bottom: Colour,
        area:          Rectangle<Type>) -> ColourGradient {
    
        todo!();
        /*
            return vertical (colourTop, (float) area.getY(), colourBottom, (float) area.getBottom());
        */
    }

    /**
      | Creates a horizontal linear gradient
      | from right to left in a rectangle
      |
      */
    pub fn horizontal<Type: Copy + Clone>(
        colour_left:  Colour,
        colour_right: Colour,
        area:         Rectangle<Type>) -> ColourGradient {
    
        todo!();
        /*
            return horizontal (colourLeft, (float) area.getX(), colourRight, (float) area.getRight());
        */
    }

    pub fn new_from_ref(other: &ColourGradient) -> Self {
    
        todo!();
        /*
        : point1(other.point1),
        : point2(other.point2),
        : is_radial(other.isRadial),
        : colours(other.colours),
        */
    }
    
    pub fn new_from_other(other: ColourGradient) -> Self {
    
        todo!();
        /*
        : point1(other.point1),
        : point2(other.point2),
        : is_radial(other.isRadial),
        : colours(std::move (other.colours)),

        
        */
    }
    
    pub fn assign_from_ref(&mut self, other: &ColourGradient) -> &mut ColourGradient {
        
        todo!();
        /*
            point1 = other.point1;
        point2 = other.point2;
        isRadial = other.isRadial;
        colours = other.colours;
        return *this;
        */
    }
    
    pub fn assign_from(&mut self, other: ColourGradient) -> &mut ColourGradient {
        
        todo!();
        /*
            point1 = other.point1;
        point2 = other.point2;
        isRadial = other.isRadial;
        colours = std::move (other.colours);
        return *this;
        */
    }
    
    /**
      | Creates a gradient object.
      | 
      | (x1, y1) is the location to draw with
      | colour1. Likewise (x2, y2) is where
      | colour2 should be. In between them there's
      | a gradient.
      | 
      | If isRadial is true, the colours form
      | a circular gradient with (x1, y1) at
      | its centre.
      | 
      | The alpha transparencies of the colours
      | are used, so note that if you blend from
      | transparent to a solid colour, the RGB
      | of the transparent colour will become
      | visible in parts of the gradient. e.g.
      | blending from Colour::transparentBlack
      | to Colours::white will produce a muddy
      | grey colour midway, but Colour::transparentWhite
      | to Colours::white will be white all
      | the way across.
      | 
      | @see ColourGradient
      |
      */
    pub fn new_with_xy(
        colour1: Colour,
        x1:      f32,
        y1:      f32,
        colour2: Colour,
        x2:      f32,
        y2:      f32,
        radial:  bool) -> Self {
    
        todo!();
        /*

            : ColourGradient (colour1, Point<float> (x1, y1),
                          colour2, Point<float> (x2, y2), radial)
        */
    }
    
    /**
      | Creates a gradient object.
      | 
      | point1 is the location to draw with colour1.
      | Likewise point2 is where colour2 should
      | be. In between them there's a gradient.
      | 
      | If isRadial is true, the colours form
      | a circular gradient with point1 at its
      | centre.
      | 
      | The alpha transparencies of the colours
      | are used, so note that if you blend from
      | transparent to a solid colour, the RGB
      | of the transparent colour will become
      | visible in parts of the gradient. e.g.
      | blending from Colour::transparentBlack
      | to Colours::white will produce a muddy
      | grey colour midway, but Colour::transparentWhite
      | to Colours::white will be white all
      | the way across.
      | 
      | @see ColourGradient
      |
      */
    pub fn new(
        colour1: Colour,
        p1:      Point<f32>,
        colour2: Colour,
        p2:      Point<f32>,
        radial:  bool) -> Self {
    
        todo!();
        /*

            : point1 (p1),
          point2 (p2),
          isRadial (radial)

        colours.add (ColourPoint { 0.0, colour1 },
                     ColourPoint { 1.0, colour2 });
        */
    }
    
    /**
      | Creates a vertical linear gradient
      | between two Y coordinates
      |
      */
    pub fn vertical_between_two_y_coords(
        &mut self, 
        c1: Colour,
        y1: f32,
        c2: Colour,
        y2: f32) -> ColourGradient {
        
        todo!();
        /*
            return { c1, 0, y1, c2, 0, y2, false };
        */
    }
    
    /**
      | Creates a horizontal linear gradient
      | between two X coordinates
      |
      */
    pub fn horizontal_between_two_x_coords(
        &mut self, 
        c1: Colour,
        x1: f32,
        c2: Colour,
        x2: f32) -> ColourGradient {
        
        todo!();
        /*
            return { c1, x1, 0, c2, x2, 0, false };
        */
    }
    
    /**
      | Removes any colours that have been added.
      | 
      | This will also remove any start and end
      | colours, so the gradient won't work.
      | You'll need to add more colours with
      | addColour().
      |
      */
    pub fn clear_colours(&mut self)  {
        
        todo!();
        /*
            colours.clear();
        */
    }
    
    /**
      | Adds a colour at a point along the length
      | of the gradient.
      | 
      | This allows the gradient to go through
      | a spectrum of colours, instead of just
      | a start and end colour.
      | 
      | -----------
      | @param proportionAlongGradient
      | 
      | a value between 0 and 1.0, which is the
      | proportion of the distance along the
      | line between the two points at which
      | the colour should occur.
      | ----------
      | @param colour
      | 
      | the colour that should be used at this
      | point
      | 
      | -----------
      | @return
      | 
      | the index at which the new point was added
      |
      */
    pub fn add_colour(
        &mut self, 
        proportion_along_gradient: f64,
        colour:                    Colour) -> i32 {
        
        todo!();
        /*
            // must be within the two end-points
        jassert (proportionAlongGradient >= 0 && proportionAlongGradient <= 1.0);

        if (proportionAlongGradient <= 0)
        {
            colours.set (0, { 0.0, colour });
            return 0;
        }

        auto pos = jmin (1.0, proportionAlongGradient);

        int i;
        for (i = 0; i < colours.size(); ++i)
            if (colours.getReference(i).position > pos)
                break;

        colours.insert (i, { pos, colour });
        return i;
        */
    }
    
    /**
      | Removes one of the colours from the gradient.
      |
      */
    pub fn remove_colour(&mut self, index: i32)  {
        
        todo!();
        /*
            jassert (index > 0 && index < colours.size() - 1);
        colours.remove (index);
        */
    }
    
    /**
      | Multiplies the alpha value of all the
      | colours by the given scale factor
      |
      */
    pub fn multiply_opacity(&mut self, multiplier: f32)  {
        
        todo!();
        /*
            for (auto& c : colours)
            c.colour = c.colour.withMultipliedAlpha (multiplier);
        */
    }
    
    /**
      | Returns the number of colour-stops
      | that have been added.
      |
      */
    pub fn get_num_colours(&self) -> i32 {
        
        todo!();
        /*
            return colours.size();
        */
    }
    
    /**
      | Returns the position along the length
      | of the gradient of the colour with this
      | index.
      | 
      | The index is from 0 to getNumColours()
      | - 1. The return value will be between
      | 0.0 and 1.0
      |
      */
    pub fn get_colour_position(&self, index: i32) -> f64 {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, colours.size()))
            return colours.getReference (index).position;

        return 0;
        */
    }
    
    /**
      | Returns the colour that was added with
      | a given index.
      | 
      | The index is from 0 to getNumColours()
      | - 1.
      |
      */
    pub fn get_colour(&self, index: i32) -> Colour {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, colours.size()))
            return colours.getReference (index).colour;

        return {};
        */
    }
    
    /**
      | Changes the colour at a given index.
      | 
      | The index is from 0 to getNumColours()
      | - 1.
      |
      */
    pub fn set_colour(
        &mut self, 
        index:      i32,
        new_colour: Colour)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, colours.size()))
            colours.getReference (index).colour = newColour;
        */
    }
    
    /**
      | Returns the an interpolated colour
      | at any position along the gradient.
      | 
      | -----------
      | @param position
      | 
      | the position along the gradient, between
      | 0 and 1
      |
      */
    pub fn get_colour_at_position(&self, position: f64) -> Colour {
        
        todo!();
        /*
            jassert (colours.getReference(0).position == 0.0); // the first colour specified has to go at position 0

        if (position <= 0 || colours.size() <= 1)
            return colours.getReference(0).colour;

        int i = colours.size() - 1;
        while (position < colours.getReference(i).position)
            --i;

        auto& p1 = colours.getReference (i);

        if (i >= colours.size() - 1)
            return p1.colour;

        auto& p2 = colours.getReference (i + 1);

        return p1.colour.interpolatedWith (p2.colour, (float) ((position - p1.position) / (p2.position - p1.position)));
        */
    }
    
    /**
      | Creates a set of interpolated premultiplied
      | ARGB values.
      | 
      | This will fill an array of a user-specified
      | size with the gradient, interpolating
      | to fit.
      | 
      | The numEntries argument specifies
      | the size of the array, and this size must
      | be greater than zero.
      | 
      | When calling this, the ColourGradient
      | must have at least 2 colour stops specified.
      |
      */
    pub fn create_lookup_table(&self, 
        lookup_table: *mut PixelARGB,
        num_entries:  i32)  {
        
        todo!();
        /*
            ALOE_COLOURGRADIENT_CHECK_COORDS_INITIALISED // Trying to use this object without setting its coordinates?
        jassert (colours.size() >= 2);
        jassert (numEntries > 0);
        jassert (colours.getReference(0).position == 0.0); // The first colour specified has to go at position 0

        auto pix1 = colours.getReference (0).colour.getPixelARGB();
        int index = 0;

        for (int j = 1; j < colours.size(); ++j)
        {
            auto& p = colours.getReference (j);
            auto numToDo = roundToInt (p.position * (numEntries - 1)) - index;
            auto pix2 = p.colour.getPixelARGB();

            for (int i = 0; i < numToDo; ++i)
            {
                jassert (index >= 0 && index < numEntries);

                lookupTable[index] = pix1;
                lookupTable[index].tween (pix2, (uint32) ((i << 8) / numToDo));
                ++index;
            }

            pix1 = pix2;
        }

        while (index < numEntries)
            lookupTable [index++] = pix1;
        */
    }
    
    /**
      | Creates a set of interpolated premultiplied
      | ARGB values.
      | 
      | This will resize the HeapBlock, fill
      | it with the colours, and will return
      | the number of colours that it added.
      | 
      | When calling this, the ColourGradient
      | must have at least 2 colour stops specified.
      |
      */
    pub fn create_lookup_table_with_heap_block(
        &self, 
        transform:    &AffineTransform,
        lookup_table: &mut HeapBlock<PixelARGB>) -> i32 {
        
        todo!();
        /*
            ALOE_COLOURGRADIENT_CHECK_COORDS_INITIALISED // Trying to use this object without setting its coordinates?
        jassert (colours.size() >= 2);

        auto numEntries = jlimit (1, jmax (1, (colours.size() - 1) << 8),
                                  3 * (int) point1.transformedBy (transform)
                                                  .getDistanceFrom (point2.transformedBy (transform)));
        lookupTable.malloc (numEntries);
        createLookupTable (lookupTable, numEntries);
        return numEntries;
        */
    }
    
    /**
      | Returns true if all colours are opaque.
      |
      */
    pub fn is_opaque(&self) -> bool {
        
        todo!();
        /*
            for (auto& c : colours)
            if (! c.colour.isOpaque())
                return false;

        return true;
        */
    }
    
    /**
      | Returns true if all colours are completely
      | transparent.
      |
      */
    pub fn is_invisible(&self) -> bool {
        
        todo!();
        /*
            for (auto& c : colours)
            if (! c.colour.isTransparent())
                return false;

        return true;
        */
    }
}
