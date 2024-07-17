crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct ZIPTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for ZIPTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("ZIP", UnitTestCategories::compression
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl ZIPTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("ZIP");

            ZipFile::ZipFileBuilder builder;
            StringArray entryNames { "first", "second", "third" };
            HashMap<String, MemoryBlock> blocks;

            for (auto& entryName : entryNames)
            {
                auto& block = blocks.getReference (entryName);
                MemoryOutputStream mo (block, false);
                mo << entryName;
                mo.flush();
                builder.addEntry (new MemoryInputStream (block, false), 9, entryName, Time::getCurrentTime());
            }

            MemoryBlock data;
            MemoryOutputStream mo (data, false);
            builder.writeToStream (mo, nullptr);
            MemoryInputStream mi (data, false);

            ZipFile zip (mi);

            expectEquals (zip.getNumEntries(), entryNames.size());

            for (auto& entryName : entryNames)
            {
                auto* entry = zip.getEntry (entryName);
                std::unique_ptr<InputStream> input (zip.createStreamForEntry (*entry));
                expectEquals (input->readEntireStreamAsString(), entryName);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static ZIPTests zipTests;
    */
}
