crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_PropertySet.h]

/**
  | A set of named property values, which
  | can be strings, integers, floating
  | point, etc.
  | 
  | Effectively, this just wraps a StringPairArray
  | in an interface that makes it easier
  | to load and save types other than strings.
  | 
  | See the PropertiesFile class for a subclass
  | of this, which automatically broadcasts
  | change messages and saves/loads the
  | list from a file.
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub struct PropertySet {
    properties:          Vec<(String,String)>,
    fallback_properties: *mut PropertySet,
    lock:                CriticalSection,
    ignore_case_of_keys: bool,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_PropertySet.cpp]
impl PropertySet {

    /**
      | Returns the keys/value pair array containing
      | all the properties.
      |
      */
    pub fn get_all_properties(&mut self) -> &mut StringPairArray {
        
        todo!();
        /*
            return properties;
        */
    }

    /**
      | Returns the lock used when reading or
      | writing to this set
      |
      */
    pub fn get_lock(&self) -> &CriticalSection {
        
        todo!();
        /*
            return lock;
        */
    }

    /**
      | Returns the fallback property set.
      | @see setFallbackPropertySet
      |
      */
    pub fn get_fallback_property_set(&self) -> *mut PropertySet {
        
        todo!();
        /*
            return fallbackProperties;
        */
    }

    /**
      | Subclasses can override this to be told
      | when one of the properties has been changed.
      |
      */
    pub fn property_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Creates an empty PropertySet.
      | 
      | -----------
      | @param ignoreCaseOfKeyNames
      | 
      | if true, the names of properties are
      | compared in a case-insensitive way
      |
      */
    pub fn new(ignore_case_of_key_names: Option<bool>) -> Self {

        let ignore_case_of_key_names: bool =
            ignore_case_of_key_names.unwrap_or(false);
    
        todo!();
        /*
        : properties(ignoreCaseOfKeyNames),
        : fallback_properties(nullptr),
        : ignore_case_of_keys(ignoreCaseOfKeyNames),

        
        */
    }
    
    /**
      | Creates a copy of another PropertySet.
      |
      */
    pub fn new_from_property_set(other: &PropertySet) -> Self {
    
        todo!();
        /*
        : properties(other.properties),
        : fallback_properties(other.fallbackProperties),
        : ignore_case_of_keys(other.ignoreCaseOfKeys),

        
        */
    }
    
    /**
      | Copies another PropertySet over this
      | one.
      |
      */
    pub fn assign_from(&mut self, other: &PropertySet) -> &mut PropertySet {
        
        todo!();
        /*
            properties = other.properties;
        fallbackProperties = other.fallbackProperties;
        ignoreCaseOfKeys = other.ignoreCaseOfKeys;

        propertyChanged();
        return *this;
        */
    }
    
    /**
      | Removes all values.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        if (properties.size() > 0)
        {
            properties.clear();
            propertyChanged();
        }
        */
    }
    
    /**
      | Returns one of the properties as a string.
      | 
      | If the value isn't found in this set,
      | then this will look for it in a fallback
      | property set (if you've specified one
      | with the setFallbackPropertySet()
      | method), and if it can't find one there,
      | it'll return the default value passed-in.
      | 
      | -----------
      | @param keyName
      | 
      | the name of the property to retrieve
      | ----------
      | @param defaultReturnValue
      | 
      | a value to return if the named property
      | doesn't actually exist
      |
      */
    pub fn get_value(
        &self, 
        key_name:      &str,
        default_value: Option<&str>) -> String {

        let default_value = default_value.unwrap_or("");
        
        todo!();
        /*
            const ScopedLock sl (lock);
        auto index = properties.getAllKeys().indexOf (keyName, ignoreCaseOfKeys);

        if (index >= 0)
            return properties.getAllValues() [index];

        return fallbackProperties != nullptr ? fallbackProperties->getValue (keyName, defaultValue)
                                             : defaultValue;
        */
    }
    
    /**
      | Returns one of the properties as an integer.
      | 
      | If the value isn't found in this set,
      | then this will look for it in a fallback
      | property set (if you've specified one
      | with the setFallbackPropertySet()
      | method), and if it can't find one there,
      | it'll return the default value passed-in.
      | 
      | -----------
      | @param keyName
      | 
      | the name of the property to retrieve
      | ----------
      | @param defaultReturnValue
      | 
      | a value to return if the named property
      | doesn't actually exist
      |
      */
    pub fn get_int_value(
        &self, 
        key_name:      &str,
        default_value: Option<i32>) -> i32 {

        let default_value: i32 = default_value.unwrap_or(0);
        
        todo!();
        /*
            const ScopedLock sl (lock);
        auto index = properties.getAllKeys().indexOf (keyName, ignoreCaseOfKeys);

        if (index >= 0)
            return properties.getAllValues() [index].getIntValue();

        return fallbackProperties != nullptr ? fallbackProperties->getIntValue (keyName, defaultValue)
                                             : defaultValue;
        */
    }
    
    /**
      | Returns one of the properties as an double.
      | 
      | If the value isn't found in this set,
      | then this will look for it in a fallback
      | property set (if you've specified one
      | with the setFallbackPropertySet()
      | method), and if it can't find one there,
      | it'll return the default value passed-in.
      | 
      | -----------
      | @param keyName
      | 
      | the name of the property to retrieve
      | ----------
      | @param defaultReturnValue
      | 
      | a value to return if the named property
      | doesn't actually exist
      |
      */
    pub fn get_double_value(
        &self, 
        key_name:      &str,
        default_value: Option<f64>) -> f64 {

        let default_value: f64 = default_value.unwrap_or(0.0);
        
        todo!();
        /*
            const ScopedLock sl (lock);
        auto index = properties.getAllKeys().indexOf (keyName, ignoreCaseOfKeys);

        if (index >= 0)
            return properties.getAllValues()[index].getDoubleValue();

        return fallbackProperties != nullptr ? fallbackProperties->getDoubleValue (keyName, defaultValue)
                                             : defaultValue;
        */
    }
    
    /**
      | Returns one of the properties as an boolean.
      | 
      | The result will be true if the string
      | found for this key name can be parsed
      | as a non-zero integer.
      | 
      | If the value isn't found in this set,
      | then this will look for it in a fallback
      | property set (if you've specified one
      | with the setFallbackPropertySet()
      | method), and if it can't find one there,
      | it'll return the default value passed-in.
      | 
      | -----------
      | @param keyName
      | 
      | the name of the property to retrieve
      | ----------
      | @param defaultReturnValue
      | 
      | a value to return if the named property
      | doesn't actually exist
      |
      */
    pub fn get_bool_value(
        &self, 
        key_name:      &str,
        default_value: Option<bool>
    ) -> bool {

        let default_value: bool = default_value.unwrap_or(false);
        
        todo!();
        /*
            const ScopedLock sl (lock);
        auto index = properties.getAllKeys().indexOf (keyName, ignoreCaseOfKeys);

        if (index >= 0)
            return properties.getAllValues() [index].getIntValue() != 0;

        return fallbackProperties != nullptr ? fallbackProperties->getBoolValue (keyName, defaultValue)
                                             : defaultValue;
        */
    }
    
    /**
      | Returns one of the properties as an XML
      | element.
      | 
      | The result will a new XMLElement object.
      | It may return nullptr if the key isn't
      | found, or if the entry contains an string
      | that isn't valid XML.
      | 
      | If the value isn't found in this set,
      | then this will look for it in a fallback
      | property set (if you've specified one
      | with the setFallbackPropertySet()
      | method), and if it can't find one there,
      | it'll return the default value passed-in.
      | 
      | -----------
      | @param keyName
      | 
      | the name of the property to retrieve
      |
      */
    pub fn get_xml_value(&self, key_name: &str) -> Box<XmlElement> {
        
        todo!();
        /*
            return parseXML (getValue (keyName));
        */
    }
    
    /**
      | Sets a named property.
      | 
      | -----------
      | @param keyName
      | 
      | the name of the property to set. (This
      | mustn't be an empty string)
      | ----------
      | @param value
      | 
      | the new value to set it to
      |
      */
    pub fn set_value(
        &mut self, 
        key_name: &str,
        v:        &Var)  {
        
        todo!();
        /*
            jassert (keyName.isNotEmpty()); // shouldn't use an empty key name!

        if (keyName.isNotEmpty())
        {
            auto value = v.toString();
            const ScopedLock sl (lock);
            auto index = properties.getAllKeys().indexOf (keyName, ignoreCaseOfKeys);

            if (index < 0 || properties.getAllValues() [index] != value)
            {
                properties.set (keyName, value);
                propertyChanged();
            }
        }
        */
    }
    
    /**
      | Deletes a property.
      | 
      | -----------
      | @param keyName
      | 
      | the name of the property to delete. (This
      | mustn't be an empty string)
      |
      */
    pub fn remove_value(&mut self, key_name: &str)  {
        
        todo!();
        /*
            if (keyName.isNotEmpty())
        {
            const ScopedLock sl (lock);
            auto index = properties.getAllKeys().indexOf (keyName, ignoreCaseOfKeys);

            if (index >= 0)
            {
                properties.remove (keyName);
                propertyChanged();
            }
        }
        */
    }
    
    /**
      | Sets a named property to an XML element.
      | 
      | -----------
      | @param keyName
      | 
      | the name of the property to set. (This
      | mustn't be an empty string)
      | ----------
      | @param xml
      | 
      | the new element to set it to. If this is
      | a nullptr, the value will be set to an
      | empty string @see getXmlValue
      |
      */
    pub fn set_value_to_xml_element(
        &mut self, 
        key_name: &str,
        xml:      *const XmlElement)  {
        
        todo!();
        /*
            setValue (keyName, xml == nullptr ? var()
                                          : var (xml->toString (XmlElement::TextFormat().singleLine().withoutHeader())));
        */
    }
    
    /**
      | Returns true if the properties include
      | the given key.
      |
      */
    pub fn contains_key(&self, key_name: &str) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        return properties.getAllKeys().contains (keyName, ignoreCaseOfKeys);
        */
    }
    
    /**
      | This copies all the values from a source
      | PropertySet to this one.
      | 
      | This won't remove any existing settings,
      | it just adds any that it finds in the source
      | set.
      |
      */
    pub fn add_all_properties_from(&mut self, source: &PropertySet)  {
        
        todo!();
        /*
            const ScopedLock sl (source.getLock());

        for (int i = 0; i < source.properties.size(); ++i)
            setValue (source.properties.getAllKeys() [i],
                      source.properties.getAllValues() [i]);
        */
    }
    
    /**
      | Sets up a second PopertySet that will
      | be used to look up any values that aren't
      | set in this one.
      | 
      | If you set this up to be a pointer to a second
      | property set, then whenever one of the
      | getValue() methods fails to find an
      | entry in this set, it will look up that
      | value in the fallback set, and if it finds
      | it, it will return that.
      | 
      | Make sure that you don't delete the fallback
      | set while it's still being used by another
      | set! To remove the fallback set, just
      | call this method with a null pointer.
      | 
      | @see getFallbackPropertySet
      |
      */
    pub fn set_fallback_property_set(&mut self, fallback_properties: *mut PropertySet)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        fallbackProperties = fallbackProperties_;
        */
    }
    
    /**
      | Returns an XML element which encapsulates
      | all the items in this property set.
      | 
      | The string parameter is the tag name
      | that should be used for the node. @see
      | restoreFromXml
      |
      */
    pub fn create_xml(&self, node_name: &String) -> Box<XmlElement> {
        
        todo!();
        /*
            auto xml = std::make_unique<XmlElement> (nodeName);

        const ScopedLock sl (lock);

        for (int i = 0; i < properties.getAllKeys().size(); ++i)
        {
            auto e = xml->createNewChildElement ("VALUE");
            e->setAttribute ("name", properties.getAllKeys()[i]);
            e->setAttribute ("val", properties.getAllValues()[i]);
        }

        return xml;
        */
    }
    
    /**
      | Reloads a set of properties that were
      | previously stored as XML.
      | 
      | The node passed in must have been created
      | by the createXml() method. @see createXml
      |
      */
    pub fn restore_from_xml(&mut self, xml: &XmlElement)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        clear();

        for (auto* e : xml.getChildWithTagNameIterator ("VALUE"))
        {
            if (e->hasAttribute ("name")
                 && e->hasAttribute ("val"))
            {
                properties.set (e->getStringAttribute ("name"),
                                e->getStringAttribute ("val"));
            }
        }

        if (properties.size() > 0)
            propertyChanged();
        */
    }
}

