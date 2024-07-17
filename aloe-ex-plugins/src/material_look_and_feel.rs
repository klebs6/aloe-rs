crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/AUv3SynthPluginDemo.h]
pub const MATERIAL_LOOK_AND_FEEL_LABEL_FONT_SIZE:       usize = 12;
pub const MATERIAL_LOOK_AND_FEEL_BUTTON_FONT_SIZE:      usize = 15;
pub const MATERIAL_LOOK_AND_FEEL_KNOB_ACTIVE_RADIUS:    usize = 12;
pub const MATERIAL_LOOK_AND_FEEL_KNOB_IN_ACTIVE_RADIUS: usize = 8;
pub const MATERIAL_LOOK_AND_FEEL_HALO_RADIUS:           usize = 18;

pub const MATERIAL_LOOK_AND_FEEL_WINDOW_BACKGROUND_COLOUR: Colour = Colour::new_from_argb(0xff262328);
pub const MATERIAL_LOOK_AND_FEEL_BACKGROUND_COLOUR:        Colour = Colour::new_from_argb(0xff4d4d4d);
pub const MATERIAL_LOOK_AND_FEEL_BRIGHT_BUTTON_COLOUR:     Colour = Colour::new_from_argb(0xff80cbc4);
pub const MATERIAL_LOOK_AND_FEEL_DISABLED_BUTTON_COLOUR:   Colour = Colour::new_from_argb(0xffe4e4e4);
pub const MATERIAL_LOOK_AND_FEEL_SLIDER_INACTIVE_PART:     Colour = Colour::new_from_argb(0xff545d62);
pub const MATERIAL_LOOK_AND_FEEL_SLIDER_ACTIVE_PART:       Colour = Colour::new_from_argb(0xff80cbc4);

pub struct MaterialLookAndFeel<'a> {
    base: LookAndFeel_V4<'a>,
}

impl<'a> Default for MaterialLookAndFeel<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setColour (ResizableWindow::backgroundColourId, windowBackgroundColour);
            setColour (TextButton::buttonOnColourId,        brightButtonColour);
            setColour (TextButton::buttonColourId,          disabledButtonColour)
        */
    }
}

impl<'a> MaterialLookAndFeel<'a> {
    
    pub fn draw_button_background(&mut self, 
        g:                    &mut Graphics,
        button:               &mut Button,
        background_colour:    &Colour,
        is_mouse_over_button: bool,
        is_button_down:       bool)  {
        
        todo!();
        /*
            auto buttonRect = button.getLocalBounds().toFloat();

            if (isButtonDown)
                g.setColour (brightButtonColour.withAlpha (0.7f));
            else if (! button.isEnabled())
                g.setColour (disabledButtonColour);
            else
                g.setColour (brightButtonColour);

            g.fillRoundedRectangle (buttonRect, 5.0f);
        */
    }
    
    pub fn draw_button_text(&mut self, 
        g:                    &mut Graphics,
        button:               &mut TextButton,
        is_mouse_over_button: bool,
        is_button_down:       bool)  {
        
        todo!();
        /*
            ignoreUnused (isMouseOverButton, isButtonDown);

            Font font (getTextButtonFont (button, button.getHeight()));
            g.setFont (font);

            if (button.isEnabled())
                g.setColour (Colours::white);
            else
                g.setColour (backgroundColour);

            g.drawFittedText (button.getButtonText(), 0, 0,
                              button.getWidth(),
                              button.getHeight(),
                              Justification::centred, 2);
        */
    }
    
    pub fn draw_linear_slider(&mut self, 
        g:              &mut Graphics,
        x:              i32,
        y:              i32,
        width:          i32,
        height:         i32,
        slider_pos:     f32,
        min_slider_pos: f32,
        max_slider_pos: f32,
        style:          SliderStyle,
        slider:         &mut Slider)  {
        
        todo!();
        /*
            ignoreUnused (style, minSliderPos, maxSliderPos);

            auto r = Rectangle<int> (x + haloRadius, y, width - (haloRadius * 2), height);
            auto backgroundBar = r.withSizeKeepingCentre(r.getWidth(), 2);

            sliderPos = (sliderPos - minSliderPos) / static_cast<float> (width);

            auto knobPos = static_cast<int> (sliderPos * (float) r.getWidth());

            g.setColour (sliderActivePart);
            g.fillRect (backgroundBar.removeFromLeft (knobPos));

            g.setColour (sliderInactivePart);
            g.fillRect (backgroundBar);

            if (slider.isMouseOverOrDragging())
            {
                auto haloBounds = r.withTrimmedLeft (knobPos - haloRadius)
                                   .withWidth (haloRadius * 2)
                                   .withSizeKeepingCentre (haloRadius * 2, haloRadius * 2);

                g.setColour (sliderActivePart.withAlpha (0.5f));
                g.fillEllipse (haloBounds.toFloat());
            }

            auto knobRadius = slider.isMouseOverOrDragging() ? knobActiveRadius : knobInActiveRadius;
            auto knobBounds = r.withTrimmedLeft (knobPos - knobRadius)
                               .withWidth (knobRadius * 2)
                               .withSizeKeepingCentre (knobRadius * 2, knobRadius * 2);

            g.setColour (sliderActivePart);
            g.fillEllipse (knobBounds.toFloat());
        */
    }
    
    pub fn get_text_button_font(&mut self, 
        button:        &mut TextButton,
        button_height: i32) -> Font {
        
        todo!();
        /*
            return LookAndFeel_V3::getTextButtonFont (button, buttonHeight).withHeight (buttonFontSize);
        */
    }
    
    pub fn get_label_font(&mut self, label: &mut Label) -> Font {
        
        todo!();
        /*
            return LookAndFeel_V3::getLabelFont (label).withHeight (labelFontSize);
        */
    }
}
