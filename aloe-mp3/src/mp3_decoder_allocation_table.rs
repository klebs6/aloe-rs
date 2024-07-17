#[cfg(feature = "mp3")] //see lib.rs for disclaimer
crate::ix!();

pub struct Mp3DecoderAllocationTable
{
    bits: i16,
    d:    i16,
}

macro_rules! a { ($bits:expr, $d:expr) => { Mp3DecoderAllocationTable { bits: $bits, d: $d } } }

pub const alloc_table0: &[Mp3DecoderAllocationTable] = 
&[
    a![4, 0], 
    a![5, 3], 
    a![3, -3], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![14, -8191], 
    a![15, -16383], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![3, -3], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![14, -8191], 
    a![15, -16383], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![3, -3], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![14, -8191], 
    a![15, -16383], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],  
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],  
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],  
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],  
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],  
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],  
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![2, 0], 
    a![5, 3], 
    a![7, 5], 
    a![16, -32767], 
    a![2, 0], 
    a![5, 3], 
    a![7, 5], 
    a![16, -32767], 
    a![2, 0], 
    a![5, 3], 
    a![7, 5], 
    a![16, -32767], 
    a![2, 0], 
    a![5, 3], 
    a![7, 5], 
    a![16, -32767]
];

pub const alloc_table1: &[Mp3DecoderAllocationTable] = 
&[
    a![4, 0], 
    a![5, 3], 
    a![3, -3], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![14, -8191], 
    a![15, -16383], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![3, -3], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![14, -8191], 
    a![15, -16383], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![3, -3], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![14, -8191], 
    a![15, -16383], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767], 
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767], 
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767], 
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767], 
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767], 
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767], 
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![3, -3], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![16, -32767],
    
    a![2, 0], 
    a![5, 3], 
    a![7, 5], 
    a![16, -32767], 
    a![2, 0], 
    a![5, 3], 
    a![7, 5], 
    a![16, -32767], 
    a![2, 0], 
    a![5, 3], 
    a![7, 5], 
    a![16, -32767], 
    a![2, 0], 
    a![5, 3], 
    a![7, 5], 
    a![16, -32767],
    
    a![2, 0], 
    a![5, 3], 
    a![7, 5], 
    a![16, -32767], 
    a![2, 0], 
    a![5, 3], 
    a![7, 5], 
    a![16, -32767], 
    a![2, 0], 
    a![5, 3], 
    a![7, 5], 
    a![16, -32767]
];

pub const alloc_table2: &[Mp3DecoderAllocationTable] = 
&[
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![14, -8191], 
    a![15, -16383],
    
    a![4, 0], 
    a![5, 3], 
    a![7, 5], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![8, -127], 
    a![9, -255], 
    a![10, -511], 
    a![11, -1023], 
    a![12, -2047], 
    a![13, -4095], 
    a![14, -8191], 
    a![15, -16383],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63],
    
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63], 
    a![3, 0], 
    a![5, 3], 
    a![7, 5], 
    a![10, 9], 
    a![4, -7], 
    a![5, -15], 
    a![6, -31], 
    a![7, -63]
];

lazy_static!{
    /*
    constexpr Mp3DecoderAllocationTable allocTable3[] =
    {
        {4, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {8, -127}, {9, -255}, {10, -511}, {11, -1023}, {12, -2047}, {13, -4095}, {14, -8191}, {15, -16383},
        {4, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {8, -127}, {9, -255}, {10, -511}, {11, -1023}, {12, -2047}, {13, -4095}, {14, -8191}, {15, -16383},
        {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63},
        {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63},
        {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63},
        {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63},
        {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}
    };
    */
}

lazy_static!{
    /*
    constexpr Mp3DecoderAllocationTable allocTable4[] =
    {
        {4, 0}, {5, 3}, {7, 5}, {3, -3}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {8, -127}, {9, -255}, {10, -511}, {11, -1023}, {12, -2047}, {13, -4095}, {14, -8191},
        {4, 0}, {5, 3}, {7, 5}, {3, -3}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {8, -127}, {9, -255}, {10, -511}, {11, -1023}, {12, -2047}, {13, -4095}, {14, -8191},
        {4, 0}, {5, 3}, {7, 5}, {3, -3}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {8, -127}, {9, -255}, {10, -511}, {11, -1023}, {12, -2047}, {13, -4095}, {14, -8191},
        {4, 0}, {5, 3}, {7, 5}, {3, -3}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {8, -127}, {9, -255}, {10, -511}, {11, -1023}, {12, -2047}, {13, -4095}, {14, -8191},
        {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63},
        {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63},
        {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63},
        {3, 0}, {5, 3}, {7, 5}, {10, 9}, {4, -7}, {5, -15}, {6, -31}, {7, -63}, {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9},
        {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9},
        {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9},
        {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9},
        {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9}, {2, 0}, {5, 3}, {7, 5}, {10, 9},
        {2, 0}, {5, 3}, {7, 5}, {10, 9}
    };
    */
}
