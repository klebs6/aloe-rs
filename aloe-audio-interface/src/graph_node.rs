crate::ix!();

/**
  | A convenient typedef for referring
  | to a pointer to a node object.
  |
  */
pub type AudioProcessorGraphNodePtr = Rc<RefCell<dyn AudioProcessorGraphNodeInterface>>;

pub trait AudioProcessorGraphNodeInterface 
: GetProcessor
+ IsBypassed
+ SetBypassed
+ SetParentGraph
+ Prepare
+ Unprepare
+ ProcessBlock
+ ProcessBlockBypassed
{}

pub trait SetParentGraph {

    fn set_parent_graph(&self, _0: *mut dyn AudioProcessorGraphInterface);
}
