crate::ix!();

pub fn write_little_endian_bits_in_buffer(
    buffer:    *mut c_void,
    start_bit: u32,
    num_bits:  u32,
    value:     u32
) {

    todo!();
    /*
        jassert (buffer != nullptr);
        jassert (numBits > 0 && numBits <= 32);
        jassert (numBits == 32 || (value >> numBits) == 0);

        uint8* data = static_cast<uint8*> (buffer) + startBit / 8;

        if (const uint32 offset = (startBit & 7))
        {
            const uint32 bitsInByte = 8 - offset;
            const uint8 current = *data;

            if (bitsInByte >= numBits)
            {
                *data = (uint8) ((current & ~(((1u << numBits) - 1u) << offset)) | (value << offset));
                return;
            }

            *data++ = current ^ (uint8) (((value << offset) ^ current) & (((1u << bitsInByte) - 1u) << offset));
            numBits -= bitsInByte;
            value >>= bitsInByte;
        }

        while (numBits >= 8)
        {
            *data++ = (uint8) value;
            value >>= 8;
            numBits -= 8;
        }

        if (numBits > 0)
            *data = (uint8) ((*data & (uint32) (0xff << numBits)) | value);
    */
}

pub fn read_little_endian_bits_in_buffer(
    buffer:    *const c_void,
    start_bit: u32,
    num_bits:  u32
) -> u32 {

    todo!();
    /*
        jassert (buffer != nullptr);
        jassert (numBits > 0 && numBits <= 32);

        uint32 result = 0;
        uint32 bitsRead = 0;
        const uint8* data = static_cast<const uint8*> (buffer) + startBit / 8;

        if (const uint32 offset = (startBit & 7))
        {
            const uint32 bitsInByte = 8 - offset;
            result = (uint32) (*data >> offset);

            if (bitsInByte >= numBits)
                return result & ((1u << numBits) - 1u);

            numBits -= bitsInByte;
            bitsRead += bitsInByte;
            ++data;
        }

        while (numBits >= 8)
        {
            result |= (((uint32) *data++) << bitsRead);
            bitsRead += 8;
            numBits -= 8;
        }

        if (numBits > 0)
            result |= ((*data & ((1u << numBits) - 1u)) << bitsRead);

        return result;
    */
}
