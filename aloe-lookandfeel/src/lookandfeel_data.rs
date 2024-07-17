crate::ix!();

#[no_copy]
#[leak_detector]
#[weak_referenceable]
pub struct LookAndFeelData {
    colours:                  SortedSet<LookAndFeelColourSetting>,
    default_sans:             String,
    default_serif:            String,
    default_fixed:            String,
    default_typeface:         TypefacePtr,
    use_native_alert_windows: bool, // default = false
}

pub trait HasLookAndFeelData {

    type LookAndFeelData;

    fn get_look_and_feel_data(&self) -> &Self::LookAndFeelData;
}

impl Drop for LookAndFeelData {

    fn drop(&mut self) {
        todo!();
        /* 
        /* This assertion is triggered if you try to delete a LookAndFeel object while something
           is still using it!

           Reasons may be:
             - it's still being used as the default LookAndFeel; or
             - it's set as a Component's current lookandfeel; or
             - there's a WeakReference to it somewhere else in your code

           Generally the fix for this will be to make sure you call
           Component::setLookandFeel (nullptr) on any components that were still using
           it before you delete it, or call LookAndFeel::setDefaultLookAndFeel (nullptr)
           if you had set it up to be the default one. This assertion can also be avoided by
           declaring your LookAndFeel object before any of the Components that use it as
           the Components will be destroyed before the LookAndFeel.

           Deleting a LookAndFeel is unlikely to cause a crash since most things will use a
           safe WeakReference to it, but it could cause some unexpected graphical behaviour,
           so it's advisable to clear up any references before destroying them!
        */
        jassert (masterReference.getNumActiveWeakReferences() == 0
                  || (masterReference.getNumActiveWeakReferences() == 1
                       && this == &getDefaultLookAndFeel()));
 */
    }
}

impl Default for LookAndFeelData {
    
    /**
      | Creates the default Aloe look and feel.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*


            /* if this fails it means you're trying to create a LookAndFeel object before
           the static Colours have been initialised. That ain't gonna work. It probably
           means that you're using a static LookAndFeel object and that your compiler has
           decided to initialise it before the Colours class.
        */
        jassert (Colours::white == Colour (0xffffffff));

        aloe_getTypefaceForFont = getTypefaceForFontFromLookAndFeel;
        */
    }
}
    
impl FindColour for LookAndFeelData {

    /**
      | Looks for a colour that has been registered
      | with the given colour ID number.
      | 
      | If a colour has been set for this ID number
      | using setColour(), then it is returned.
      | If none has been set, it will just return
      | Colours::black.
      | 
      | The colour IDs for various purposes
      | are stored as enums in the components
      | that they are relevant to - for an example,
      | see Slider::ColourIds,
      | 
      | Label::ColourIds, TextEditor::ColourIds,
      | TreeView::ColourIds, etc.
      | 
      | If you're looking up a colour for use
      | in drawing a component, it's usually
      | best not to call this directly, but to
      | use the Component::findColour() method
      | instead. That will first check whether
      | a suitable colour has been registered
      | directly with the component, and will
      | fall-back on calling the component's
      | 
      | LookAndFeel's findColour() method
      | if none is found.
      | 
      | @see setColour, Component::findColour,
      | Component::setColour
      |
      */
    fn find_colour(&self, colourid: i32) -> Colour {
        
        todo!();
        /*
            const LookAndFeelColourSetting c = { colourID, Colour() };
        auto index = colours.indexOf (c);

        if (index >= 0)
            return colours[index].colour;

        jassertfalse;
        return Colours::black;
        */
    }
}
    
impl SetColour for LookAndFeelData {

    /**
      | Registers a colour to be used for a particular
      | purpose.
      | 
      | For more details, see the comments for
      | findColour(). @see findColour, Component::findColour,
      | Component::setColour
      |
      */
    fn set_colour(
        &mut self, 
        colourid:   i32,
        new_colour: Colour

    ) {
        
        todo!();
        /*
            const LookAndFeelColourSetting c = { colourID, newColour };
        auto index = colours.indexOf (c);

        if (index >= 0)
            colours.getReference (index).colour = newColour;
        else
            colours.add (c);
        */
    }
}
    
impl IsColourSpecified for LookAndFeelData {

    /**
      | Returns true if the specified colour
      | ID has been explicitly set using the
      | setColour() method.
      |
      */
    fn is_colour_specified(&self, colourid: i32) -> bool {
        
        todo!();
        /*
            const LookAndFeelColourSetting c = { colourID, Colour() };
        return colours.contains (c);
        */
    }
}

impl LookAndFeelData {
    
    /**
      | Returns the current default look-and-feel
      | for a component to use when it hasn't
      | got one explicitly set.
      | 
      | @see setDefaultLookAndFeel
      |
      */
    pub fn get_default_look_and_feel(&mut self) -> &mut dyn LookAndFeel {
        
        todo!();
        /*
            return Desktop::getInstance().getDefaultLookAndFeel();
        */
    }
    
    /**
      | Changes the default look-and-feel.
      | 
      | -----------
      | @param newDefaultLookAndFeel
      | 
      | the new look-and-feel object to use
      | - if this is set to null, it will revert
      | to using the default one. The object
      | passed-in must be deleted by the caller
      | when it's no longer needed. @see getDefaultLookAndFeel
      |
      */
    pub fn set_default_look_and_feel(&mut self, new_default_look_and_feel: *mut dyn LookAndFeel)  {
        
        todo!();
        /*
            Desktop::getInstance().setDefaultLookAndFeel (newDefaultLookAndFeel);
        */
    }
    
    pub fn get_typeface_for_font(&mut self, font: &Font) -> TypefacePtr {
        
        todo!();
        /*
            if (font.getTypefaceName() == Font::getDefaultSansSerifFontName())
        {
            if (defaultTypeface != nullptr)
                return defaultTypeface;

            if (defaultSans.isNotEmpty())
            {
                Font f (font);
                f.setTypefaceName (defaultSans);
                return Typeface::createSystemTypefaceFor (f);
            }
        }

        return Font::getDefaultTypefaceForFont (font);
        */
    }
    
    /**
      | Allows you to supply a default typeface
      | that will be returned as the default
      | sans-serif font.
      | 
      | Instead of a typeface object, you can
      | specify a typeface by name using the
      | setDefaultSansSerifTypefaceName()
      | method.
      | 
      | You can perform more complex typeface
      | substitutions by overloading getTypefaceForFont()
      | but this lets you easily set a global
      | typeface.
      |
      */
    pub fn set_default_sans_serif_typeface(&mut self, new_default_typeface: TypefacePtr)  {
        
        todo!();
        /*
            if (defaultTypeface != newDefaultTypeface)
        {
            defaultTypeface = newDefaultTypeface;
            Typeface::clearTypefaceCache();
        }
        */
    }
    
    /**
      | Allows you to change the default sans-serif
      | font.
      | 
      | If you need to supply your own Typeface
      | object for any of the default fonts,
      | rather than just supplying the name
      | (e.g. if you want to use an embedded font),
      | then you can instead call setDefaultSansSerifTypeface()
      | with an object to use.
      |
      */
    pub fn set_default_sans_serif_typeface_name(&mut self, new_name: &String)  {
        
        todo!();
        /*
            if (defaultSans != newName)
        {
            defaultTypeface.reset();
            Typeface::clearTypefaceCache();
            defaultSans = newName;
        }
        */
    }
    
    pub fn get_mouse_cursor_for(&mut self, component: &mut Component) -> MouseCursor {
        
        todo!();
        /*
            auto cursor = component.getMouseCursor();

        for (auto* parent = component.getParentComponent();
             parent != nullptr && cursor == MouseCursor::ParentCursor;
             parent = parent->getParentComponent())
        {
            cursor = parent->getMouseCursor();
        }

        return cursor;
        */
    }
    
    pub fn create_graphics_context(
        &mut self, 
        image_to_render_on: &Image,
        origin:             Point<i32>,
        initial_clip:       &RectangleList<i32>

    ) -> Box<dyn LowLevelGraphicsContext> {
        
        todo!();
        /*
            return std::make_unique<LowLevelGraphicsSoftwareRenderer> (imageToRenderOn, origin, initialClip);
        */
    }
    
    pub fn set_using_native_alert_windows(&mut self, should_use_native_alerts: bool)  {
        
        todo!();
        /*
            useNativeAlertWindows = shouldUseNativeAlerts;
        */
    }
    
    pub fn is_using_native_alert_windows(&mut self) -> bool {
        
        todo!();
        /*
            #if ALOE_LINUX || ALOE_BSD
        return false; // not available currently..
       #else
        return useNativeAlertWindows;
       #endif
        */
    }
}
