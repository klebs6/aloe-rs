crate::ix!();

/**
  | Another really simple look and feel
  | that is very flat and square.
  | 
  | This inherits from CustomLookAndFeel
  | above for the linear bar and slider backgrounds.
  |
  */
pub struct SquareLookAndFeel<'a> {
    base: CustomLookAndFeel<'a>,
}

impl<'a> SquareLookAndFeel<'a> {
    
    pub fn draw_button_background(&mut self, 
        g:                    &mut Graphics,
        button:               &mut Button,
        background_colour:    &Colour,
        is_mouse_over_button: bool,
        is_button_down:       bool)  {
        
        todo!();
        /*
            auto baseColour = backgroundColour.withMultipliedSaturation (button.hasKeyboardFocus (true) ? 1.3f : 0.9f)
                                              .withMultipliedAlpha      (button.isEnabled() ? 0.9f : 0.5f);

            if (isButtonDown || isMouseOverButton)
                baseColour = baseColour.contrasting (isButtonDown ? 0.2f : 0.1f);

            auto width  = (float) button.getWidth()  - 1.0f;
            auto height = (float) button.getHeight() - 1.0f;

            if (width > 0 && height > 0)
            {
                g.setGradientFill (ColourGradient::vertical (baseColour, 0.0f,
                                                             baseColour.darker (0.1f), height));

                g.fillRect (button.getLocalBounds());
            }
        */
    }
    
    pub fn draw_tick_box(&mut self, 
        g:                    &mut Graphics,
        component:            &mut Component,
        x:                    f32,
        y:                    f32,
        w:                    f32,
        h:                    f32,
        ticked:               bool,
        is_enabled:           bool,
        is_mouse_over_button: bool,
        is_button_down:       bool)  {
        
        todo!();
        /*
            auto boxSize = w * 0.7f;

            auto isDownOrDragging = component.isEnabled() && (component.isMouseOverOrDragging() || component.isMouseButtonDown());

            auto colour = component.findColour (TextButton::buttonOnColourId)
                                   .withMultipliedSaturation ((component.hasKeyboardFocus (false) || isDownOrDragging) ? 1.3f : 0.9f)
                                   .withMultipliedAlpha (component.isEnabled() ? 1.0f : 0.7f);

            g.setColour (colour);

            Rectangle<float> r (x, y + (h - boxSize) * 0.5f, boxSize, boxSize);
            g.fillRect (r);

            if (ticked)
            {
                auto tickPath = LookAndFeel_V4::getTickShape (6.0f);
                g.setColour (isEnabled ? findColour (TextButton::buttonColourId) : Colours::grey);

                auto transform = RectanglePlacement (RectanglePlacement::centred)
                                   .getTransformToFit (tickPath.getBounds(),
                                                       r.reduced (r.getHeight() * 0.05f));

                g.fillPath (tickPath, transform);
            }
        */
    }
    
    pub fn draw_linear_slider_thumb(&mut self, 
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
            auto sliderRadius = (float) getSliderThumbRadius (slider);

            bool isDownOrDragging = slider.isEnabled() && (slider.isMouseOverOrDragging() || slider.isMouseButtonDown());

            auto knobColour = slider.findColour (Slider::rotarySliderFillColourId)
                                    .withMultipliedSaturation ((slider.hasKeyboardFocus (false) || isDownOrDragging) ? 1.3f : 0.9f)
                                    .withMultipliedAlpha (slider.isEnabled() ? 1.0f : 0.7f);

            g.setColour (knobColour);

            if (style == Slider::LinearHorizontal || style == Slider::LinearVertical)
            {
                float kx, ky;

                if (style == Slider::LinearVertical)
                {
                    kx = (float) x + (float) width * 0.5f;
                    ky = sliderPos;
                    g.fillRect (Rectangle<float> (kx - sliderRadius, ky - 2.5f, sliderRadius * 2.0f, 5.0f));
                }
                else
                {
                    kx = sliderPos;
                    ky = (float) y + (float) height * 0.5f;
                    g.fillRect (Rectangle<float> (kx - 2.5f, ky - sliderRadius, 5.0f, sliderRadius * 2.0f));
                }
            }
            else
            {
                // Just call the base class for the demo
                LookAndFeel_V2::drawLinearSliderThumb (g, x, y, width, height, sliderPos, minSliderPos, maxSliderPos, style, slider);
            }
        */
    }
    
    pub fn draw_rotary_slider(&mut self, 
        g:                  &mut Graphics,
        x:                  i32,
        y:                  i32,
        width:              i32,
        height:             i32,
        slider_pos:         f32,
        rotary_start_angle: f32,
        rotary_end_angle:   f32,
        slider:             &mut Slider)  {
        
        todo!();
        /*
            auto diameter = (float) jmin (width, height) - 4.0f;
            auto radius = (diameter / 2.0f) * std::cos (MathConstants<float>::pi / 4.0f);
            auto centreX = (float) x + (float) width  * 0.5f;
            auto centreY = (float) y + (float) height * 0.5f;
            auto rx = centreX - radius;
            auto ry = centreY - radius;
            auto rw = radius * 2.0f;
            auto angle = rotaryStartAngle + sliderPos * (rotaryEndAngle - rotaryStartAngle);
            bool isMouseOver = slider.isMouseOverOrDragging() && slider.isEnabled();

            auto baseColour = slider.isEnabled() ? slider.findColour (Slider::rotarySliderFillColourId).withAlpha (isMouseOver ? 0.8f : 1.0f)
                                                 : Colour (0x80808080);

            Rectangle<float> r (rx, ry, rw, rw);
            auto transform = AffineTransform::rotation (angle, r.getCentreX(), r.getCentreY());

            auto x1 = r.getTopLeft()   .getX();
            auto y1 = r.getTopLeft()   .getY();
            auto x2 = r.getBottomLeft().getX();
            auto y2 = r.getBottomLeft().getY();

            transform.transformPoints (x1, y1, x2, y2);

            g.setGradientFill (ColourGradient (baseColour, x1, y1,
                                               baseColour.darker (0.1f), x2, y2,
                                               false));

            Path knob;
            knob.addRectangle (r);
            g.fillPath (knob, transform);

            Path needle;
            auto r2 = r * 0.1f;
            needle.addRectangle (r2.withPosition ({ r.getCentreX() - (r2.getWidth() / 2.0f), r.getY() }));

            g.setColour (slider.findColour (Slider::rotarySliderOutlineColourId));
            g.fillPath (needle, AffineTransform::rotation (angle, r.getCentreX(), r.getCentreY()));
        */
    }
}
