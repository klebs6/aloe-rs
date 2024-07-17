#[cfg(feature = "mp3")] //see lib.rs for disclaimer
crate::ix!();

pub struct Mp3DecoderBitsToTableMap
{
    bits:  u32,
    table: *const i16,
}

lazy_static!{
    /*
    constexpr Mp3DecoderBitsToTableMap huffmanTables1[] =
    {
        { 0, huffmanTab0  }, { 0, huffmanTab1  }, { 0,  huffmanTab2  }, { 0,  huffmanTab3  },
        { 0, huffmanTab0  }, { 0, huffmanTab5  }, { 0,  huffmanTab6  }, { 0,  huffmanTab7  },
        { 0, huffmanTab8  }, { 0, huffmanTab9  }, { 0,  huffmanTab10 }, { 0,  huffmanTab11 },
        { 0, huffmanTab12 }, { 0, huffmanTab13 }, { 0,  huffmanTab0  }, { 0,  huffmanTab15 },
        { 1, huffmanTab16 }, { 2, huffmanTab16 }, { 3,  huffmanTab16 }, { 4,  huffmanTab16 },
        { 6, huffmanTab16 }, { 8, huffmanTab16 }, { 10, huffmanTab16 }, { 13, huffmanTab16 },
        { 4, huffmanTab24 }, { 5, huffmanTab24 }, { 6,  huffmanTab24 }, { 7,  huffmanTab24 },
        { 8, huffmanTab24 }, { 9, huffmanTab24 }, { 11, huffmanTab24 }, { 13, huffmanTab24 }
    };
    */
}

lazy_static!{
    /*
    constexpr int16 huffmanTabC0[] = { -29,-21,-13,-7,-3,-1,11,15,-1,13,14,-3,-1,7,5,9,-3,-1,6,3,-1,10,12,-3,-1,2,1,-1,4,8,0 };
    constexpr int16 huffmanTabC1[] = { -15,-7,-3,-1,15,14,-1,13,12,-3,-1,11,10,-1,9,8,-7,-3,-1,7,6,-1,5,4,-3,-1,3,2,-1,1,0 };
    constexpr Mp3DecoderBitsToTableMap huffmanTables2[] = { { 0, huffmanTabC0 }, { 0, huffmanTabC1 } };
    */
}
