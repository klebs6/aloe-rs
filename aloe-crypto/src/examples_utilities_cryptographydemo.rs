crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/CryptographyDemo.h]

#[no_copy]
#[leak_detector]
pub struct RSAComponent<'a> {
    base:                Component<'a>,
    rsa_group:           GroupComponent<'a>, // default = { {}, "RSA Encryption"  }
    generate_rsa_button: TextButton<'a>,     // default = { "Generate RSA"  }
    bit_size_label:      Label<'a>,          // default = { {}, "Num Bits to Use:"  }
    bit_size:            TextEditor<'a>,
    rsa_result_box:      TextEditor<'a>,
}

impl<'a> Default for RSAComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*
            addAndMakeVisible (rsaGroup);

            addAndMakeVisible (bitSize);
            bitSize.setText (String (256));
            bitSizeLabel.attachToComponent (&bitSize, true);

            addAndMakeVisible (generateRSAButton);
            generateRSAButton.onClick = [this] { createRSAKey(); };

            addAndMakeVisible (rsaResultBox);
            rsaResultBox.setReadOnly (true);
            rsaResultBox.setMultiLine (true)
        */
    }
}

impl<'a> RSAComponent<'a> {

    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds();
            rsaGroup.setBounds (area);
            area.removeFromTop (10);
            area.reduce (5, 5);

            auto topArea = area.removeFromTop (34);
            topArea.removeFromLeft (110);
            bitSize.setBounds (topArea.removeFromLeft (topArea.getWidth() / 2).reduced (5));
            generateRSAButton.setBounds (topArea.reduced (5));

            rsaResultBox.setBounds (area.reduced (5));
        */
    }
    
    pub fn create_rsa_key(&mut self)  {
        
        todo!();
        /*
            auto bits = jlimit (32, 1024, bitSize.getText().getIntValue());
            bitSize.setText (String (bits), dontSendNotification);

            // Create a key-pair...
            RSAKey publicKey, privateKey;
            RSAKey::createKeyPair (publicKey, privateKey, bits);

            // Test the new key on a piece of data...
            BigInteger testValue;
            testValue.parseString ("1234567890abcdef", 16);

            auto encodedValue = testValue;
            publicKey.applyToValue (encodedValue);

            auto decodedValue = encodedValue;
            privateKey.applyToValue (decodedValue);

            // ..and show the results..
            String message;
            message << "Number of bits: " << bits << newLine
                    << "Public Key: "  << publicKey .toString() << newLine
                    << "Private Key: " << privateKey.toString() << newLine
                    << newLine
                    << "Test input: " << testValue.toString (16) << newLine
                    << "Encoded: " << encodedValue.toString (16) << newLine
                    << "Decoded: " << decodedValue.toString (16) << newLine;

            rsaResultBox.setText (message, false);
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            rsaGroup.setColour (GroupComponent::outlineColourId,
                                getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::outline,
                                                        Colours::grey));
            rsaGroup.setColour (GroupComponent::textColourId,
                                getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::defaultText,
                                                        Colours::white));
            rsaResultBox.setColour (TextEditor::backgroundColourId,
                                    getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::widgetBackground,
                                                            Colours::white.withAlpha (0.5f)));

            bitSize.applyFontToAllText (bitSize.getFont());
            rsaResultBox.applyFontToAllText (rsaResultBox.getFont());
        */
    }
}

///-------------------
#[no_copy]
#[leak_detector]
pub struct HashesComponent<'a> {
    base:             Component<'a>,
    hash_group:       GroupComponent<'a>, // default = { {}, "Hashes"  }
    hash_entry_box:   TextEditor<'a>,
    md_5result:       Label<'a>,
    sha_result:       Label<'a>,
    whirlpool_result: Label<'a>,
    hash_label1:      Label<'a>, // default = { {}, "Text to Hash:"  }
    hash_label2:      Label<'a>, // default = { {}, "MD5 Result:"  }
    hash_label3:      Label<'a>, // default = { {}, "SHA Result:"  }
    hash_label4:      Label<'a>, // default = { {}, "Whirlpool Result:"  }
}

impl<'a> Default for HashesComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (hashGroup);

            addAndMakeVisible (hashEntryBox);
            hashEntryBox.setMultiLine (true);

            hashEntryBox.setReturnKeyStartsNewLine (true);
            hashEntryBox.setText ("Type some text in this box and the resulting MD5, SHA and Whirlpool hashes will update below");

            auto updateHashes = [this]
            {
                auto text = hashEntryBox.getText();

                updateMD5Result       (text.toUTF8());
                updateSHA256Result    (text.toUTF8());
                updateWhirlpoolResult (text.toUTF8());
            };

            hashEntryBox.onTextChange = updateHashes;
            hashEntryBox.onReturnKey  = updateHashes;

            hashLabel1.attachToComponent (&hashEntryBox,    true);
            hashLabel2.attachToComponent (&md5Result,       true);
            hashLabel3.attachToComponent (&shaResult,       true);
            hashLabel4.attachToComponent (&whirlpoolResult, true);

            addAndMakeVisible (md5Result);
            addAndMakeVisible (shaResult);
            addAndMakeVisible (whirlpoolResult);

            updateHashes()
        */
    }
}

impl<'a> HashesComponent<'a> {

    
    pub fn update_md5result(&mut self, text: CharPointer_UTF8)  {
        
        todo!();
        /*
            md5Result.setText (MD5 (text).toHexString(), dontSendNotification);
        */
    }
    
    pub fn update_sha256result(&mut self, text: CharPointer_UTF8)  {
        
        todo!();
        /*
            shaResult.setText (SHA256 (text).toHexString(), dontSendNotification);
        */
    }
    
    pub fn update_whirlpool_result(&mut self, text: CharPointer_UTF8)  {
        
        todo!();
        /*
            whirlpoolResult.setText (Whirlpool (text).toHexString(), dontSendNotification);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds();

            hashGroup.setBounds (area);

            area.removeFromLeft (120);
            area.removeFromTop (10);
            area.reduce (5, 5);

            whirlpoolResult.setBounds (area.removeFromBottom (34));
            shaResult      .setBounds (area.removeFromBottom (34));
            md5Result      .setBounds (area.removeFromBottom (34));
            hashEntryBox   .setBounds (area.reduced (5));
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            hashGroup.setColour (GroupComponent::outlineColourId,
                                 getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::outline,
                                                         Colours::grey));
            hashGroup.setColour (GroupComponent::textColourId,
                                 getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::defaultText,
                                                         Colours::white));
            hashEntryBox.setColour (TextEditor::backgroundColourId,
                                    getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::widgetBackground,
                                                            Colours::white.withAlpha (0.5f)));

            hashEntryBox.applyFontToAllText (hashEntryBox.getFont());
        */
    }
}

///-----------------
#[no_copy]
#[leak_detector]
pub struct CryptographyDemo<'a> {
    base:      Component<'a>,
    rsa_demo:  RSAComponent<'a>,
    hash_demo: HashesComponent<'a>,
}

impl<'a> Default for CryptographyDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (rsaDemo);
            addAndMakeVisible (hashDemo);

            setSize (750, 750)
        */
    }
}

impl<'a> CryptographyDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                               Colour::greyLevel (0.4f)));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds();

            rsaDemo .setBounds (area.removeFromTop (getHeight() / 2).reduced (5));
            hashDemo.setBounds (area.reduced (5));
        */
    }
}
