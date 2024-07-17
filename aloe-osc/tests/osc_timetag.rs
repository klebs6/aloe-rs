crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct OSCTimeTagTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for OSCTimeTagTests {
    
    fn default() -> Self {
        todo!();
        /*

            : UnitTest ("OSCTimeTag class", UnitTestCategories::osc)
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl OSCTimeTagTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Basics");

            {
                OSCTimeTag tag;
                expect (tag.isImmediately());
            }
            {
                OSCTimeTag tag (3535653);
                expect (! tag.isImmediately());

                OSCTimeTag otherTag;
                otherTag = tag;
                expect (! otherTag.isImmediately());

                OSCTimeTag copyTag (tag);
                expect (! copyTag.isImmediately());
            }

            beginTest ("Conversion to/from Aloe Time");

            {
                Time time;
                OSCTimeTag tag (time);
                expect (! tag.isImmediately());
            }
            {
                OSCTimeTag tag;
                Time time = tag.toTime();
                expect (time < Time::getCurrentTime());
            }
            {
                Time currentTime (Time::currentTimeMillis());
                double deltaInSeconds = 1.234;
                RelativeTime delta (deltaInSeconds);
                Time laterTime = currentTime + delta;

                OSCTimeTag currentTimeTag (currentTime);
                OSCTimeTag laterTimeTag (laterTime);

                uint64 currentTimeTagRaw = currentTimeTag.getRawTimeTag();
                uint64 laterTimeTagRaw = laterTimeTag.getRawTimeTag();

                // in the raw time tag, the most significant 32 bits are seconds,
                // so let's verify that the difference is right:
                uint64 diff = laterTimeTagRaw - currentTimeTagRaw;
                double acceptableErrorInSeconds = 0.000001; // definitely not audible anymore.

                expect ((float) diff / float (1ULL << 32) < deltaInSeconds + acceptableErrorInSeconds );
                expect ((float) diff / float (1ULL << 32) > deltaInSeconds - acceptableErrorInSeconds );

                // round trip:

                Time currentTime2 = currentTimeTag.toTime();
                Time laterTime2 = laterTimeTag.toTime();
                RelativeTime delta2 = laterTime2 - currentTime2;

                expect (currentTime2 == currentTime);
                expect (laterTime2 == laterTime);
                expect (delta2 == delta);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static OSCTimeTagTests OSCTimeTagUnitTests;
    */
}
