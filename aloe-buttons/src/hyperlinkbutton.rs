crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_HyperlinkButton.h]

/**
  | A button showing an underlined weblink,
  | that will launch the link when it's clicked.
  | 
  | @see Button
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct HyperlinkButton<'a> {
    base:          Button<'a>,
    url:           Url,
    font:          Font,
    resize_font:   bool,
    justification: Justification,
}

impl<'a> Default for HyperlinkButton<'a> {
    
    /**
      | Creates a HyperlinkButton.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : button(String()),
        : font(14.0f, Font::underlined),
        : resize_font(true),
        : justification(Justification::centred),

            setMouseCursor (MouseCursor::PointingHandCursor);
        */
    }
    
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_HyperlinkButton.cpp]
impl<'a> HyperlinkButton<'a> {

    /**
      | Returns the Url that the button will
      | trigger.
      |
      */
    pub fn geturl(&self) -> &Url {
        
        todo!();
        /*
            return url;
        */
    }

    /**
      | Returns the type of justification,
      | as set in setJustificationType().
      |
      */
    pub fn get_justification_type(&self) -> Justification {
        
        todo!();
        /*
            return justification;
        */
    }
    
    /**
      | Creates a HyperlinkButton.
      | 
      | -----------
      | @param linkText
      | 
      | the text that will be displayed in the
      | button - this is also set as the Component's
      | name, but the text can be changed later
      | with the Button::setButtonText()
      | method
      | ----------
      | @param linkURL
      | 
      | the Url to launch when the user clicks
      | the button
      |
      */
    pub fn new(
        link_text: &String,
        linkurl:   &Url) -> Self {
    
        todo!();
        /*
        : button(linkText),
        : url(linkURL),
        : font(14.0f, Font::underlined),
        : resize_font(true),
        : justification(Justification::centred),

            setMouseCursor (MouseCursor::PointingHandCursor);
        setTooltip (linkURL.toString (false));
        */
    }
    
    /**
      | Changes the font to use for the text.
      | 
      | If resizeToMatchComponentHeight
      | is true, the font's height will be adjusted
      | to match the size of the component.
      |
      */
    pub fn set_font(
        &mut self, 
        new_font:                         &Font,
        resize_to_match_component_height: bool,
        justification_type:               Option<JustificationFlags>

    ) {

        let justification_type: JustificationFlags = justification_type.unwrap_or(JustificationFlags::horizontallyCentred);
        
        todo!();
        /*
            font = newFont;
        resizeFont = resizeToMatchComponentHeight;
        justification = justificationType;
        repaint();
        */
    }
    
    /**
      | Changes the Url that the button will
      | trigger.
      |
      */
    pub fn seturl(&mut self, newurl: &Url)  {
        
        todo!();
        /*
            url = newURL;
        setTooltip (newURL.toString (false));
        */
    }
    
    pub fn get_font_to_use(&self) -> Font {
        
        todo!();
        /*
            if (resizeFont)
            return font.withHeight ((float) getHeight() * 0.7f);

        return font;
        */
    }
    
    /**
      | Resizes the button horizontally to
      | fit snugly around the text.
      | 
      | This won't affect the button's height.
      |
      */
    pub fn change_width_to_fit_text(&mut self)  {
        
        todo!();
        /*
            setSize (getFontToUse().getStringWidth (getButtonText()) + 6, getHeight());
        */
    }
    
    /**
      | Sets the style of justification to be
      | used for positioning the text. (The
      | default is Justification::centred)
      |
      */
    pub fn set_justification_type(&mut self, new_justification: Justification)  {
        
        todo!();
        /*
            if (justification != newJustification)
        {
            justification = newJustification;
            repaint();
        }
        */
    }
    
    pub fn colour_changed(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn clicked(&mut self)  {
        
        todo!();
        /*
            if (url.isWellFormed())
            url.launchInDefaultBrowser();
        */
    }
    
    pub fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            const Colour textColour (findColour (textColourId));

        if (isEnabled())
            g.setColour ((shouldDrawButtonAsHighlighted) ? textColour.darker ((shouldDrawButtonAsDown) ? 1.3f : 0.4f)
                                             : textColour);
        else
            g.setColour (textColour.withMultipliedAlpha (0.4f));

        g.setFont (getFontToUse());

        g.drawText (getButtonText(), getLocalBounds().reduced (1, 0),
                    justification.getOnlyHorizontalFlags() | Justification::verticallyCentred,
                    true);
        */
    }
}
