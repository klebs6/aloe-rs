crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUScopeElement.cpp]

pub type AUScopeElementVector = Vec<*mut AUElement>;

pub struct AUScope {
    creator:  *mut AUBase,
    scope:    AudioUnitScope,
    elements: AUScopeElementVector,
    delegate: *mut AUScopeDelegate,
}

impl Default for AUScope {
    
    fn default() -> Self {
        todo!();
        /*
        : creator(NULL),
        : scope(0),
        : delegate(0),

        
        */
    }
}

impl Drop for AUScope {

    fn drop(&mut self) {
        todo!();
        /*
            for (ElementVector::iterator it = mElements.begin(); it != mElements.end(); ++it)
            delete *it;
        */
    }
}

impl AUScope {

    pub fn initialize(&mut self, 
        creator:      *mut AUBase,
        scope:        AudioUnitScope,
        num_elements: u32)  {
        
        todo!();
        /*
            mCreator = creator;
            mScope = scope;

            if (mDelegate)
                return mDelegate->Initialize(creator, scope, numElements);

            SetNumberOfElements(numElements);
        */
    }
    
    pub fn get_number_of_elements(&self) -> u32 {
        
        todo!();
        /*
            if (mDelegate)
                return mDelegate->GetNumberOfElements();

            return static_cast<UInt32>(mElements.size());
        */
    }
    
    pub fn get_element(&self, element_index: u32) -> *mut AUElement {
        
        todo!();
        /*
            if (mDelegate)
                return mDelegate->GetElement(elementIndex);

            AUScopeElementVector::const_iterator i = mElements.begin() + elementIndex;
                // catch passing -1 in as the elementIndex - causes a wrap around
            return (i >= mElements.end() || i < mElements.begin()) ? NULL : *i;
        */
    }
    
    pub fn safe_get_element(&mut self, element_index: u32) -> *mut AUElement {
        
        todo!();
        /*
            AUElement *element = GetElement(elementIndex);
            if (element == NULL)
                COMPONENT_THROW(kAudioUnitErr_InvalidElement);
            return element;
        */
    }
    
    pub fn get_io_element(&self, element_index: u32) -> *mut AUIOElement {
        
        todo!();
        /*
            AUElement *element = GetElement(elementIndex);
            AUIOElement *ioel = element ? element->AsIOElement () : NULL;
            if (!ioel)
                COMPONENT_THROW (kAudioUnitErr_InvalidElement);
            return ioel;
        */
    }
    
    pub fn get_scope(&self) -> AudioUnitScope {
        
        todo!();
        /*
            return mScope;
        */
    }
    
    pub fn set_delegate(&mut self, in_delegate: *mut AUScopeDelegate)  {
        
        todo!();
        /*
            mDelegate = inDelegate;
        */
    }
    
    pub fn set_number_of_elements(&mut self, num_elements: u32)  {
        
        todo!();
        /*
            if (mDelegate)
            return mDelegate->SetNumberOfElements(numElements);

        if (numElements > mElements.size()) {
            mElements.reserve(numElements);
            while (numElements > mElements.size()) {
                AUElement *elem = mCreator->CreateElement(GetScope(), static_cast<UInt32>(mElements.size()));
                mElements.push_back(elem);
            }
        } else
            while (numElements < mElements.size()) {
                AUElement *elem = mElements.back();
                mElements.pop_back();
                delete elem;
            }
        */
    }
    
    pub fn has_element_with_name(&self) -> bool {
        
        todo!();
        /*
            for (UInt32 i = 0; i < GetNumberOfElements(); ++i) {
            AUElement * el = const_cast<AUScope*>(this)->GetElement (i);
            if (el && el->HasName()) {
                return true;
            }
        }
        return false;
        */
    }
    
    pub fn add_element_names_to_dict(&mut self, in_name_dict: &mut CFMutableDictionaryRef)  {
        
        todo!();
        /*
            if (HasElementWithName())
        {
            static char string[32];
            CFMutableDictionaryRef elementDict = CFDictionaryCreateMutable  (NULL, 0,
                                    &kCFTypeDictionaryKeyCallBacks, &kCFTypeDictionaryValueCallBacks);
            CFStringRef str;
            for (UInt32 i = 0; i < GetNumberOfElements(); ++i) {
                AUElement * el = GetElement (i);
                if (el && el->HasName()) {
                    snprintf (string, sizeof(string), "%d", int(i));
                    str = CFStringCreateWithCString (NULL, string, kCFStringEncodingASCII);
                    CFDictionarySetValue (elementDict, str, el->GetName());
                    CFRelease (str);
                }
            }

            snprintf (string, sizeof(string), "%d", int(mScope));
            str = CFStringCreateWithCString (NULL, string, kCFStringEncodingASCII);
            CFDictionarySetValue (inNameDict, str, elementDict);
            CFRelease (str);
            CFRelease (elementDict);
        }
        */
    }
    
    pub fn restore_element_names(&mut self, in_name_dict: &mut CFDictionaryRef) -> bool {
        
        todo!();
        /*
            static char string[32];

        //first we have to see if we have enough elements
        bool didAddElements = false;
        unsigned int maxElNum = GetNumberOfElements();

        int dictSize = static_cast<int>(CFDictionaryGetCount(inNameDict));
        CFStringRef * keys = (CFStringRef*)CA_malloc (dictSize * sizeof (CFStringRef));
        CFDictionaryGetKeysAndValues (inNameDict, reinterpret_cast<const void**>(keys), NULL);
        for (int i = 0; i < dictSize; i++)
        {
            unsigned int intKey = 0;
            CFStringGetCString (keys[i], string, 32, kCFStringEncodingASCII);
            int result = sscanf (string, "%u", &intKey);
            // check if sscanf succeeded and element index is less than max elements.
            if (result && UInt32(intKey) < maxElNum)
            {
                CFStringRef elName = reinterpret_cast<CFStringRef>(CFDictionaryGetValue (inNameDict,  keys[i]));
                AUElement* element = GetElement (intKey);
                if (element)
                    element->SetName (elName);
            }
        }
        free (keys);

        return didAddElements;
        */
    }
    
    pub fn save_state(&mut self, data: CFMutableDataRef)  {
        
        todo!();
        /*
            AudioUnitElement nElems = GetNumberOfElements();
        for (AudioUnitElement ielem = 0; ielem < nElems; ++ielem) {
            AUElement *element = GetElement(ielem);
            UInt32 nparams = element->GetNumberOfParameters();
            if (nparams > 0) {
                struct {
                    UInt32  scope;
                    UInt32  element;
                } hdr;

                hdr.scope = CFSwapInt32HostToBig(GetScope());
                hdr.element = CFSwapInt32HostToBig(ielem);
                CFDataAppendBytes(data, (UInt8 *)&hdr, sizeof(hdr));

                element->SaveState(data);
            }
        }
        */
    }
    
    pub fn restore_state(&mut self, state: *const UInt8) -> *const UInt8 {
        
        todo!();
        /*
            const UInt8 *p = state;
        UInt32 elementIdx = CFSwapInt32BigToHost(*(UInt32 *)p); p += sizeof(UInt32);
        AUElement *element = GetElement(elementIdx);
        if (!element) {
            struct {
                AudioUnitParameterID        paramID;
                AudioUnitParameterValue     value;
            } entry;
            UInt32 nparams = CFSwapInt32BigToHost(*(UInt32 *)p);
            p += sizeof(UInt32);

            p += nparams * sizeof(entry);
        } else
            p = element->RestoreState(p);

        return p;
        */
    }
}
