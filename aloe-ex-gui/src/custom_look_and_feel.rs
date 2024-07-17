crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/LookAndFeelDemo.h]

/**
  | Custom Look And Feel subclasss.
  | 
  | Simply override the methods you need
  | to, anything else will be inherited
  | from the base class. It's a good idea
  | not to hard code your colours, use the
  | findColour method along with appropriate
  | ColourIds so you can set these on a per-component
  | basis.
  |
  */
pub struct CustomLookAndFeel<'a> {
    base: LookAndFeel_V4<'a>,
}

impl<'a> CustomLookAndFeel<'a> {

    pub fn draw_round_thumb(&mut self, 
        g:                 &mut Graphics,
        x:                 f32,
        y:                 f32,
        diameter:          f32,
        colour:            Colour,
        outline_thickness: f32)  {
        
        todo!();
        /*
            auto halfThickness = outlineThickness * 0.5f;

            Path p;
            p.addEllipse (x + halfThickness,
                          y + halfThickness,
                          diameter - outlineThickness,
                          diameter - outlineThickness);

            DropShadow (Colours::black, 1, {}).drawForPath (g, p);

            g.setColour (colour);
            g.fillPath (p);

            g.setColour (colour.brighter());
            g.strokePath (p, PathStrokeType (outlineThickness));
        */
    }
    
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

            auto flatOnLeft   = button.isConnectedOnLeft();
            auto flatOnRight  = button.isConnectedOnRight();
            auto flatOnTop    = button.isConnectedOnTop();
            auto flatOnBottom = button.isConnectedOnBottom();

            auto width  = (float) button.getWidth()  - 1.0f;
            auto height = (float) button.getHeight() - 1.0f;

            if (width > 0 && height > 0)
            {
                auto cornerSize = jmin (15.0f, jmin (width, height) * 0.45f);
                auto lineThickness = cornerSize    * 0.1f;
                auto halfThickness = lineThickness * 0.5f;

                Path outline;
                outline.addRoundedRectangle (0.5f + halfThickness, 0.5f + halfThickness, width - lineThickness, height - lineThickness,
                                             cornerSize, cornerSize,
                                             ! (flatOnLeft  || flatOnTop),
                                             ! (flatOnRight || flatOnTop),
                                             ! (flatOnLeft  || flatOnBottom),
                                             ! (flatOnRight || flatOnBottom));

                auto outlineColour = button.findColour (button.getToggleState() ? TextButton::textColourOnId
                                                                                : TextButton::textColourOffId);

                g.setColour (baseColour);
                g.fillPath (outline);

                if (! button.getToggleState())
                {
                    g.setColour (outlineColour);
                    g.strokePath (outline, PathStrokeType (lineThickness));
                }
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

            auto colour = component.findColour (TextButton::buttonColourId)
                                   .withMultipliedSaturation ((component.hasKeyboardFocus (false) || isDownOrDragging) ? 1.3f : 0.9f)
                                   .withMultipliedAlpha (component.isEnabled() ? 1.0f : 0.7f);

            drawRoundThumb (g, x, y + (h - boxSize) * 0.5f, boxSize, colour,
                            isEnabled ? ((isButtonDown || isMouseOverButton) ? 1.1f : 0.5f) : 0.3f);

            if (ticked)
            {
                g.setColour (isEnabled ? findColour (TextButton::buttonOnColourId) : Colours::grey);

                auto scale = 9.0f;
                auto trans = AffineTransform::scale (w / scale, h / scale).translated (x - 2.5f, y + 1.0f);

                g.fillPath (LookAndFeel_V4::getTickShape (6.0f), trans);
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
            auto sliderRadius = (float) (getSliderThumbRadius (slider) - 2);

            auto isDownOrDragging = slider.isEnabled() && (slider.isMouseOverOrDragging() || slider.isMouseButtonDown());

            auto knobColour = slider.findColour (Slider::thumbColourId)
                                    .withMultipliedSaturation ((slider.hasKeyboardFocus (false) || isDownOrDragging) ? 1.3f : 0.9f)
                                    .withMultipliedAlpha (slider.isEnabled() ? 1.0f : 0.7f);

            if (style == Slider::LinearHorizontal || style == Slider::LinearVertical)
            {
                float kx, ky;

                if (style == Slider::LinearVertical)
                {
                    kx = (float) x + (float) width * 0.5f;
                    ky = sliderPos;
                }
                else
                {
                    kx = sliderPos;
                    ky = (float) y + (float) height * 0.5f;
                }

                auto outlineThickness = slider.isEnabled() ? 0.8f : 0.3f;

                drawRoundThumb (g,
                                kx - sliderRadius,
                                ky - sliderRadius,
                                sliderRadius * 2.0f,
                                knobColour, outlineThickness);
            }
            else
            {
                // Just call the base class for the demo
                LookAndFeel_V2::drawLinearSliderThumb (g, x, y, width, height, sliderPos, minSliderPos, maxSliderPos, style, slider);
            }
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
            g.fillAll (slider.findColour (Slider::backgroundColourId));

            if (style == Slider::LinearBar || style == Slider::LinearBarVertical)
            {
                Path p;

                if (style == Slider::LinearBarVertical)
                    p.addRectangle ((float) x, sliderPos, (float) width, 1.0f + (float) height - sliderPos);
                else
                    p.addRectangle ((float) x, (float) y, sliderPos - (float) x, (float) height);

                auto baseColour = slider.findColour (Slider::rotarySliderFillColourId)
                                        .withMultipliedSaturation (slider.isEnabled() ? 1.0f : 0.5f)
                                        .withMultipliedAlpha (0.8f);

                g.setColour (baseColour);
                g.fillPath (p);

                auto lineThickness = jmin (15.0f, (float) jmin (width, height) * 0.45f) * 0.1f;
                g.drawRect (slider.getLocalBounds().toFloat(), lineThickness);
            }
            else
            {
                drawLinearSliderBackground (g, x, y, width, height, sliderPos, minSliderPos, maxSliderPos, style, slider);
                drawLinearSliderThumb      (g, x, y, width, height, sliderPos, minSliderPos, maxSliderPos, style, slider);
            }
        */
    }
    
    pub fn draw_linear_slider_background(
        &mut self, 
        g:              &mut Graphics,
        x:              i32,
        y:              i32,
        width:          i32,
        height:         i32,
        slider_pos:     f32,
        min_slider_pos: f32,
        max_slider_pos: f32,
        style:          SliderStyle,
        slider:         &mut Slider
    ) {
        
        todo!();
        /*
            auto sliderRadius = (float) getSliderThumbRadius (slider) - 5.0f;
            Path on, off;

            if (slider.isHorizontal())
            {
                auto iy = (float) y + (float) height * 0.5f - sliderRadius * 0.5f;
                Rectangle<float> r ((float) x - sliderRadius * 0.5f, iy, (float) width + sliderRadius, sliderRadius);
                auto onW = r.getWidth() * ((float) slider.valueToProportionOfLength (slider.getValue()));

                on.addRectangle (r.removeFromLeft (onW));
                off.addRectangle (r);
            }
            else
            {
                auto ix = (float) x + (float) width * 0.5f - sliderRadius * 0.5f;
                Rectangle<float> r (ix, (float) y - sliderRadius * 0.5f, sliderRadius, (float) height + sliderRadius);
                auto onH = r.getHeight() * ((float) slider.valueToProportionOfLength (slider.getValue()));

                on.addRectangle (r.removeFromBottom (onH));
                off.addRectangle (r);
            }

            g.setColour (slider.findColour (Slider::rotarySliderFillColourId));
            g.fillPath (on);

            g.setColour (slider.findColour (Slider::trackColourId));
            g.fillPath (off);
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
            auto radius = (float) jmin (width / 2, height / 2) - 2.0f;
            auto centreX = (float) x + (float) width  * 0.5f;
            auto centreY = (float) y + (float) height * 0.5f;
            auto rx = centreX - radius;
            auto ry = centreY - radius;
            auto rw = radius * 2.0f;
            auto angle = rotaryStartAngle + sliderPos * (rotaryEndAngle - rotaryStartAngle);
            auto isMouseOver = slider.isMouseOverOrDragging() && slider.isEnabled();

            if (slider.isEnabled())
                g.setColour (slider.findColour (Slider::rotarySliderFillColourId).withAlpha (isMouseOver ? 1.0f : 0.7f));
            else
                g.setColour (Colour (0x80808080));

            {
                Path filledArc;
                filledArc.addPieSegment (rx, ry, rw, rw, rotaryStartAngle, angle, 0.0);
                g.fillPath (filledArc);
            }

            {
                auto lineThickness = jmin (15.0f, (float) jmin (width, height) * 0.45f) * 0.1f;
                Path outlineArc;
                outlineArc.addPieSegment (rx, ry, rw, rw, rotaryStartAngle, rotaryEndAngle, 0.0);
                g.strokePath (outlineArc, PathStrokeType (lineThickness));
            }
        */
    }
}
