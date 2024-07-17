crate::ix!();

#[cfg(not(HAL_Build))]
pub struct RefCountedLayout {
    base:      CAReferenceCounted,

    byte_size: u32,

    /**
      | mACL is variable length and thus must
      | be last
      |
      */
    macl:      AudioChannelLayout,

    /*
       only the constructors can change the actual
       state of the layout
      */
}

#[cfg(not(HAL_Build))]
impl RefCountedLayout {

    pub fn operator_new(&mut self, 
        size:     usize,
        acl_size: usize)  {
        
        todo!();
        /*
            return CA_malloc(sizeof(RefCountedLayout) - sizeof(AudioChannelLayout) + aclSize);
        */
    }
    
    pub fn operator_delete(&mut self, mem: *mut c_void)  {
        
        todo!();
        /*
            free(mem);
        */
    }
    
    pub fn new(in_data_size: u32) -> Self {
    
        todo!();
        /*
        : byte_size(inDataSize),

            memset(&mACL, 0, inDataSize);
        */
    }
    
    pub fn create_with_number_channel_descriptions(n_channels: u32) -> *mut RefCountedLayout {
        
        todo!();
        /*
            size_t size = CAAudioChannelLayout::CalculateByteSize(nChannels);
            return new(size) RefCountedLayout((UInt32)size);
        */
    }
    
    pub fn create_with_layout(layout: *const AudioChannelLayout) -> *mut RefCountedLayout {
        
        todo!();
        /*
            size_t size = CAAudioChannelLayout::CalculateByteSize(layout->mNumberChannelDescriptions);
            RefCountedLayout *acl = new(size) RefCountedLayout((UInt32)size);
            memcpy(&acl->mACL, layout, size);
            return acl;
        */
    }
    
    pub fn create_with_layout_tag(layout_tag: AudioChannelLayoutTag) -> *mut RefCountedLayout {
        
        todo!();
        /*
            RefCountedLayout *acl = CreateWithNumberChannelDescriptions(0);
            acl->mACL.mChannelLayoutTag = layoutTag;
            return acl;
        */
    }
    
    pub fn layout(&self) -> &AudioChannelLayout {
        
        todo!();
        /*
            return mACL;
        */
    }
    
    pub fn size(&self) -> u32 {
        
        todo!();
        /*
            return mByteSize;
        */
    }
    
    pub fn number_channels(&mut self) -> u32 {
        
        todo!();
        /*
            return CAAudioChannelLayout::NumberChannels(Layout());
        */
    }
    
    pub fn get_layout(&mut self) -> *mut AudioChannelLayout {
        
        todo!();
        /*
            return &mACL;
        */
    }
}
