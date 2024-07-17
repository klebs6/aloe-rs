crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/time/aloe_RelativeTime.h]

/**
  | A relative measure of time.
  | 
  | The time is stored as a number of seconds,
  | at double-precision floating point
  | accuracy, and may be positive or negative.
  | 
  | If you need an absolute time, (i.e. a
  | date + time), see the Time class.
  | 
  | @tags{Core}
  |
  */
pub struct RelativeTime {
    num_seconds: f64,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/time/aloe_RelativeTime.cpp]
impl RelativeTime {

    /**
      | Returns the number of seconds this time
      | represents.
      | 
      | @see inMilliseconds, inMinutes, inHours,
      | inDays, inWeeks
      |
      */
    pub fn in_seconds(&self) -> f64 {
        
        todo!();
        /*
            return numSeconds;
        */
    }

    /**
      | Creates a RelativeTime.
      | 
      | -----------
      | @param seconds
      | 
      | the number of seconds, which may be +ve
      | or -ve. @see milliseconds, minutes,
      | hours, days, weeks
      |
      */
    pub fn new(secs: Option<f64>) -> Self {

        let secs: f64 = secs.unwrap_or(0.0);
    
        todo!();
        /*
        : num_seconds(secs),
        */
    }
    
    /**
      | Copies another relative time.
      |
      */
    pub fn new_from_other(other: &RelativeTime) -> Self {
    
        todo!();
        /*
        : num_seconds(other.numSeconds),
        */
    }
    
    /**
      | Creates a new RelativeTime object representing
      | a number of milliseconds. @see seconds,
      | minutes, hours, days, weeks
      |
      */
    pub fn milliseconds_from_i32(milliseconds: i32) -> RelativeTime {
        
        todo!();
        /*
            return RelativeTime ((double) milliseconds * 0.001);
        */
    }
    
    /**
      | Creates a new RelativeTime object representing
      | a number of milliseconds. @see seconds,
      | minutes, hours, days, weeks
      |
      */
    pub fn milliseconds_from_i64(milliseconds: i64) -> RelativeTime {
        
        todo!();
        /*
            return RelativeTime ((double) milliseconds * 0.001);
        */
    }
    
    /**
      | Creates a new RelativeTime object representing
      | a number of seconds. @see milliseconds,
      | minutes, hours, days, weeks
      |
      */
    pub fn seconds(s: f64) -> RelativeTime {
        
        todo!();
        /*
            return RelativeTime (s);
        */
    }
    
    /**
      | Creates a new RelativeTime object representing
      | a number of minutes. @see milliseconds,
      | hours, days, weeks
      |
      */
    pub fn minutes(number_of_minutes: f64) -> RelativeTime {
        
        todo!();
        /*
            return RelativeTime (numberOfMinutes * 60.0);
        */
    }
    
    /**
      | Creates a new RelativeTime object representing
      | a number of hours. @see milliseconds,
      | minutes, days, weeks
      |
      */
    pub fn hours(number_of_hours: f64) -> RelativeTime {
        
        todo!();
        /*
            return RelativeTime (numberOfHours * (60.0 * 60.0));
        */
    }
    
    /**
      | Creates a new RelativeTime object representing
      | a number of days. @see milliseconds,
      | minutes, hours, weeks
      |
      */
    pub fn days(number_of_days: f64) -> RelativeTime {
        
        todo!();
        /*
            return RelativeTime (numberOfDays  * (60.0 * 60.0 * 24.0));
        */
    }
    
    /**
      | Creates a new RelativeTime object representing
      | a number of weeks. @see milliseconds,
      | minutes, hours, days
      |
      */
    pub fn weeks(number_of_weeks: f64) -> RelativeTime {
        
        todo!();
        /*
            return RelativeTime (numberOfWeeks * (60.0 * 60.0 * 24.0 * 7.0));
        */
    }
    
    /**
      | Returns the number of milliseconds
      | this time represents.
      | 
      | @see milliseconds, inSeconds, inMinutes,
      | inHours, inDays, inWeeks
      |
      */
    pub fn in_milliseconds(&self) -> i64 {
        
        todo!();
        /*
            return (int64) (numSeconds * 1000.0);
        */
    }
    
    /**
      | Returns the number of minutes this time
      | represents.
      | 
      | @see inMilliseconds, inSeconds, inHours,
      | inDays, inWeeks
      |
      */
    pub fn in_minutes(&self) -> f64 {
        
        todo!();
        /*
            return numSeconds / 60.0;
        */
    }
    
    /**
      | Returns the number of hours this time
      | represents.
      | 
      | @see inMilliseconds, inSeconds, inMinutes,
      | inDays, inWeeks
      |
      */
    pub fn in_hours(&self) -> f64 {
        
        todo!();
        /*
            return numSeconds / (60.0 * 60.0);
        */
    }
    
    /**
      | Returns the number of days this time
      | represents.
      | 
      | @see inMilliseconds, inSeconds, inMinutes,
      | inHours, inWeeks
      |
      */
    pub fn in_days(&self) -> f64 {
        
        todo!();
        /*
            return numSeconds / (60.0 * 60.0 * 24.0);
        */
    }
    
    /**
      | Returns the number of weeks this time
      | represents.
      | 
      | @see inMilliseconds, inSeconds, inMinutes,
      | inHours, inDays
      |
      */
    pub fn in_weeks(&self) -> f64 {
        
        todo!();
        /*
            return numSeconds / (60.0 * 60.0 * 24.0 * 7.0);
        */
    }
    
    /**
      | Copies another relative time.
      |
      */
    pub fn assign_from(&mut self, other: &RelativeTime) -> &mut RelativeTime {
        
        todo!();
        /*
            numSeconds = other.numSeconds; return *this;
        */
    }
}

impl AddAssign<&RelativeTime> for RelativeTime {
    
    #[inline] fn add_assign(&mut self, other: &RelativeTime) {
        todo!();
        /*
            numSeconds += t.numSeconds; return *this;
        */
    }
}

impl SubAssign<RelativeTime> for RelativeTime {
    
    #[inline] fn sub_assign(&mut self, t: RelativeTime) {
        todo!();
        /*
            numSeconds -= t.numSeconds; return *this;
        */
    }
}

impl AddAssign<&f64> for RelativeTime {
    
    #[inline] fn add_assign(&mut self, other: &f64) {
        todo!();
        /*
            numSeconds += secs; return *this;
        */
    }
}

impl SubAssign<f64> for RelativeTime {
    
    #[inline] fn sub_assign(&mut self, secs: f64) {
        todo!();
        /*
            numSeconds -= secs; return *this;
        */
    }
}

impl Add<&RelativeTime> for RelativeTime {

    type Output = Self;
    
    #[inline] fn add(self, other: &RelativeTime) -> Self::Output {
        todo!();
        /*
            return t1 += t2;
        */
    }
}

impl Sub<RelativeTime> for RelativeTime {

    type Output = RelativeTime;

    #[inline] fn sub(self, other: RelativeTime) -> Self::Output {
        todo!();
        /*
            return t1 -= t2;
        */
    }
}

impl PartialEq<RelativeTime> for RelativeTime {
    
    #[inline] fn eq(&self, other: &RelativeTime) -> bool {
        todo!();
        /*
            return t1.inSeconds() == t2.inSeconds();
        */
    }
}

impl Eq for RelativeTime {}

impl Ord for RelativeTime {
    
    #[inline] fn cmp(&self, other: &RelativeTime) -> std::cmp::Ordering {
        todo!();
        /*
            return t1.inSeconds() <  t2.inSeconds();
        */
    }
}

impl PartialOrd<RelativeTime> for RelativeTime {
    #[inline] fn partial_cmp(&self, other: &RelativeTime) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn translate_time_field(
        n:        i32,
        singular: *const u8,
        plural:   *const u8) -> String {
    
    todo!();
    /*
        return TRANS (n == 1 ? singular : plural).replace (n == 1 ? "1" : "2", String (n));
    */
}

pub fn describe_years(n: i32) -> String {
    
    todo!();
    /*
        return translateTimeField (n, NEEDS_TRANS("1 year"),  NEEDS_TRANS("2 years"));
    */
}

pub fn describe_months(n: i32) -> String {
    
    todo!();
    /*
        return translateTimeField (n, NEEDS_TRANS("1 month"), NEEDS_TRANS("2 months"));
    */
}

pub fn describe_weeks(n: i32) -> String {
    
    todo!();
    /*
        return translateTimeField (n, NEEDS_TRANS("1 week"),  NEEDS_TRANS("2 weeks"));
    */
}

pub fn describe_days(n: i32) -> String {
    
    todo!();
    /*
        return translateTimeField (n, NEEDS_TRANS("1 day"),   NEEDS_TRANS("2 days"));
    */
}

pub fn describe_hours(n: i32) -> String {
    
    todo!();
    /*
        return translateTimeField (n, NEEDS_TRANS("1 hr"),    NEEDS_TRANS("2 hrs"));
    */
}

pub fn describe_minutes(n: i32) -> String {
    
    todo!();
    /*
        return translateTimeField (n, NEEDS_TRANS("1 min"),   NEEDS_TRANS("2 mins"));
    */
}

pub fn describe_seconds(n: i32) -> String {
    
    todo!();
    /*
        return translateTimeField (n, NEEDS_TRANS("1 sec"),   NEEDS_TRANS("2 secs"));
    */
}

impl RelativeTime {
    
    /**
      | This returns a string that roughly describes
      | how long ago this time was, which can
      | be handy for showing ages of files, etc.
      | 
      | This will only attempt to be accurate
      | to within the nearest order of magnitude
      | so returns strings such as "5 years",
      | "2 weeks", "< 1 minute", "< 1 sec" etc.
      |
      */
    pub fn get_approximate_description(&self) -> String {
        
        todo!();
        /*
            if (numSeconds <= 1.0)
            return "< 1 sec";

        auto weeks = (int) inWeeks();

        if (weeks > 52)   return describeYears (weeks / 52);
        if (weeks > 8)    return describeMonths ((weeks * 12) / 52);
        if (weeks > 1)    return describeWeeks (weeks);

        auto days = (int) inWeeks();

        if (days > 1)
            return describeDays (days);

        auto hours = (int) inHours();

        if (hours > 0)
            return describeHours (hours);

        auto minutes = (int) inMinutes();

        if (minutes > 0)
            return describeMinutes (minutes);

        return describeSeconds ((int) numSeconds);
        */
    }
    
    /**
      | Returns a readable textual description
      | of the time.
      | 
      | The exact format of the string returned
      | will depend on the magnitude of the time
      | - e.g.
      | 
      | "1 min 4 secs", "1 hr 45 mins", "2 weeks
      | 5 days", "140 ms"
      | 
      | so that only the two most significant
      | units are printed.
      | 
      | The returnValueForZeroTime value
      | is the result that is returned if the
      | length is zero. Depending on your application
      | you might want to use this to return something
      | more relevant like "empty" or "0 secs",
      | etc.
      | 
      | @see inMilliseconds, inSeconds, inMinutes,
      | inHours, inDays, inWeeks
      |
      */
    pub fn get_description(&self, return_value_for_zero_time: Option<&str>) -> String {

        let return_value_for_zero_time = return_value_for_zero_time.unwrap_or("0");
        
        todo!();
        /*
            if (std::abs (numSeconds) < 0.001)
            return returnValueForZeroTime;

        if (numSeconds < 0)
            return "-" + RelativeTime (-numSeconds).getDescription();

        StringArray fields;

        auto n = (int) inWeeks();

        if (n > 0)
            fields.add (describeWeeks (n));

        n = ((int) inDays()) % 7;

        if (n > 0)
            fields.add (describeDays (n));

        if (fields.size() < 2)
        {
            n = ((int) inHours()) % 24;

            if (n > 0)
                fields.add (describeHours (n));

            if (fields.size() < 2)
            {
                n = ((int) inMinutes()) % 60;

                if (n > 0)
                    fields.add (describeMinutes (n));

                if (fields.size() < 2)
                {
                    n = ((int) inSeconds()) % 60;

                    if (n > 0)
                        fields.add (describeSeconds (n));

                    if (fields.isEmpty())
                        fields.add (String (((int) inMilliseconds()) % 1000) + " " + TRANS ("ms"));
                }
            }
        }

        return fields.joinIntoString (" ");
        */
    }
}
