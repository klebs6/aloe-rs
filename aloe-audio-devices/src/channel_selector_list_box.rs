crate::ix!();

pub enum ChannelSelectorListBoxBoxType
{
    audioInputType,
    audioOutputType
}

#[no_copy]
#[leak_detector]
pub struct ChannelSelectorListBox<'a> {
    base:             ListBox<'a>,
    base2:            ListBoxModel,
    setup:            AudioDeviceSetupDetails<'a>,
    ty:               ChannelSelectorListBoxBoxType,
    no_items_message: String,
    items:            Vec<String>,
}

impl<'a> ChannelSelectorListBox<'a> {

    pub fn new(
        setup_details: &AudioDeviceSetupDetails,
        box_type:      ChannelSelectorListBoxBoxType,
        no_items_text: &String

    ) -> Self {
    
        todo!();
        /*


            : ListBox ({}, nullptr), setup (setupDetails), type (boxType), noItemsMessage (noItemsText)

                refresh();
                setModel (this);
                setOutlineThickness (1);
        */
    }
    
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            items.clear();

                if (auto* currentDevice = setup.manager->getCurrentAudioDevice())
                {
                    if (type == audioInputType)
                        items = currentDevice->getInputChannelNames();
                    else if (type == audioOutputType)
                        items = currentDevice->getOutputChannelNames();

                    if (setup.useStereoPairs)
                    {
                        Vec<String> pairs;

                        for (int i = 0; i < items.size(); i += 2)
                        {
                            auto& name = items[i];

                            if (i + 1 >= items.size())
                                pairs.add (name.trim());
                            else
                                pairs.add (getNameForChannelPair (name, items[i + 1]));
                        }

                        items = pairs;
                    }
                }

                updateContent();
                repaint();
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return items.size();
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        row:    i32,
        g:      &mut Graphics,
        width:  i32,
        height: i32,
        _4:     bool)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (row, items.size()))
                {
                    g.fillAll (findColour (ListBox::backgroundColourId));

                    auto item = items[row];
                    bool enabled = false;
                    auto config = setup.manager->getAudioDeviceSetup();

                    if (setup.useStereoPairs)
                    {
                        if (type == audioInputType)
                            enabled = config.inputChannels[row * 2] || config.inputChannels[row * 2 + 1];
                        else if (type == audioOutputType)
                            enabled = config.outputChannels[row * 2] || config.outputChannels[row * 2 + 1];
                    }
                    else
                    {
                        if (type == audioInputType)
                            enabled = config.inputChannels[row];
                        else if (type == audioOutputType)
                            enabled = config.outputChannels[row];
                    }

                    auto x = getTickX();
                    auto tickW = (float) height * 0.75f;

                    getLookAndFeel().drawTickBox (g, *this, (float) x - tickW, ((float) height - tickW) * 0.5f, tickW, tickW,
                                                  enabled, true, true, false);

                    drawTextLayout (g, *this, item, { x + 5, 0, width - x - 5, height }, enabled);
                }
        */
    }
    
    pub fn list_box_item_clicked(&mut self, 
        row: i32,
        e:   &MouseEvent)  {
        
        todo!();
        /*
            selectRow (row);

                if (e.x < getTickX())
                    flipEnablement (row);
        */
    }
    
    pub fn list_box_item_double_clicked(&mut self, 
        row: i32,
        _1:  &MouseEvent)  {
        
        todo!();
        /*
            flipEnablement (row);
        */
    }
    
    pub fn return_key_pressed(&mut self, row: i32)  {
        
        todo!();
        /*
            flipEnablement (row);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            ListBox::paint (g);

                if (items.isEmpty())
                {
                    g.setColour (Colours::grey);
                    g.setFont (0.5f * (float) getRowHeight());
                    g.drawText (noItemsMessage,
                                0, 0, getWidth(), getHeight() / 2,
                                Justification::centred, true);
                }
        */
    }
    
    pub fn get_best_height(&mut self, max_height: i32) -> i32 {
        
        todo!();
        /*
            return getRowHeight() * jlimit (2, jmax (2, maxHeight / getRowHeight()),
                                                getNumRows())
                           + getOutlineThickness() * 2;
        */
    }
    
    pub fn get_name_for_channel_pair(
        name1: &String,
        name2: &String) -> String {
        
        todo!();
        /*
            String commonBit;

                for (int j = 0; j < name1.length(); ++j)
                    if (name1.substring (0, j).equalsIgnoreCase (name2.substring (0, j)))
                        commonBit = name1.substring (0, j);

                // Make sure we only split the name at a space, because otherwise, things
                // like "input 11" + "input 12" would become "input 11 + 2"
                while (commonBit.isNotEmpty() && ! CharacterFunctions::isWhitespace (commonBit.getLastCharacter()))
                    commonBit = commonBit.dropLastCharacters (1);

                return name1.trim() + " + " + name2.substring (commonBit.length()).trim();
        */
    }
    
    pub fn flip_enablement(&mut self, row: i32)  {
        
        todo!();
        /*
            jassert (type == audioInputType || type == audioOutputType);

                if (isPositiveAndBelow (row, items.size()))
                {
                    auto config = setup.manager->getAudioDeviceSetup();

                    if (setup.useStereoPairs)
                    {
                        BigInteger bits;
                        auto& original = (type == audioInputType ? config.inputChannels
                                                                 : config.outputChannels);

                        for (int i = 0; i < 256; i += 2)
                            bits.setBit (i / 2, original[i] || original[i + 1]);

                        if (type == audioInputType)
                        {
                            config.useDefaultInputChannels = false;
                            flipBit (bits, row, setup.minNumInputChannels / 2, setup.maxNumInputChannels / 2);
                        }
                        else
                        {
                            config.useDefaultOutputChannels = false;
                            flipBit (bits, row, setup.minNumOutputChannels / 2, setup.maxNumOutputChannels / 2);
                        }

                        for (int i = 0; i < 256; ++i)
                            original.setBit (i, bits[i / 2]);
                    }
                    else
                    {
                        if (type == audioInputType)
                        {
                            config.useDefaultInputChannels = false;
                            flipBit (config.inputChannels, row, setup.minNumInputChannels, setup.maxNumInputChannels);
                        }
                        else
                        {
                            config.useDefaultOutputChannels = false;
                            flipBit (config.outputChannels, row, setup.minNumOutputChannels, setup.maxNumOutputChannels);
                        }
                    }

                    setup.manager->setAudioDeviceSetup (config, true);
                }
        */
    }
    
    pub fn flip_bit(
        chans:      &mut BigInteger,
        index:      i32,
        min_number: i32,
        max_number: i32)  {
        
        todo!();
        /*
            auto numActive = chans.countNumberOfSetBits();

                if (chans[index])
                {
                    if (numActive > minNumber)
                        chans.setBit (index, false);
                }
                else
                {
                    if (numActive >= maxNumber)
                    {
                        auto firstActiveChan = chans.findNextSetBit (0);
                        chans.clearBit (index > firstActiveChan ? firstActiveChan : chans.getHighestBit());
                    }

                    chans.setBit (index, true);
                }
        */
    }
    
    pub fn get_tickx(&self) -> i32 {
        
        todo!();
        /*
            return getRowHeight();
        */
    }
}
