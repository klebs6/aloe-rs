crate::ix!();

#[no_copy]
#[leak_detector]
pub struct VoiceRow<'a> {
    base:                 Component<'a>,
    base2:                Timer,
    is_selected:          bool, // default = false
    has_been_purchased:   bool, // default = false
    purchase_in_progress: bool, // default = false
    row_selected:         i32, // default = -1
    avatar:               Image,
    voices:               StringArray,
    purchases:            &'a mut VoicePurchases<'a>,
    name_label:           Label<'a>,
    price_label:          Label<'a>,
    purchase_button:      TextButton<'a>, // default = { "Purchase" }
}

impl<'a> VoiceRow<'a> {

    pub fn new(voice_purchases: &mut VoicePurchases) -> Self {
    
        todo!();
        /*
        : purchases(voicePurchases),

            addAndMakeVisible (nameLabel);
                addAndMakeVisible (purchaseButton);
                addAndMakeVisible (priceLabel);

                purchaseButton.onClick = [this] { clickPurchase(); };

                voices = purchases.getVoiceNames();

                setSize (600, 33);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (4);
                {
                    auto voiceIconBounds = r.removeFromLeft (r.getHeight());
                    g.setColour (Colours::black);
                    g.drawRect (voiceIconBounds);

                    voiceIconBounds.reduce (1, 1);
                    g.setColour (hasBeenPurchased ? Colours::white : Colours::grey);
                    g.fillRect (voiceIconBounds);

                    g.drawImage (avatar, voiceIconBounds.toFloat());

                    if (! hasBeenPurchased)
                    {
                        g.setColour (Colours::white.withAlpha (0.8f));
                        g.fillRect (voiceIconBounds);

                        if (purchaseInProgress)
                            getLookAndFeel().drawSpinningWaitAnimation (g, Colours::darkgrey,
                                                                        voiceIconBounds.getX(),
                                                                        voiceIconBounds.getY(),
                                                                        voiceIconBounds.getWidth(),
                                                                        voiceIconBounds.getHeight());
                    }
                }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (4 + 8, 4);
                auto h = r.getHeight();
                auto w = static_cast<int> (h * 1.5);

                r.removeFromLeft (h);
                purchaseButton.setBounds (r.removeFromRight (w).withSizeKeepingCentre (w, h / 2));

                nameLabel.setBounds (r.removeFromTop (18));
                priceLabel.setBounds (r.removeFromTop (18));
        */
    }
    
    pub fn update(&mut self, 
        row_number:      i32,
        row_is_selected: bool)  {
        
        todo!();
        /*
            isSelected  = rowIsSelected;
                rowSelected = rowNumber;

                if (isPositiveAndBelow (rowNumber, voices.size()))
                {
                    auto imageResourceName = voices[rowNumber] + ".png";

                    nameLabel.setText (voices[rowNumber], NotificationType::dontSendNotification);

                    auto purchase = purchases.getPurchase (rowNumber);
                    hasBeenPurchased = purchase.isPurchased;
                    purchaseInProgress = purchase.purchaseInProgress;

                    if (purchaseInProgress)
                        startTimer (1000 / 50);
                    else
                        stopTimer();

                    nameLabel.setFont (Font (16).withStyle (Font::bold | (hasBeenPurchased ? 0 : Font::italic)));
                    nameLabel.setColour (Label::textColourId, hasBeenPurchased ? Colours::white : Colours::grey);

                    priceLabel.setFont (Font (10).withStyle (purchase.priceIsKnown ? 0 : Font::italic));
                    priceLabel.setColour (Label::textColourId, hasBeenPurchased ? Colours::white : Colours::grey);
                    priceLabel.setText (purchase.purchasePrice, NotificationType::dontSendNotification);

                    if (rowNumber == 0)
                    {
                        purchaseButton.setButtonText ("Internal");
                        purchaseButton.setEnabled (false);
                    }
                    else
                    {
                        purchaseButton.setButtonText (hasBeenPurchased ? "Purchased" : "Purchase");
                        purchaseButton.setEnabled (! hasBeenPurchased && purchase.priceIsKnown);
                    }

                    setInterceptsMouseClicks (! hasBeenPurchased, ! hasBeenPurchased);

                    if (auto fileStream = createAssetInputStream (String ("Purchases/" + String (imageResourceName)).toRawUTF8()))
                        avatar = PNGImageFormat().decodeImage (*fileStream);
                }
        */
    }
    
    pub fn click_purchase(&mut self)  {
        
        todo!();
        /*
            if (rowSelected >= 0)
                {
                    if (! hasBeenPurchased)
                    {
                        purchases.purchaseVoice (rowSelected);
                        purchaseInProgress = true;
                        startTimer (1000 / 50);
                    }
                }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
}
