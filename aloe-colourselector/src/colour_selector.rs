crate::ix!();

/**
  | A component that lets the user choose
  | a colour.
  | 
  | This shows RGB sliders and a colourspace
  | that the user can pick colours from.
  | 
  | This class is also a ChangeBroadcaster,
  | so listeners can register to be told
  | when the colour changes.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ColourSelector<'a> {
    base:              Component<'a>,
    base2:             ChangeBroadcaster<'a>,
    colour:            Colour,
    h:                 f32,
    s:                 f32,
    v:                 f32,
    sliders:           [Box<Slider<'a>>; 4],
    colour_space:      Box<ColourSpaceView<'a>>,
    hue_selector:      Box<HueSelectorComp<'a>>,
    preview_component: Box<ColourPreviewComp<'a>>,
    swatch_components: Vec<Box<SwatchComponent<'a>>>,
    flags:             i32,
    edge_gap:          i32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_ColourSelector.cpp]
impl<'a> Drop for ColourSelector<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            dispatchPendingMessages();
        swatchComponents.clear();
        */
    }
}

impl<'a> ColourSelector<'a> {
    
    /**
      | Creates a ColourSelector object.
      | 
      | The flags are a combination of values
      | from the ColourSelectorOptions enum,
      | specifying which of the selector's
      | features should be visible.
      | 
      | The edgeGap value specifies the amount
      | of space to leave around the edge.
      | 
      | gapAroundColourSpaceComponent indicates
      | how much of a gap to put around the colourspace
      | and hue selector components.
      |
      */
    pub fn new(
        sections_to_show:                  Option<ColourSelectorOptions>,
        edge:                              Option<i32>,
        gap_around_colour_space_component: Option<i32>,

    ) -> Self {

        let sections_to_show = sections_to_show.unwrap_or(
            ColourSelectorOptions::showAlphaChannel 
            | ColourSelectorOptions::showColourAtTop 
            | ColourSelectorOptions::showSliders 
            | ColourSelectorOptions::showColourspace
        );

        let edge: i32 = edge.unwrap_or(4); //edge gap

        let gap_around_colour_space_component: i32 =
                 gap_around_colour_space_component.unwrap_or(7);
    
        todo!();
        /*
        : colour(Colours::white),
        : flags(sectionsToShow),
        : edge_gap(edge),

            // not much point having a selector with no components in it!
        jassert ((flags & (showColourAtTop | showSliders | showColourspace)) != 0);

        updateHSV();

        if ((flags & showColourAtTop) != 0)
        {
            previewComponent.reset (new ColourPreviewComp (*this, (flags & editableColour) != 0));
            addAndMakeVisible (previewComponent.get());
        }

        if ((flags & showSliders) != 0)
        {
            sliders[0].reset (new ColourComponentSlider (TRANS ("red")));
            sliders[1].reset (new ColourComponentSlider (TRANS ("green")));
            sliders[2].reset (new ColourComponentSlider (TRANS ("blue")));
            sliders[3].reset (new ColourComponentSlider (TRANS ("alpha")));

            addAndMakeVisible (sliders[0].get());
            addAndMakeVisible (sliders[1].get());
            addAndMakeVisible (sliders[2].get());
            addChildComponent (sliders[3].get());

            sliders[3]->setVisible ((flags & showAlphaChannel) != 0);

            // VS2015 needs some scoping braces around this if statement to
            // avoid a compiler bug.
            for (auto& slider : sliders)
            {
                slider->onValueChange = [this] { changeColour(); };
            }
        }

        if ((flags & showColourspace) != 0)
        {
            colourSpace.reset (new ColourSpaceView (*this, h, s, v, gapAroundColourSpaceComponent));
            hueSelector.reset (new HueSelectorComp (*this, h, gapAroundColourSpaceComponent));

            addAndMakeVisible (colourSpace.get());
            addAndMakeVisible (hueSelector.get());
        }

        update (dontSendNotification);
        */
    }
    
    /**
      | Returns the colour that the user has
      | currently selected.
      | 
      | The ColourSelector class is also a ChangeBroadcaster,
      | so listeners can register to be told
      | when the colour changes.
      | 
      | @see setCurrentColour
      |
      */
    pub fn get_current_colour(&self) -> Colour {
        
        todo!();
        /*
            return ((flags & showAlphaChannel) != 0) ? colour : colour.withAlpha ((uint8) 0xff);
        */
    }
    
    /**
      | Changes the colour that is currently
      | being shown.
      | 
      | -----------
      | @param newColour
      | 
      | the new colour to show
      | ----------
      | @param notificationType
      | 
      | whether to send a notification of the
      | change to listeners.
      | 
      | A notification will only be sent if the
      | colour has changed.
      |
      */
    pub fn set_current_colour(
        &mut self, 
        c:            Colour,
        notification: Option<NotificationType>

    ) {

        let notification = notification.unwrap_or(NotificationType::sendNotification);
        
        todo!();
        /*
            if (c != colour)
        {
            colour = ((flags & showAlphaChannel) != 0) ? c : c.withAlpha ((uint8) 0xff);

            updateHSV();
            update (notification);
        }
        */
    }
    
    pub fn set_hue(&mut self, newh: f32)  {
        
        todo!();
        /*
            newH = jlimit (0.0f, 1.0f, newH);

        if (h != newH)
        {
            h = newH;
            colour = Colour (h, s, v, colour.getFloatAlpha());
            update (sendNotification);
        }
        */
    }
    
    pub fn setsv(&mut self, 
        news: f32,
        newv: f32)  {
        
        todo!();
        /*
            newS = jlimit (0.0f, 1.0f, newS);
        newV = jlimit (0.0f, 1.0f, newV);

        if (s != newS || v != newV)
        {
            s = newS;
            v = newV;
            colour = Colour (h, s, v, colour.getFloatAlpha());
            update (sendNotification);
        }
        */
    }
    
    pub fn updatehsv(&mut self)  {
        
        todo!();
        /*
            colour.getHSB (h, s, v);
        */
    }
    
    pub fn update(&mut self, notification: NotificationType)  {
        
        todo!();
        /*
            if (sliders[0] != nullptr)
        {
            sliders[0]->setValue ((int) colour.getRed(),   notification);
            sliders[1]->setValue ((int) colour.getGreen(), notification);
            sliders[2]->setValue ((int) colour.getBlue(),  notification);
            sliders[3]->setValue ((int) colour.getAlpha(), notification);
        }

        if (colourSpace != nullptr)
        {
            colourSpace->updateIfNeeded();
            hueSelector->updateIfNeeded();
        }

        if (previewComponent != nullptr)
            previewComponent->updateIfNeeded();

        if (notification != dontSendNotification)
            sendChangeMessage();

        if (notification == sendNotificationSync)
            dispatchPendingMessages();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (backgroundColourId));

        if ((flags & showSliders) != 0)
        {
            g.setColour (findColour (labelTextColourId));
            g.setFont (11.0f);

            for (auto& slider : sliders)
            {
                if (slider->isVisible())
                    g.drawText (slider->getName() + ":",
                                0, slider->getY(),
                                slider->getX() - 8, slider->getHeight(),
                                Justification::centredRight, false);
            }
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            const int swatchesPerRow = 8;
        const int swatchHeight = 22;

        const int numSliders = ((flags & showAlphaChannel) != 0) ? 4 : 3;
        const int numSwatches = getNumSwatches();

        const int swatchSpace = numSwatches > 0 ? edgeGap + swatchHeight * ((numSwatches + 7) / swatchesPerRow) : 0;
        const int sliderSpace = ((flags & showSliders) != 0)  ? jmin (22 * numSliders + edgeGap, proportionOfHeight (0.3f)) : 0;
        const int topSpace = ((flags & showColourAtTop) != 0) ? jmin (30 + edgeGap * 2, proportionOfHeight (0.2f)) : edgeGap;

        if (previewComponent != nullptr)
            previewComponent->setBounds (edgeGap, edgeGap, getWidth() - edgeGap * 2, topSpace - edgeGap * 2);

        int y = topSpace;

        if ((flags & showColourspace) != 0)
        {
            const int hueWidth = jmin (50, proportionOfWidth (0.15f));

            colourSpace->setBounds (edgeGap, y,
                                    getWidth() - hueWidth - edgeGap - 4,
                                    getHeight() - topSpace - sliderSpace - swatchSpace - edgeGap);

            hueSelector->setBounds (colourSpace->getRight() + 4, y,
                                    getWidth() - edgeGap - (colourSpace->getRight() + 4),
                                    colourSpace->getHeight());

            y = getHeight() - sliderSpace - swatchSpace - edgeGap;
        }

        if ((flags & showSliders) != 0)
        {
            auto sliderHeight = jmax (4, sliderSpace / numSliders);

            for (int i = 0; i < numSliders; ++i)
            {
                sliders[i]->setBounds (proportionOfWidth (0.2f), y,
                                       proportionOfWidth (0.72f), sliderHeight - 2);

                y += sliderHeight;
            }
        }

        if (numSwatches > 0)
        {
            const int startX = 8;
            const int xGap = 4;
            const int yGap = 4;
            const int swatchWidth = (getWidth() - startX * 2) / swatchesPerRow;
            y += edgeGap;

            if (swatchComponents.size() != numSwatches)
            {
                swatchComponents.clear();

                for (int i = 0; i < numSwatches; ++i)
                {
                    auto* sc = new SwatchComponent (*this, i);
                    swatchComponents.add (sc);
                    addAndMakeVisible (sc);
                }
            }

            int x = startX;

            for (int i = 0; i < swatchComponents.size(); ++i)
            {
                auto* sc = swatchComponents.getUnchecked(i);

                sc->setBounds (x + xGap / 2,
                               y + yGap / 2,
                               swatchWidth - xGap,
                               swatchHeight - yGap);

                if (((i + 1) % swatchesPerRow) == 0)
                {
                    x = startX;
                    y += swatchHeight;
                }
                else
                {
                    x += swatchWidth;
                }
            }
        }
        */
    }
    
    pub fn change_colour(&mut self)  {
        
        todo!();
        /*
            if (sliders[0] != nullptr)
            setCurrentColour (Colour ((uint8) sliders[0]->getValue(),
                                      (uint8) sliders[1]->getValue(),
                                      (uint8) sliders[2]->getValue(),
                                      (uint8) sliders[3]->getValue()));
        */
    }
    
    pub fn get_num_swatches(&self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn get_swatch_colour(&self, _0: i32) -> Colour {
        
        todo!();
        /*
            jassertfalse; // if you've overridden getNumSwatches(), you also need to implement this method
        return Colours::black;
        */
    }
    
    pub fn set_swatch_colour(&mut self, 
        _0: i32,
        _1: &Colour)  {
        
        todo!();
        /*
            jassertfalse; // if you've overridden getNumSwatches(), you also need to implement this method
        */
    }
}
