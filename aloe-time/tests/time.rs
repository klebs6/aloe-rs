crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct TimeTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for TimeTests {
    
    fn default() -> Self {
        todo!();
        /*

            : UnitTest ("Time", UnitTestCategories::time
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl TimeTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Time");

            Time t = Time::getCurrentTime();
            expect (t > Time());

            Thread::sleep (15);
            expect (Time::getCurrentTime() > t);

            expect (t.getTimeZone().isNotEmpty());
            expect (t.getUTCOffsetString (true)  == "Z" || t.getUTCOffsetString (true).length() == 6);
            expect (t.getUTCOffsetString (false) == "Z" || t.getUTCOffsetString (false).length() == 5);

            expect (Time::fromISO8601 (t.toISO8601 (true)) == t);
            expect (Time::fromISO8601 (t.toISO8601 (false)) == t);

            expect (Time::fromISO8601 ("2016-02-16") == Time (2016, 1, 16, 0, 0, 0, 0, false));
            expect (Time::fromISO8601 ("20160216Z")  == Time (2016, 1, 16, 0, 0, 0, 0, false));

            expect (Time::fromISO8601 ("2016-02-16T15:03:57+00:00") == Time (2016, 1, 16, 15, 3, 57, 0, false));
            expect (Time::fromISO8601 ("20160216T150357+0000")      == Time (2016, 1, 16, 15, 3, 57, 0, false));

            expect (Time::fromISO8601 ("2016-02-16T15:03:57.999+00:00") == Time (2016, 1, 16, 15, 3, 57, 999, false));
            expect (Time::fromISO8601 ("20160216T150357.999+0000")      == Time (2016, 1, 16, 15, 3, 57, 999, false));
            expect (Time::fromISO8601 ("2016-02-16T15:03:57.999Z")      == Time (2016, 1, 16, 15, 3, 57, 999, false));
            expect (Time::fromISO8601 ("2016-02-16T15:03:57,999Z")      == Time (2016, 1, 16, 15, 3, 57, 999, false));
            expect (Time::fromISO8601 ("20160216T150357.999Z")          == Time (2016, 1, 16, 15, 3, 57, 999, false));
            expect (Time::fromISO8601 ("20160216T150357,999Z")          == Time (2016, 1, 16, 15, 3, 57, 999, false));

            expect (Time::fromISO8601 ("2016-02-16T15:03:57.999-02:30") == Time (2016, 1, 16, 17, 33, 57, 999, false));
            expect (Time::fromISO8601 ("2016-02-16T15:03:57,999-02:30") == Time (2016, 1, 16, 17, 33, 57, 999, false));
            expect (Time::fromISO8601 ("20160216T150357.999-0230")      == Time (2016, 1, 16, 17, 33, 57, 999, false));
            expect (Time::fromISO8601 ("20160216T150357,999-0230")      == Time (2016, 1, 16, 17, 33, 57, 999, false));

            expect (Time (1970,  0,  1,  0,  0,  0, 0, false) == Time (0));
            expect (Time (2106,  1,  7,  6, 28, 15, 0, false) == Time (4294967295000));
            expect (Time (2007, 10,  7,  1,  7, 20, 0, false) == Time (1194397640000));
            expect (Time (2038,  0, 19,  3, 14,  7, 0, false) == Time (2147483647000));
            expect (Time (2016,  2,  7, 11, 20,  8, 0, false) == Time (1457349608000));
            expect (Time (1969, 11, 31, 23, 59, 59, 0, false) == Time (-1000));
            expect (Time (1901, 11, 13, 20, 45, 53, 0, false) == Time (-2147483647000));

            expect (Time (1982, 1, 1, 12, 0, 0, 0, true) + RelativeTime::days (365) == Time (1983, 1, 1, 12, 0, 0, 0, true));
            expect (Time (1970, 1, 1, 12, 0, 0, 0, true) + RelativeTime::days (365) == Time (1971, 1, 1, 12, 0, 0, 0, true));
            expect (Time (2038, 1, 1, 12, 0, 0, 0, true) + RelativeTime::days (365) == Time (2039, 1, 1, 12, 0, 0, 0, true));

            expect (Time (1982, 1, 1, 12, 0, 0, 0, false) + RelativeTime::days (365) == Time (1983, 1, 1, 12, 0, 0, 0, false));
            expect (Time (1970, 1, 1, 12, 0, 0, 0, false) + RelativeTime::days (365) == Time (1971, 1, 1, 12, 0, 0, 0, false));
            expect (Time (2038, 1, 1, 12, 0, 0, 0, false) + RelativeTime::days (365) == Time (2039, 1, 1, 12, 0, 0, 0, false));
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static TimeTests timeTests;
    */
}
