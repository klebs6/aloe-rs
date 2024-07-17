crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vstparameters.cpp]

pub type ParameterPtrVector         = Vec<IPtr<Parameter>>;
pub type ParameterContainerIndexMap = HashMap<ParamID,usize>;

/**
  | Collection of parameters. \ingroup
  | vstClasses
  |
  */
pub struct ParameterContainer {
    params:   *mut ParameterPtrVector,
    id2index: ParameterContainerIndexMap,
}

impl Drop for ParameterContainer {

    fn drop(&mut self) {
        todo!();
        /*
            if (params)
            delete params;
        */
    }
}

impl Default for ParameterContainer {

    fn default() -> Self {

        todo!();
        /*
           : params(nullptr),


           */
    }
}

impl ParameterContainer {
    
    /**
      | Init param array.
      |
      */
    pub fn init(
        &mut self, 
        initial_size: Option<i32>,
        resize_delta: Option<i32>
    ) {

        let initial_size: i32 = initial_size.unwrap_or(10);
        let resize_delta: i32 = resize_delta.unwrap_or(100);
        
        todo!();
        /*
            if (!params)
        {
            params = new ParameterPtrVector;
            if (initialSize > 0)
                params->reserve (initialSize);
        }
        */
    }
    
    /**
      | Adds a given parameter.
      |
      */
    pub fn add_parameter_from_parameter_ptr(&mut self, p: *mut Parameter) -> *mut Parameter {
        
        todo!();
        /*
            if (!params)
            init ();
        id2index[p->getInfo ().id] = params->size ();
        params->push_back (IPtr<Parameter> (p, false));
        return p;
        */
    }
    
    /**
      | Creates and adds a new parameter from
      | a ParameterInfo.
      |
      */
    pub fn add_parameter_from_parameter_info(&mut self, info: &ParameterInfo) -> *mut Parameter {
        
        todo!();
        /*
            if (!params)
            init ();
        auto* p = new Parameter (info);
        if (addParameter (p))
            return p;
        p->release ();
        return nullptr;
        */
    }
    
    /**
      | Gets parameter by ID.
      |
      */
    pub fn get_parameter(&self, tag: ParamID) -> *mut Parameter {
        
        todo!();
        /*
            if (params)
        {
            auto it = id2index.find (tag);
            if (it != id2index.end ())
                return params->at (it->second);
        }
        return nullptr;
        */
    }
    
    /**
      | Remove a specific parameter by ID.
      |
      */
    pub fn remove_parameter(&mut self, tag: ParamID) -> bool {
        
        todo!();
        /*
            if (!params)
            return false;
        
        IndexMap::const_iterator it = id2index.find (tag);
        if (it != id2index.end ())
        {
            params->erase (params->begin () + it->second);
            id2index.erase (it);
        }
        return false;
        */
    }
    
    /**
      | Creates and adds a new parameter with
      | given properties.
      |
      */
    pub fn add_parameter(
        &mut self, 
        title:                    *const TChar,
        units:                    *const TChar,
        step_count:               Option<i32>,
        default_normalized_value: Option<ParamValue>,
        flags:                    Option<ParameterInfoParameterFlags>,
        tag:                      Option<i32>,
        unitid:                   Option<UnitID>,
        short_title:              *const TChar

    ) -> *mut Parameter {

        let step_count:                i32          = step_count.unwrap_or(0);
        let default_normalized_value:  ParamValue   = default_normalized_value.unwrap_or(0.);
        let tag:                       i32          = tag.unwrap_or(-1);
        let unitid:                    UnitID       = unitid.unwrap_or(ROOT_UNIT_ID);

        let flags = flags.unwrap_or(ParameterInfoParameterFlags::CanAutomate);
        
        todo!();
        /*
            if (!title)
        {
            return nullptr;
        }

        ParameterInfo info = {};

        UString (info.title, str16BufferSize (String128)).assign (title);
        if (units)
            UString (info.units, str16BufferSize (String128)).assign (units);
        if (shortTitle)
            UString (info.shortTitle, str16BufferSize (String128)).assign (shortTitle);

        info.stepCount = stepCount;
        info.defaultNormalizedValue = defaultNormalizedValue;
        info.flags = flags;
        info.id = (tag >= 0) ? tag : getParameterCount ();
        info.unitId = unitID;

        return addParameter (info);
        */
    }

    /**
      | Returns the count of parameters.
      |
      */
    pub fn get_parameter_count(&self) -> i32 {
        
        todo!();
        /*
            return params ? static_cast<int32> (params->size ()) : 0;
        */
    }

    /**
      | Gets parameter by index.
      |
      */
    pub fn get_parameter_by_index(&self, index: i32) -> *mut Parameter {
        
        todo!();
        /*
            return params ? params->at (index) : nullptr;
        */
    }

    /**
      | Removes all parameters.
      |
      */
    pub fn remove_all(&mut self)  {
        
        todo!();
        /*
            if (params)
                params->clear ();
            id2index.clear ();
        */
    }
}
