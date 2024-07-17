crate::ix!();

/**
  | This is a handy base class for the state
  | of a processor (such as parameter values)
  | which is typically shared among several
  | processors. This is useful for multi-mono
  | filters which share the same state among
  | several mono processors.
  | 
  | @tags{DSP}
  |
  */
pub trait ProcessorState { }

/**
  | The ProcessorState structure is ref-counted,
  | so this is a handy type that can be used
  | as a pointer to one.
  |
  */
pub type ProcessorStatePtr = Rc<RefCell<dyn ProcessorState>>;
