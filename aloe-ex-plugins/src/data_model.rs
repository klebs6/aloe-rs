crate::ix!();

pub struct DataModel<'a> {
    audio_format_manager: *mut AudioFormatManager,
    value_tree:           ValueTree,
    sample_reader:        CachedValue<'a, Arc<dyn AudioFormatReaderFactory>>,
    centre_frequency_hz:  CachedValue<'a, f64>,
    loop_mode:            CachedValue<'a, LoopMode>,
    loop_points_seconds:  CachedValue<'a, Range<f64>>,
    listener_list:        ListenerList<Box<dyn DataModelListener>>,
}

impl<'a> ValueTreeListener for DataModel<'a> {

}

impl<'a> From<&DataModel<'a>> for DataModel<'a> {

    fn from(other: &DataModel) -> Self {
    
        todo!();
        /*
            : DataModel (*other.audioFormatManager, other.valueTree)
        */
    }
}
    
impl<'a> From<&mut AudioFormatManager> for DataModel<'a> {
    
    fn from(audio_format_manager_in: &mut AudioFormatManager) -> Self {
    
        todo!();
        /*
        : data_model(audioFormatManagerIn, ValueTree (IDs::DATA_MODEL)),

        
        */
    }
}

impl<'a> DataModel<'a> {
    
    pub fn new(
        audio_format_manager_in: &mut AudioFormatManager,
        vt:                      &ValueTree) -> Self {
    
        todo!();
        /*


            : audioFormatManager (&audioFormatManagerIn),
              valueTree (vt),
              sampleReader      (valueTree, IDs::sampleReader,      nullptr),
              centreFrequencyHz (valueTree, IDs::centreFrequencyHz, nullptr),
              loopMode          (valueTree, IDs::loopMode,          nullptr, LoopMode::none),
              loopPointsSeconds (valueTree, IDs::loopPointsSeconds, nullptr)

            jassert (valueTree.hasType (IDs::DATA_MODEL));
            valueTree.addListener (this);
        */
    }
    
    pub fn assign_from(&mut self, other: &DataModel) -> &mut DataModel {
        
        todo!();
        /*
            auto copy (other);
            swap (copy);
            return *this;
        */
    }
    
    pub fn get_sample_reader(&self) -> Box<AudioFormatReader> {
        
        todo!();
        /*
            return sampleReader != nullptr ? sampleReader.get()->make (*audioFormatManager) : nullptr;
        */
    }
    
    pub fn set_sample_reader(&mut self, 
        reader_factory: Box<dyn AudioFormatReaderFactory>,
        undo_manager:   *mut UndoManager)  {
        
        todo!();
        /*
            sampleReader.setValue (move (readerFactory), undoManager);
            setLoopPointsSeconds (Range<double> (0, getSampleLengthSeconds()).constrainRange (loopPointsSeconds),
                                  undoManager);
        */
    }
    
    pub fn get_sample_length_seconds(&self) -> f64 {
        
        todo!();
        /*
            if (auto r = getSampleReader())
                return (double) r->lengthInSamples / r->sampleRate;

            return 1.0;
        */
    }
    
    pub fn get_centre_frequency_hz(&self) -> f64 {
        
        todo!();
        /*
            return centreFrequencyHz;
        */
    }
    
    pub fn set_centre_frequency_hz(&mut self, 
        value:        f64,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            centreFrequencyHz.setValue (Range<double> (20, 20000).clipValue (value),
                                        undoManager);
        */
    }
    
    pub fn get_loop_mode(&self) -> LoopMode {
        
        todo!();
        /*
            return loopMode;
        */
    }
    
    pub fn set_loop_mode(&mut self, 
        value:        LoopMode,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            loopMode.setValue (value, undoManager);
        */
    }
    
    pub fn get_loop_points_seconds(&self) -> Range<f64> {
        
        todo!();
        /*
            return loopPointsSeconds;
        */
    }
    
    pub fn set_loop_points_seconds(&mut self, 
        value:        Range<f64>,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            loopPointsSeconds.setValue (Range<double> (0, getSampleLengthSeconds()).constrainRange (value),
                                        undoManager);
        */
    }
    
    pub fn mpe_settings(&mut self) -> MPESettingsDataModel {
        
        todo!();
        /*
            return MPESettingsDataModel (valueTree.getOrCreateChildWithName (IDs::MPE_SETTINGS, nullptr));
        */
    }
    
    pub fn add_listener(&mut self, listener: &mut dyn DataModelListener)  {
        
        todo!();
        /*
            listenerList.add (&listener);
        */
    }
    
    pub fn remove_listener(&mut self, listener: &mut dyn DataModelListener)  {
        
        todo!();
        /*
            listenerList.remove (&listener);
        */
    }
    
    pub fn swap(&mut self, other: &mut DataModel)  {
        
        todo!();
        /*
            using std::swap;
            swap (other.valueTree, valueTree);
        */
    }
    
    pub fn get_audio_format_manager(&self) -> &mut AudioFormatManager {
        
        todo!();
        /*
            return *audioFormatManager;
        */
    }
    
    pub fn value_tree_property_changed(&mut self, 
        _0:       &mut ValueTree,
        property: &Identifier)  {
        
        todo!();
        /*
            if (property == IDs::sampleReader)
            {
                sampleReader.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.sampleReaderChanged (sampleReader); });
            }
            else if (property == IDs::centreFrequencyHz)
            {
                centreFrequencyHz.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.centreFrequencyHzChanged (centreFrequencyHz); });
            }
            else if (property == IDs::loopMode)
            {
                loopMode.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.loopModeChanged (loopMode); });
            }
            else if (property == IDs::loopPointsSeconds)
            {
                loopPointsSeconds.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.loopPointsSecondsChanged (loopPointsSeconds); });
            }
        */
    }
    
    pub fn value_tree_child_added(&mut self, 
        _0: &mut ValueTree,
        _1: &mut ValueTree)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn value_tree_child_removed(&mut self, 
        _0: &mut ValueTree,
        _1: &mut ValueTree,
        _2: i32)  {
        
        todo!();
        /*
            jassertfalse;
        */
    }
    
    pub fn value_tree_child_order_changed(&mut self, 
        _0: &mut ValueTree,
        _1: i32,
        _2: i32)  {
        
        todo!();
        /*
            jassertfalse;
        */
    }
    
    pub fn value_tree_parent_changed(&mut self, _0: &mut ValueTree)  {
        
        todo!();
        /*
            jassertfalse;
        */
    }
}
