crate::ix!();

/**
  | Queue of changes for a specific parameter:
  | Vst::IParamValueQueue \ingroup vstIHost
  | vst300
  | 
  | - [host imp]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | The change queue can be interpreted
  | as segment of an automation curve. For
  | each processing block, a segment with
  | the size of the block is transmitted
  | to the processor.
  | 
  | The curve is expressed as sampling points
  | of a linear approximation of the original
  | automation curve. If the original already
  | is a linear curve, it can be transmitted
  | precisely. A non-linear curve has to
  | be converted to a linear approximation
  | by the host. Every point of the value
  | queue defines a linear section of the
  | curve as a straight line from the previous
  | point of a block to the new one. So the
  | plug-in can calculate the value of the
  | curve for any sample position in the
  | block.
  | 
  | <b>Implicit Points:</b> \n
  | 
  | In each processing block, the section
  | of the curve for each parameter is transmitted.
  | 
  | In order to reduce the amount of points,
  | the point at block position 0 can be omitted.
  | 
  | - If the curve has a slope of 0 over a period
  | of multiple blocks, only one point is
  | transmitted for the block where the
  | constant curve section starts. The
  | queue for the following blocks will
  | be empty as long as the curve slope is
  | 0.
  | 
  | - If the curve has a constant slope other
  | than 0 over the period of several blocks,
  | only the value for the last sample of
  | the block is transmitted. In this case,
  | the last valid point is at block position
  | -1. The processor can calculate the
  | value for each sample in the block by
  | using a linear interpolation:
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | double x1 = -1; // position of last point related to current buffer
  | double y1 = currentParameterValue; // last transmitted value
  | 
  | int32 pointTime = 0;
  | ParamValue pointValue = 0;
  | IParamValueQueue::getPoint (0, pointTime, pointValue);
  | 
  | double x2 = pointTime;
  | double y2 = pointValue;
  | 
  | double slope = (y2 - y1) / (x2 - x1);
  | double offset = y1 - (slope * x1);
  | 
  | double curveValue = (slope * bufferTime) + offset; // bufferTime is any position in buffer
  | 
  | \b Jumps: \n
  | 
  | A jump in the automation curve has to
  | be transmitted as two points: one with
  | the old value and one with the new value
  | at the next sample position.
  | 
  | \image html "automation.jpg"
  | 
  | See \ref IParameterChanges, \ref ProcessData
  |
  */
pub trait IParamValueQueue: FUnknown {

    /**
      | Returns its associated ID.
      |
      */
    #[PLUGIN_API]
    fn get_parameter_id(&mut self) -> ParamID;

    /**
      | Returns count of points in the queue.
      |
      */
    #[PLUGIN_API]
    fn get_point_count(&mut self) -> i32;

    /**
      | Gets the value and offset at a given index.
      |
      */
    #[PLUGIN_API]
    fn get_point(&mut self, 
            index:         i32,
            sample_offset: &mut i32,
            value:         &mut ParamValue) -> tresult;

    /**
      | Adds a new value at the end of the queue,
      | its index is returned.
      |
      */
    #[PLUGIN_API]
    fn add_point(&mut self, 
            sample_offset: i32,
            value:         ParamValue,
            index:         &mut i32) -> tresult;
}

lazy_static!{
    /*
    static const FUID iparam_value_queue_iid;
    */
}

declare_class_iid!{
    IParamValueQueue, 
    0x01263A18, 
    0xED074F6F, 
    0x98C9D356, 
    0x4686F9BA
}
