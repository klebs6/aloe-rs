crate::ix!();

/**
  | An enumeration of the available entropy
  | coding methods.
  |
  */
#[derive(Copy,Clone)]
pub enum EntropyCodingMethodType {

    /**
      | Residual is coded by partitioning into
      | contexts, each with it's own 4-bit Rice
      | parameter.
      |
      */
    ENTROPY_CODING_METHOD_PARTITIONED_RICE = 0,

    /**
      | Residual is coded by partitioning into
      | contexts, each with it's own 5-bit Rice
      | parameter.
      |
      */
    ENTROPY_CODING_METHOD_PARTITIONED_RICE2 = 1
}

/**
  | Maps a EntropyCodingMethodType
  | to a C string.
  | 
  | Using a EntropyCodingMethodType
  | as the index to this array will give the
  | string equivalent. The contents should
  | not be modified.
  |
  */
lazy_static!{
    /*
    extern  const char * const EntropyCodingMethodTypeString[];
    */
}

/**
  | == 2 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned ENTROPY_CODING_METHOD_TYPE_LEN;
    */
}


/**
  | Contents of a Rice partitioned residual
  |
  */
pub struct EntropyCodingMethod_PartitionedRiceContents {

    /**
      | The Rice parameters for each context.
      |
      */
    parameters:        *mut u32,

    /**
      | Widths for escape-coded partitions.
      | Will be non-zero for escaped partitions
      | and zero for unescaped partitions.
      |
      */
    raw_bits:          *mut u32,

    /**
      | The capacity of the \a parameters and
      | \a raw_bits arrays specified as an order,
      | i.e. the number of array elements allocated
      | is 2 ^ \a capacity_by_order.
      |
      */
    capacity_by_order: u32,
}

/**
  | Header for a Rice partitioned residual.
  | (c.f. <A HREF="../format.html#partitioned_rice">format
  | specification</A>)
  |
  */
#[derive(Copy,Clone)]
pub struct EntropyCodingMethod_PartitionedRice {

    /**
      | The partition order, i.e. # of contexts
      | = 2 ^ \a order.
      |
      */
    order:    u32,

    /**
      | The context's Rice parameters and/or
      | raw bits.
      |
      */
    contents: *const EntropyCodingMethod_PartitionedRiceContents,
}

/**
  | == 4 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned ENTROPY_CODING_METHOD_PARTITIONED_RICE_ORDER_LEN;
    */
}

/**
  | == 4 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned ENTROPY_CODING_METHOD_PARTITIONED_RICE_PARAMETER_LEN;
    */
}

/**
  | == 5 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned ENTROPY_CODING_METHOD_PARTITIONED_RICE2_PARAMETER_LEN;
    */
}

/**
  | == 5 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned ENTROPY_CODING_METHOD_PARTITIONED_RICE_RAW_LEN;
    */
}

/**
  | == (1<<ENTROPY_CODING_METHOD_PARTITIONED_RICE_PARAMETER_LEN)-1
  |
  */
lazy_static!{
    /*
    extern  const unsigned ENTROPY_CODING_METHOD_PARTITIONED_RICE_ESCAPE_PARAMETER;
    */
}

/**
  | == (1<<ENTROPY_CODING_METHOD_PARTITIONED_RICE2_PARAMETER_LEN)-1
  |
  */
lazy_static!{
    /*
    extern  const unsigned ENTROPY_CODING_METHOD_PARTITIONED_RICE2_ESCAPE_PARAMETER;
    */
}

/**
  | Header for the entropy coding method.
  | (c.f. <A HREF="../format.html#residual">format
  | specification</A>)
  |
  */
#[derive(Copy,Clone)]
pub struct EntropyCodingMethod {
    ty:   EntropyCodingMethodType,
    data: FlacEntropyCodingMethodU,
}

#[derive(Copy,Clone)]
pub union FlacEntropyCodingMethodU {
    partitioned_rice: EntropyCodingMethod_PartitionedRice,
}
