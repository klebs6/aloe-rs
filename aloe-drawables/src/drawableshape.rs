crate::ix!();

pub fn replace_colour_in_fill(
        fill:        &mut FillType,
        original:    Colour,
        replacement: Colour) -> bool {
    
    todo!();
    /*
        if (fill.colour == original && fill.isColour())
        {
            fill = FillType (replacement);
            return true;
        }

        return false;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawableShape.h]

/**
  | A base class implementing common functionality
  | for Drawable classes which consist
  | of some kind of filled and stroked outline.
  | 
  | @see DrawablePath, DrawableRectangle
  | 
  | @tags{GUI}
  |
  */
pub struct DrawableShape<'a> {
    base:         Drawable<'a>,
    stroke_type:  PathStrokeType,
    dash_lengths: Vec<f32>,
    path:         aloe_geometry::Path,
    stroke_path:  aloe_geometry::Path,
    main_fill:    FillType,
    stroke_fill:  FillType,
}

impl<'a> Default for DrawableShape<'a> {

    fn default() -> Self {
    
        todo!();
        /*
        : stroke_type(0.0f),
        : main_fill(Colours::black),
        : stroke_fill(Colours::black),

        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawableShape.cpp]
impl<'a> DrawableShape<'a> {
    
    /**
      | Returns the current fill type. @see
      | setFill
      |
      */
    pub fn get_fill(&self) -> &FillType {
        
        todo!();
        /*
            return mainFill;
        */
    }

    /**
      | Returns the current stroke fill. @see
      | setStrokeFill
      |
      */
    pub fn get_stroke_fill(&self) -> &FillType {
        
        todo!();
        /*
            return strokeFill;
        */
    }

    /**
      | Returns the current outline style.
      |
      */
    pub fn get_stroke_type(&self) -> &PathStrokeType {
        
        todo!();
        /*
            return strokeType;
        */
    }

    /**
      | Returns the set of dash lengths that
      | the path is using.
      |
      */
    pub fn get_dash_lengths(&self) -> &[f32] {
        
        todo!();
        /*
            return dashLengths;
        */
    }
    
    pub fn new(other: &DrawableShape) -> Self {
    
        todo!();
        /*
        : drawable(other),
        : stroke_type(other.strokeType),
        : dash_lengths(other.dashLengths),
        : main_fill(other.mainFill),
        : stroke_fill(other.strokeFill),

        
        */
    }
    
    /**
      | Sets a fill type for the path.
      | 
      | This colour is used to fill the path -
      | if you don't want the path to be filled
      | (e.g. if you're just drawing an outline),
      | set this to a transparent colour.
      | 
      | @see setPath, setStrokeFill
      |
      */
    pub fn set_fill(&mut self, new_fill: &FillType)  {
        
        todo!();
        /*
            if (mainFill != newFill)
        {
            mainFill = newFill;
            repaint();
        }
        */
    }
    
    /**
      | Sets the fill type with which the outline
      | will be drawn. @see setFill
      |
      */
    pub fn set_stroke_fill(&mut self, new_fill: &FillType)  {
        
        todo!();
        /*
            if (strokeFill != newFill)
        {
            strokeFill = newFill;
            repaint();
        }
        */
    }
    
    /**
      | Changes the properties of the outline
      | that will be drawn around the path.
      | 
      | If the stroke has 0 thickness, no stroke
      | will be drawn. @see setStrokeThickness,
      | setStrokeColour
      |
      */
    pub fn set_stroke_type(&mut self, new_stroke_type: &PathStrokeType)  {
        
        todo!();
        /*
            if (strokeType != newStrokeType)
        {
            strokeType = newStrokeType;
            strokeChanged();
        }
        */
    }
    
    /**
      | Provides a set of dash lengths to use
      | for stroking the path.
      |
      */
    pub fn set_dash_lengths(&mut self, new_dash_lengths: &[f32])  {
        
        todo!();
        /*
            if (dashLengths != newDashLengths)
        {
            dashLengths = newDashLengths;
            strokeChanged();
        }
        */
    }
    
    /**
      | Changes the stroke thickness.
      | 
      | This is a shortcut for calling setStrokeType.
      |
      */
    pub fn set_stroke_thickness(&mut self, new_thickness: f32)  {
        
        todo!();
        /*
            setStrokeType (PathStrokeType (newThickness, strokeType.getJointStyle(), strokeType.getEndStyle()));
        */
    }
    
    /**
      | True if there's a stroke with a non-zero
      | thickness and non-transparent colour.
      |
      */
    pub fn is_stroke_visible(&self) -> bool {
        
        todo!();
        /*
            return strokeType.getStrokeThickness() > 0.0f && ! strokeFill.isInvisible();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            transformContextToCorrectOrigin (g);
        applyDrawableClipPath (g);

        g.setFillType (mainFill);
        g.fillPath (path);

        if (isStrokeVisible())
        {
            g.setFillType (strokeFill);
            g.fillPath (strokePath);
        }
        */
    }
    
    /**
      | Called when the cached path should be
      | updated.
      |
      */
    pub fn path_changed(&mut self)  {
        
        todo!();
        /*
            strokeChanged();
        */
    }
    
    /**
      | Called when the cached stroke should
      | be updated.
      |
      */
    pub fn stroke_changed(&mut self)  {
        
        todo!();
        /*
            strokePath.clear();
        const float extraAccuracy = 4.0f;

        if (dashLengths.isEmpty())
            strokeType.createStrokedPath (strokePath, path, AffineTransform(), extraAccuracy);
        else
            strokeType.createDashedStroke (strokePath, path, dashLengths.getRawDataPointer(),
                                           dashLengths.size(), AffineTransform(), extraAccuracy);

        setBoundsToEnclose (getDrawableBounds());
        repaint();
        */
    }
    
    pub fn get_drawable_bounds(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            if (isStrokeVisible())
            return strokePath.getBounds();

        return path.getBounds();
        */
    }
    
    pub fn hit_test(&mut self, x: i32, y: i32) -> bool {
        
        todo!();
        /*
            bool allowsClicksOnThisComponent, allowsClicksOnChildComponents;
        getInterceptsMouseClicks (allowsClicksOnThisComponent, allowsClicksOnChildComponents);

        if (! allowsClicksOnThisComponent)
            return false;

        auto globalX = (float) (x - originRelativeToComponent.x);
        auto globalY = (float) (y - originRelativeToComponent.y);

        return path.contains (globalX, globalY)
                || (isStrokeVisible() && strokePath.contains (globalX, globalY));
        */
    }
    
    pub fn replace_colour(&mut self, 
        original:    Colour,
        replacement: Colour) -> bool {
        
        todo!();
        /*
            bool changed1 = replaceColourInFill (mainFill,   original, replacement);
        bool changed2 = replaceColourInFill (strokeFill, original, replacement);
        return changed1 || changed2;
        */
    }
    
    pub fn get_outline_as_path(&self) -> PathBuf {
        
        todo!();
        /*
            auto outline = isStrokeVisible() ? strokePath : path;
        outline.applyTransform (getTransform());
        return outline;
        */
    }
}
