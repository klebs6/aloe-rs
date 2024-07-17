crate::ix!();

pub trait BusIsInput {

    /**
      | Returns true if this bus is an input bus.
      |
      */
    fn is_input(&self) -> bool;
}

pub trait GetBusIndex {

    /**
      | Returns the index of this bus.
      |
      */
    fn get_bus_index(&self) -> i32;
}

pub trait BusIsMain {

    /**
      | Returns true if the current bus is the
      | main input or output bus.
      |
      */
    fn is_main(&self) -> bool;
}
