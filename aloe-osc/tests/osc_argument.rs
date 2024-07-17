pub fn get_memory_block_with_random_data(num_bytes: usize) -> MemoryBlock {
    
    todo!();
        /*
            MemoryBlock block (numBytes);

            Random rng = getRandom();

            for (size_t i = 0; i < numBytes; ++i)
                block[i] = (char) rng.nextInt (256);

            return block;
        */
}

#[test] fn test_int32() {

    /*
        int value = 123456789;

        OSCArgument arg (value);

        expect (arg.getType() == OSCTypes::int32);
        expect (arg.isInt32());
        expect (! arg.isFloat32());
        expect (! arg.isString());
        expect (! arg.isBlob());
        expect (! arg.isColour());

        expect (arg.getInt32() == value);
    */
}

#[test] fn test_float32() {
    /*
        float value = 12345.6789f;

        OSCArgument arg (value);

        expect (arg.getType() == OSCTypes::float32);
        expect (! arg.isInt32());
        expect (arg.isFloat32());
        expect (! arg.isString());
        expect (! arg.isBlob());
        expect (! arg.isColour());

        expect (arg.getFloat32() == value);
    */
}

#[test] fn test_string() {

    /*
        String value = "Hello, World!";
        OSCArgument arg (value);

        expect (arg.getType() == OSCTypes::string);
        expect (! arg.isInt32());
        expect (! arg.isFloat32());
        expect (arg.isString());
        expect (! arg.isBlob());
        expect (! arg.isColour());

        expect (arg.getString() == value);
    */
}

#[test] fn test_string_from_cstring() {

    /*
        OSCArgument arg ("Hello, World!");

        expect (arg.getType() == OSCTypes::string);
        expect (! arg.isInt32());
        expect (! arg.isFloat32());
        expect (arg.isString());
        expect (! arg.isBlob());
        expect (! arg.isColour());

        expect (arg.getString() == "Hello, World!");
    */
}

#[test] fn test_blob() {

    /*
        auto blob = getMemoryBlockWithRandomData (413);
        OSCArgument arg (blob);

        expect (arg.getType() == OSCTypes::blob);
        expect (! arg.isInt32());
        expect (! arg.isFloat32());
        expect (! arg.isString());
        expect (arg.isBlob());
        expect (! arg.isColour());

        expect (arg.getBlob() == blob);
    */
}

#[test] fn test_colour() {

    /*
        Random rng = getRandom();

        for (int i = 100; --i >= 0;)
        {
            OSCColour col = { (uint8) rng.nextInt (256),
                              (uint8) rng.nextInt (256),
                              (uint8) rng.nextInt (256),
                              (uint8) rng.nextInt (256) };

            OSCArgument arg (col);

            expect (arg.getType() == OSCTypes::colour);
            expect (! arg.isInt32());
            expect (! arg.isFloat32());
            expect (! arg.isString());
            expect (! arg.isBlob());
            expect (arg.isColour());

            expect (arg.getColour().toInt32() == col.toInt32());
        }
    */
}

#[test] fn test_copy_move_assign() {
    /*
    {
        int value = -42;
        OSCArgument arg (value);

        OSCArgument copy = arg;
        expect (copy.getType() == OSCTypes::int32);
        expect (copy.getInt32() == value);

        OSCArgument assignment ("this will be overwritten!");
        assignment = copy;
        expect (assignment.getType() == OSCTypes::int32);
        expect (assignment.getInt32() == value);
   }
   {
        const size_t numBytes = 412;
        MemoryBlock blob = getMemoryBlockWithRandomData (numBytes);
        OSCArgument arg (blob);

        OSCArgument copy = arg;
        expect (copy.getType() == OSCTypes::blob);
        expect (copy.getBlob() == blob);

        OSCArgument assignment ("this will be overwritten!");
        assignment = copy;
        expect (assignment.getType() == OSCTypes::blob);
        expect (assignment.getBlob() == blob);
   }

    */
}
