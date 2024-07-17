crate::ix!();

pub fn make_attachment<Attachment, Control>(
        state_to_use: &AudioProcessorValueTreeState,
        parameterid:  &String,
        control:      &mut Control) -> Box<Attachment> {

    todo!();
        /*
            if (auto* parameter = stateToUse.getParameter (parameterID))
            return std::make_unique<Attachment> (*parameter, control, stateToUse.undoManager);

        jassertfalse;
        return nullptr;
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioProcessorValueTreeState.h]

/**
  | This class contains a ValueTree that
  | is used to manage an AudioProcessor's
  | entire state.
  | 
  | It has its own internal class of parameter
  | object that is linked to values within
  | its ValueTree, and which are each identified
  | by a string ID.
  | 
  | You can get access to the underlying
  | ValueTree object via the state member
  | variable, so you can add extra properties
  | to it as necessary.
  | 
  | It also provides some utility child
  | classes for connecting parameters
  | directly to
  | 
  | GUI controls like sliders.
  | 
  | The favoured constructor of this class
  | takes a collection of RangedAudioParameters
  | or
  | 
  | AudioProcessorParameterGroups of
  | RangedAudioParameters and adds them
  | to the attached
  | 
  | AudioProcessor directly.
  | 
  | The deprecated way of using this class
  | is as follows:
  | 
  | 1) Create an AudioProcessorValueTreeState,
  | and give it some parameters using createAndAddParameter().
  | 2) Initialise the state member variable
  | with a type name.
  | 
  | The deprecated constructor will be
  | removed from the API in a future version
  | of Aloe!
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioProcessorValueTreeState<'a> {

    base:                Timer,

    /**
      | A reference to the processor with which
      | this state is associated.
      |
      */
    processor:           &'a mut AudioProcessor<'a>,

    /**
      | The state of the whole processor.
      | 
      | This must be initialised after all calls
      | to createAndAddParameter().
      | 
      | You can replace this with your own ValueTree
      | object, and can add properties and children
      | to the tree. This class will automatically
      | add children for each of the parameter
      | objects that are created by createAndAddParameter().
      |
      */
    state:               ValueTree,

    /**
      | Provides access to the undo manager
      | that this object is using.
      |
      */
    undo_manager:        *const UndoManager<'a>,

    value_type:          Identifier, // default = { "PARAM"  }
    value_propertyid:    Identifier, // default = { "value"  }
    id_propertyid:       Identifier, // default = { "id"  }
    adapter_table:       HashMap<
        &'a str,
        Box<AudioProcessorValueTreeStateParameterAdapter<'a>>,
        AudioProcessorValueTreeStateStringRefLessThan
    >,
    value_tree_changing: CriticalSection,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioProcessorValueTreeState.cpp]
impl<'a> AudioProcessorValueTreeStateListener for AudioProcessorValueTreeState<'a> {

    fn parameter_changed(
        &mut self, 
        parameterid: &String, 
        new_value:   f32
    ) {}
}

impl<'a> Drop for AudioProcessorValueTreeState<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            stopTimer();
        */
    }
}

impl<'a> AudioProcessorValueTreeState<'a> {

    /**
      | Creates a state object for a given processor,
      | and sets up all the parameters that will
      | control that processor.
      | 
      | You should *not* assign a new ValueTree
      | to the state, or call createAndAddParameter,
      | after using this constructor.
      | 
      | -----------
      | @note
      | 
      | each AudioProcessorValueTreeState
      | should be attached to only one processor,
      | and must have the same lifetime as the
      | processor, as they will have dependencies
      | on each other.
      | 
      | The AudioProcessorValueTreeStateParameterLayout parameter has
      | a set of constructors that allow you
      | to add multiple RangedAudioParameters
      | and AudioProcessorParameterGroups
      | containing
      | 
      | RangedAudioParameters to the AudioProcessorValueTreeState
      | inside this constructor.
      | 
      | -----------
      | @param processorToConnectTo
      | 
      | The Processor that will be managed by
      | this object
      | ----------
      | @param undoManagerToUse
      | 
      | An optional UndoManager to use; pass
      | nullptr if no UndoManager is required
      | ----------
      | @param valueTreeType
      | 
      | The identifier used to initialise the
      | internal ValueTree
      | ----------
      | @param parameterLayout
      | 
      | An object that holds all parameters
      | and parameter groups that the
      | 
      | AudioProcessor should use.
      | 
      | -----------
      | @code
      | 
      | YourAudioProcessor()
      |     : apvts (*this, &undoManager, "PARAMETERS",
      |              { std::make_unique<AudioParameterFloat> ("a", "AudioProcessorValueTreeStateParameter A", NormalisableRange<float> (-100.0f, 100.0f), 0),
      |                std::make_unique<AudioParameterInt> ("b", "AudioProcessorValueTreeStateParameter B", 0, 5, 2) })
      | 
      | To add parameters programatically
      | you can call `add` repeatedly on a
      | 
      | AudioProcessorValueTreeStateParameterLayout instance:
      | ----------
      | @code
      | 
      | AudioProcessorValueTreeState::AudioProcessorValueTreeStateParameterLayout createParameterLayout()
      | {
      |     AudioProcessorValueTreeState::AudioProcessorValueTreeStateParameterLayout layout;
      | 
      |     for (int i = 1; i < 9; ++i)
      |         layout.add (std::make_unique<AudioParameterInt> (String (i), String (i), 0, i, 0));
      | 
      |     return layout;
      | }
      | 
      | YourAudioProcessor()
      |     : apvts (*this, &undoManager, "PARAMETERS", createParameterLayout())
      | {
      | }
      |
      */
    pub fn new_with_parameter_layout(
        processor_to_connect_to: &mut AudioProcessor,
        undo_manager_to_use:     *mut UndoManager,
        value_tree_type:         &Identifier,
        parameter_layout:        AudioProcessorValueTreeStateParameterLayout) -> Self {
    
        todo!();
        /*


            : AudioProcessorValueTreeState (processorToConnectTo, undoManagerToUse)

        struct PushBackVisitor : AudioProcessorValueTreeStateParameterLayout::Visitor
        {
            explicit PushBackVisitor (AudioProcessorValueTreeState& stateIn)
                : state (&stateIn) {}

            void visit (std::unique_ptr<RangedAudioParameter> param) const override
            {
                if (param == nullptr)
                {
                    jassertfalse;
                    return;
                }

                state->addParameterAdapter (*param);
                state->processor.addParameter (param.release());
            }

            void visit (std::unique_ptr<AudioProcessorParameterGroup> group) const override
            {
                if (group == nullptr)
                {
                    jassertfalse;
                    return;
                }

                for (const auto param : group->getParameters (true))
                {
                    if (const auto rangedParam = dynamic_cast<RangedAudioParameter*> (param))
                    {
                        state->addParameterAdapter (*rangedParam);
                    }
                    else
                    {
                        // If you hit this assertion then you are attempting to add a parameter that is
                        // not derived from RangedAudioParameter to the AudioProcessorValueTreeState.
                        jassertfalse;
                    }
                }

                state->processor.addParameterGroup (move (group));
            }

            AudioProcessorValueTreeState* state;
        };

        for (auto& item : parameterLayout.parameters)
            item->accept (PushBackVisitor (*this));

        state = ValueTree (valueTreeType);
        */
    }
    
    /**
      | This constructor is discouraged and
      | will be deprecated in a future version
      | of Aloe!
      | 
      | Use the other constructor instead.
      | 
      | Creates a state object for a given processor.
      | 
      | The UndoManager is optional and can
      | be a nullptr. After creating your state
      | object, you should add parameters with
      | the createAndAddParameter() method.
      | Note that each
      | 
      | AudioProcessorValueTreeState should
      | be attached to only one processor, and
      | must have the same lifetime as the processor,
      | as they will have dependencies on each
      | other.
      |
      */
    pub fn new(
        p:  &mut AudioProcessor,
        um: *mut UndoManager) -> Self {
    
        todo!();
        /*
        : processor(p),
        : undo_manager(um),

            startTimerHz (10);
        state.addListener (this);
        */
    }
    
    pub fn create_and_add_parameter_with_full_args(&mut self, 
        paramid:                  &String,
        param_name:               &String,
        label_text:               &String,
        range:                    NormalisableRange<f32>,
        default_val:              f32,
        value_to_text_function:   fn(_0: f32) -> String,
        text_to_value_function:   fn(_0: &String) -> f32,
        is_meta_parameter:        bool,
        is_automatable_parameter: bool,
        is_discrete_parameter:    bool,
        category:                 AudioProcessorParameterCategory,
        is_boolean_parameter:     bool) -> *mut RangedAudioParameter {
        
        todo!();
        /*
            return createAndAddParameter (std::make_unique<AudioProcessorValueTreeStateParameter> (paramID,
                                                                   paramName,
                                                                   labelText,
                                                                   range,
                                                                   defaultVal,
                                                                   std::move (valueToTextFunction),
                                                                   std::move (textToValueFunction),
                                                                   isMetaParameter,
                                                                   isAutomatableParameter,
                                                                   isDiscreteParameter,
                                                                   category,
                                                                   isBooleanParameter));
        */
    }
    
    /**
      | This function adds a parameter to the
      | attached AudioProcessor and that parameter
      | will be managed by this AudioProcessorValueTreeState
      | object.
      |
      */
    pub fn create_and_add_parameter(&mut self, param: Box<RangedAudioParameter>) -> *mut RangedAudioParameter {
        
        todo!();
        /*
            if (param == nullptr)
            return nullptr;

        // All parameters must be created before giving this manager a ValueTree state!
        jassert (! state.isValid());

        if (getParameter (param->paramID) != nullptr)
            return nullptr;

        addParameterAdapter (*param);

        processor.addParameter (param.get());

        return param.release();
        */
    }
    
    pub fn add_parameter_adapter(&mut self, param: &mut RangedAudioParameter)  {
        
        todo!();
        /*
            adapterTable.emplace (param.paramID, std::make_unique<AudioProcessorValueTreeStateParameterAdapter> (param));
        */
    }
    
    pub fn get_parameter_adapter(&self, paramid: &str) -> *mut AudioProcessorValueTreeStateParameterAdapter {
        
        todo!();
        /*
            auto it = adapterTable.find (paramID);
        return it == adapterTable.end() ? nullptr : it->second.get();
        */
    }
    
    /**
      | Attaches a callback to one of the parameters,
      | which will be called when the parameter
      | changes.
      |
      */
    pub fn add_parameter_listener(&mut self, 
        paramid:  &str,
        listener: *mut dyn AudioProcessorValueTreeStateListener)  {
        
        todo!();
        /*
            if (auto* p = getParameterAdapter (paramID))
            p->addListener (listener);
        */
    }
    
    /**
      | Removes a callback that was previously
      | added with addParameterCallback().
      |
      */
    pub fn remove_parameter_listener(&mut self, 
        paramid:  &str,
        listener: *mut dyn AudioProcessorValueTreeStateListener)  {
        
        todo!();
        /*
            if (auto* p = getParameterAdapter (paramID))
            p->removeListener (listener);
        */
    }
    
    /**
      | Returns a Value object that can be used
      | to control a particular parameter.
      |
      */
    pub fn get_parameter_as_value(&self, paramid: &str) -> Value {
        
        todo!();
        /*
            if (auto* adapter = getParameterAdapter (paramID))
            if (adapter->tree.isValid())
                return adapter->tree.getPropertyAsValue (valuePropertyID, undoManager);

        return {};
        */
    }
    
    /**
      | Returns the range that was set when the
      | given parameter was created.
      |
      */
    pub fn get_parameter_range(&self, paramid: &str) -> NormalisableRange<f32> {
        
        todo!();
        /*
            if (auto* p = getParameterAdapter (paramID))
            return p->getRange();

        return {};
        */
    }
    
    /**
      | Returns a parameter by its ID string.
      |
      */
    pub fn get_parameter(&self, paramid: &str) -> *mut RangedAudioParameter {
        
        todo!();
        /*
            if (auto adapter = getParameterAdapter (paramID))
            return &adapter->getParameter();

        return nullptr;
        */
    }
    
    /**
      | Returns a pointer to a floating point
      | representation of a particular parameter
      | which a realtime process can read to
      | find out its current value.
      | 
      | -----------
      | @note
      | 
      | calling this method from within AudioProcessorValueTreeState::AudioProcessorValueTreeStateListener::parameterChanged()
      | is not guaranteed to return an up-to-date
      | value for the parameter.
      |
      */
    pub fn get_raw_parameter_value(&self, paramid: &str) -> *mut Atomic<f32> {
        
        todo!();
        /*
            if (auto* p = getParameterAdapter (paramID))
            return &p->getRawDenormalisedValue();

        return nullptr;
        */
    }
    
    /**
      | Returns a copy of the state value tree.
      | 
      | The AudioProcessorValueTreeState's
      | ValueTree is updated internally on
      | the message thread, but there may be
      | cases when you may want to access the
      | state from a different thread (getStateInformation
      | is a good example). This method flushes
      | all pending audio parameter value updates
      | and returns a copy of the state in a thread
      | safe way.
      | 
      | -----------
      | @note
      | 
      | This method uses locks to synchronise
      | thread access, so whilst it is thread-safe,
      | it is not realtime-safe. Do not call
      | this method from within your audio processing
      | code!
      |
      */
    pub fn copy_state(&mut self) -> ValueTree {
        
        todo!();
        /*
            ScopedLock lock (valueTreeChanging);
        flushParameterValuesToValueTree();
        return state.createCopy();
        */
    }
    
    /**
      | Replaces the state value tree.
      | 
      | The AudioProcessorValueTreeState's
      | ValueTree is updated internally on
      | the message thread, but there may be
      | cases when you may want to modify the
      | state from a different thread (setStateInformation
      | is a good example). This method allows
      | you to replace the state in a thread safe
      | way.
      | 
      | -----------
      | @note
      | 
      | This method uses locks to synchronise
      | thread access, so whilst it is thread-safe,
      | it is not realtime-safe. Do not call
      | this method from within your audio processing
      | code!
      |
      */
    pub fn replace_state(&mut self, new_state: &ValueTree)  {
        
        todo!();
        /*
            ScopedLock lock (valueTreeChanging);

        state = newState;

        if (undoManager != nullptr)
            undoManager->clearUndoHistory();
        */
    }
    
    pub fn set_new_state(&mut self, vt: ValueTree)  {
        
        todo!();
        /*
            jassert (vt.getParent() == state);

        if (auto* p = getParameterAdapter (vt.getProperty (idPropertyID).toString()))
        {
            p->tree = vt;
            p->setDenormalisedValue (p->tree.getProperty (valuePropertyID, p->getDenormalisedDefaultValue()));
        }
        */
    }
    
    pub fn update_parameter_connections_to_child_trees(&mut self)  {
        
        todo!();
        /*
            ScopedLock lock (valueTreeChanging);

        for (auto& p : adapterTable)
            p.second->tree = ValueTree();

        for (const auto& child : state)
            setNewState (child);

        for (auto& p : adapterTable)
        {
            auto& adapter = *p.second;

            if (! adapter.tree.isValid())
            {
                adapter.tree = ValueTree (valueType);
                adapter.tree.setProperty (idPropertyID, adapter.getParameter().paramID, nullptr);
                state.appendChild (adapter.tree, nullptr);
            }
        }

        flushParameterValuesToValueTree();
        */
    }
    
    pub fn value_tree_property_changed(&mut self, 
        tree: &mut ValueTree,
        _1:   &Identifier)  {
        
        todo!();
        /*
            if (tree.hasType (valueType) && tree.getParent() == state)
            setNewState (tree);
        */
    }
    
    pub fn value_tree_child_added(&mut self, 
        parent: &mut ValueTree,
        tree:   &mut ValueTree)  {
        
        todo!();
        /*
            if (parent == state && tree.hasType (valueType))
            setNewState (tree);
        */
    }
    
    pub fn value_tree_redirected(&mut self, v: &mut ValueTree)  {
        
        todo!();
        /*
            if (v == state)
            updateParameterConnectionsToChildTrees();
        */
    }
    
    pub fn flush_parameter_values_to_value_tree(&mut self) -> bool {
        
        todo!();
        /*
            ScopedLock lock (valueTreeChanging);

        bool anyUpdated = false;

        for (auto& p : adapterTable)
            anyUpdated |= p.second->flushToTree (valuePropertyID, undoManager);

        return anyUpdated;
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            auto anythingUpdated = flushParameterValuesToValueTree();

        startTimer (anythingUpdated ? 1000 / 50
                                    : jlimit (50, 500, getTimerInterval() + 20));
        */
    }
}
