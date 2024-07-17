crate::ix!();

pub struct CAAudioChannelLayout {
    layout: *mut RefCountedLayout,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAAudioChannelLayout.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAAudioChannelLayout.cpp]

impl PartialEq<CAAudioChannelLayout> for CAAudioChannelLayout {
    
    #[inline] fn eq(&self, other: &CAAudioChannelLayout) -> bool {
        todo!();
        /*
            // compare based on the number of channel descriptions present
        // (this may be too strict a comparison if all you care about are matching layout tags)
        UInt32 theSize1 = CAAudioChannelLayout::CalculateByteSize(x.mNumberChannelDescriptions);
        UInt32 theSize2 = CAAudioChannelLayout::CalculateByteSize(y.mNumberChannelDescriptions);

        if (theSize1 != theSize2)
            return false;

        return !memcmp (&x, &y, theSize1);
        */
    }
}

impl Eq for CAAudioChannelLayout {}

impl Into<AudioChannelLayout> for CAAudioChannelLayout {
    
    #[inline] fn into(self) -> AudioChannelLayout {
        todo!();
        /*
            return &Layout();
        */
    }
}

impl CAAudioChannelLayout {
    
    pub fn calculate_byte_size(in_number_channel_descriptions: u32) -> u32 {
        
        todo!();
        /*
            return SizeOf32(AudioChannelLayout) - SizeOf32(AudioChannelDescription) + (inNumberChannelDescriptions * SizeOf32(AudioChannelDescription));
        */
    }
    
    /**
      | if inChooseSurround is false, then symmetrical
      | speaker arrangements are chosen in place of
      | surround layouts if there is a choice
      |
      | This call chooses layouts based on the expected
      | defaults in AudioUnit usage
      */
    #[cfg(not(HAL_Build))]
    pub fn new(
        in_number_channels: u32,
        in_choose_surround: bool) -> Self {
    
        todo!();
        /*
        
        */
    }
    
    
    #[cfg(not(HAL_Build))]
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return NumberChannels() > 0;
        */
    }
    
    #[cfg(not(HAL_Build))]
    pub fn size(&self) -> u32 {
        
        todo!();
        /*
            return mLayout ? mLayout->Size() : 0;
        */
    }
    
    #[cfg(not(HAL_Build))]
    pub fn number_channels(&self) -> u32 {
        
        todo!();
        /*
            return mLayout ? mLayout->NumberChannels() : 0;
        */
    }
    
    #[cfg(not(HAL_Build))]
    pub fn tag(&self) -> AudioChannelLayoutTag {
        
        todo!();
        /*
            return Layout().mChannelLayoutTag;
        */
    }
    
    #[cfg(not(HAL_Build))]
    pub fn layout(&self) -> &AudioChannelLayout {
        
        todo!();
        /*
            return mLayout->Layout();
        */
    }
    
    #[cfg(not(HAL_Build))]
    pub fn print(&self)  {
        
        todo!();
        /*
            Print (stdout);
        */
    }
    
    #[cfg(not(HAL_Build))]
    pub fn print_to_file(&self, file: *mut libc::FILE)  {
        
        todo!();
        /*
        
        */
    }
    
    #[cfg(not(HAL_Build))]
    pub fn save(&self, out_data: *mut CFPropertyListRef) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    #[cfg(not(HAL_Build))]
    pub fn restore(&mut self, in_data: &mut CFPropertyListRef) -> OSStatus {
        
        todo!();
        /*
        
        */
    }

    pub fn create(&mut self, in_number_channel_descriptions: u32) -> *mut AudioChannelLayout {
        
        todo!();
        /*
            UInt32 theSize = CalculateByteSize(inNumberChannelDescriptions);
        AudioChannelLayout* theAnswer = static_cast<AudioChannelLayout*>(CA_calloc(1, theSize));
        if(theAnswer != NULL)
        {
            SetAllToUnknown(*theAnswer, inNumberChannelDescriptions);
        }
        return theAnswer;
        */
    }
    
    pub fn destroy(&mut self, in_channel_layout: *mut AudioChannelLayout)  {
        
        todo!();
        /*
            free(inChannelLayout);
        */
    }
    
    pub fn set_all_to_unknown(&mut self, 
        out_channel_layout:             &mut AudioChannelLayout,
        in_number_channel_descriptions: u32)  {
        
        todo!();
        /*
            outChannelLayout.mChannelLayoutTag = kAudioChannelLayoutTag_UseChannelDescriptions;
        outChannelLayout.mChannelBitmap = 0;
        outChannelLayout.mNumberChannelDescriptions = inNumberChannelDescriptions;
        for(UInt32 theChannelIndex = 0; theChannelIndex < inNumberChannelDescriptions; ++theChannelIndex)
        {
            outChannelLayout.mChannelDescriptions[theChannelIndex].mChannelLabel = kAudioChannelLabel_Unknown;
            outChannelLayout.mChannelDescriptions[theChannelIndex].mChannelFlags = 0;
            outChannelLayout.mChannelDescriptions[theChannelIndex].mCoordinates[0] = 0;
            outChannelLayout.mChannelDescriptions[theChannelIndex].mCoordinates[1] = 0;
            outChannelLayout.mChannelDescriptions[theChannelIndex].mCoordinates[2] = 0;
        }
        */
    }

    pub fn number_channels_with_layout(&mut self, in_layout: &AudioChannelLayout) -> u32 {
        
        todo!();
        /*
            if (inLayout.mChannelLayoutTag == kAudioChannelLayoutTag_UseChannelDescriptions)
            return inLayout.mNumberChannelDescriptions;

        if (inLayout.mChannelLayoutTag == kAudioChannelLayoutTag_UseChannelBitmap)
            return CountOnes (inLayout.mChannelBitmap);

        return AudioChannelLayoutTag_GetNumberOfChannels(inLayout.mChannelLayoutTag);
        */
    }
}
