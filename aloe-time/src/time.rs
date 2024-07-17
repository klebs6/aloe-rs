crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/time/aloe_Time.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_core/time/aloe_Time.cpp]

/**
  | Holds an absolute date and time.
  | 
  | Internally, the time is stored at millisecond
  | precision.
  | 
  | @see RelativeTime
  | 
  | @tags{Core}
  |
  */
pub struct Time {
    millis_since_epoch: i64, // default = 0
}

impl Time {
    
    /**
      | Returns the time as a number of milliseconds.
      | 
      | -----------
      | @return
      | 
      | the number of milliseconds this Time
      | object represents, since midnight
      | Jan 1st 1970 UTC. @see getMilliseconds
      |
      */
    pub fn to_milliseconds(&self) -> i64 {
        
        todo!();
        /*
            return millisSinceEpoch;
        */
    }

    /**
      | Tries to set the computer's clock.
      | 
      | -----------
      | @return
      | 
      | true if this succeeds, although depending
      | on the system, the application might
      | not have sufficient privileges to do
      | this.
      |
      */
    pub fn set_system_time_to_this_time(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /* Static methods for getting system timers directly */

    /**
      | Returns the number of millisecs since
      | a fixed event (usually system startup).
      | 
      | This has the same function as getMillisecondCounter(),
      | but returns a more accurate value, using
      | a higher-resolution timer if one is
      | available.
      | 
      | @see getMillisecondCounter
      |
      */
    pub fn get_millisecond_counter_hi_res() -> f64 {
        
        todo!();
        /*
        
        */
    }

    /* ----------- High-resolution timers..   ----------- */

    /**
      | Returns the current high-resolution
      | counter's tick-count.
      | 
      | This is a similar idea to getMillisecondCounter(),
      | but with a higher resolution.
      | 
      | @see getHighResolutionTicksPerSecond,
      | highResolutionTicksToSeconds, secondsToHighResolutionTicks
      |
      */
    pub fn get_high_resolution_ticks() -> i64 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the resolution of the high-resolution
      | counter in ticks per second.
      | 
      | @see getHighResolutionTicks, highResolutionTicksToSeconds,
      | secondsToHighResolutionTicks
      |
      */
    pub fn get_high_resolution_ticks_per_second() -> i64 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Creates a time based on a number of milliseconds.
      | 
      | To create a time object set to the current
      | time, use getCurrentTime().
      | 
      | -----------
      | @param millisecondsSinceEpoch
      | 
      | the number of milliseconds since the
      | unix 'epoch' (midnight Jan 1st 1970
      | UTC). @see getCurrentTime, currentTimeMillis
      |
      */
    pub fn new_from_millis(ms: i64) -> Self {
    
        todo!();
        /*
        : millis_since_epoch(ms),

        
        */
    }
    
    /**
      | Creates a time from a set of date components.
      | 
      | -----------
      | @param year
      | 
      | the year, in 4-digit format, e.g. 2004
      | ----------
      | @param month
      | 
      | the month, in the range 0 to 11
      | ----------
      | @param day
      | 
      | the day of the month, in the range 1 to
      | 31
      | ----------
      | @param hours
      | 
      | hours in 24-hour clock format, 0 to 23
      | ----------
      | @param minutes
      | 
      | minutes 0 to 59
      | ----------
      | @param seconds
      | 
      | seconds 0 to 59
      | ----------
      | @param milliseconds
      | 
      | milliseconds 0 to 999
      | ----------
      | @param useLocalTime
      | 
      | if true, assume input is in this machine's
      | local timezone if false, assume input
      | is in UTC.
      |
      */
    pub fn new(
        year:           i32,
        month:          i32,
        day:            i32,
        hours:          i32,
        minutes:        i32,
        seconds:        Option<i32>,
        milliseconds:   Option<i32>,
        use_local_time: Option<bool>) -> Self {

        let seconds:        i32  = seconds.unwrap_or(0);
        let milliseconds:   i32  = milliseconds.unwrap_or(0);
        let use_local_time: bool = use_local_time.unwrap_or(true);
    
        todo!();
        /*

            std::tm t;
        t.tm_year   = year - 1900;
        t.tm_mon    = month;
        t.tm_mday   = day;
        t.tm_hour   = hours;
        t.tm_min    = minutes;
        t.tm_sec    = seconds;
        t.tm_isdst  = -1;

        millisSinceEpoch = 1000 * (useLocalTime ? (int64) mktime (&t)
                                                : TimeHelpers::mktime_utc (t))
                             + milliseconds;
        */
    }
    
    /**
      | Returns the current system time.
      | 
      | Returns the number of milliseconds
      | since midnight Jan 1st 1970 UTC.
      | 
      | Should be accurate to within a few millisecs,
      | depending on platform, hardware, etc.
      |
      */
    pub fn current_time_millis(&mut self) -> i64 {
        
        todo!();
        /*
            #if ALOE_WINDOWS && ! ALOE_MINGW
        struct _timeb t;
        _ftime_s (&t);
        return ((int64) t.time) * 1000 + t.millitm;
       #else
        struct timeval tv;
        gettimeofday (&tv, nullptr);
        return ((int64) tv.tv_sec) * 1000 + tv.tv_usec / 1000;
       #endif
        */
    }
    
    /**
      | Returns a Time object that is set to the
      | current system time.
      | 
      | This may not be monotonic, as the system
      | time can change at any moment. You should
      | therefore not use this method for measuring
      | time intervals.
      | 
      | @see currentTimeMillis
      |
      */
    pub fn get_current_time(&mut self) -> Time {
        
        todo!();
        /*
            return Time (currentTimeMillis());
        */
    }
    
    /**
      | Returns the number of millisecs since
      | a fixed event (usually system startup).
      | 
      | This returns a monotonically increasing
      | value which is unaffected by changes
      | to the system clock. It should be accurate
      | to within a few millisecs, depending
      | on platform, hardware, etc.
      | 
      | Being a 32-bit return value, it will
      | of course wrap back to 0 after 2^32 seconds
      | of uptime, so be careful to take that
      | into account. If you need a 64-bit time,
      | you can use currentTimeMillis() instead.
      | 
      | @see getApproximateMillisecondCounter
      |
      */
    pub fn get_millisecond_counter(&mut self) -> u32 {
        
        todo!();
        /*
            auto now = aloe_millisecondsSinceStartup();

        if (now < TimeHelpers::lastMSCounterValue.get())
        {
            // in multi-threaded apps this might be called concurrently, so
            // make sure that our last counter value only increases and doesn't
            // go backwards..
            if (now < TimeHelpers::lastMSCounterValue.get() - (uint32) 1000)
                TimeHelpers::lastMSCounterValue = now;
        }
        else
        {
            TimeHelpers::lastMSCounterValue = now;
        }

        return now;
        */
    }
    
    /**
      | Less-accurate but faster version of
      | getMillisecondCounter().
      | 
      | This will return the last value that
      | getMillisecondCounter() returned,
      | so doesn't need to make a system call,
      | but is less accurate - it shouldn't be
      | more than 100ms away from the correct
      | time, though, so is still accurate enough
      | for a lot of purposes.
      | 
      | @see getMillisecondCounter
      |
      */
    pub fn get_approximate_millisecond_counter(&mut self) -> u32 {
        
        todo!();
        /*
            auto t = TimeHelpers::lastMSCounterValue.get();
        return t == 0 ? getMillisecondCounter() : t;
        */
    }
    
    /**
      | Waits until the getMillisecondCounter()
      | reaches a given value.
      | 
      | This will make the thread sleep as efficiently
      | as it can while it's waiting.
      |
      */
    pub fn wait_for_millisecond_counter(&mut self, target_time: u32)  {
        
        todo!();
        /*
            for (;;)
        {
            auto now = getMillisecondCounter();

            if (now >= targetTime)
                break;

            auto toWait = (int) (targetTime - now);

            if (toWait > 2)
            {
                Thread::sleep (jmin (20, toWait >> 1));
            }
            else
            {
                // xxx should consider using mutex_pause on the mac as it apparently
                // makes it seem less like a spinlock and avoids lowering the thread pri.
                for (int i = 10; --i >= 0;)
                    Thread::yield();
            }
        }
        */
    }
    
    /**
      | Converts a number of high-resolution
      | ticks into seconds.
      | 
      | @see getHighResolutionTicks, getHighResolutionTicksPerSecond,
      | secondsToHighResolutionTicks
      |
      */
    pub fn high_resolution_ticks_to_seconds(&mut self, ticks: i64) -> f64 {
        
        todo!();
        /*
            return (double) ticks / (double) getHighResolutionTicksPerSecond();
        */
    }
    
    /**
      | Converts a number seconds into high-resolution
      | ticks.
      | 
      | @see getHighResolutionTicks, getHighResolutionTicksPerSecond,
      | highResolutionTicksToSeconds
      |
      */
    pub fn seconds_to_high_resolution_ticks(&mut self, seconds: f64) -> i64 {
        
        todo!();
        /*
            return (int64) (seconds * (double) getHighResolutionTicksPerSecond());
        */
    }
    
    /**
      | Returns a string version of this date
      | and time, using this machine's local
      | timezone.
      | 
      | For a more powerful way of formatting
      | the date and time, see the formatted()
      | method.
      | 
      | -----------
      | @param includeDate
      | 
      | whether to include the date in the string
      | ----------
      | @param includeTime
      | 
      | whether to include the time in the string
      | ----------
      | @param includeSeconds
      | 
      | if the time is being included, this provides
      | an option not to include the seconds
      | in it
      | ----------
      | @param use24HourClock
      | 
      | if the time is being included, sets whether
      | to use am/pm or 24 hour notation. @see
      | formatted
      |
      */
    pub fn to_string(&self, 
        include_date:     bool,
        include_time:     bool,
        include_seconds:  Option<bool>,
        use_24hour_clock: Option<bool>) -> String {

        let include_seconds:  bool = include_seconds.unwrap_or(true);
        let use_24hour_clock: bool = use_24hour_clock.unwrap_or(false);
        
        todo!();
        /*
            String result;

        if (includeDate)
        {
            result << getDayOfMonth() << ' '
                   << getMonthName (true) << ' '
                   << getYear();

            if (includeTime)
                result << ' ';
        }

        if (includeTime)
        {
            auto mins = getMinutes();

            result << (use24HourClock ? getHours() : getHoursInAmPmFormat())
                   << (mins < 10 ? ":0" : ":") << mins;

            if (includeSeconds)
            {
                auto secs = getSeconds();
                result << (secs < 10 ? ":0" : ":") << secs;
            }

            if (! use24HourClock)
                result << (isAfternoon() ? "pm" : "am");
        }

        return result.trimEnd();
        */
    }

    /**
      | Converts this date/time to a string
      | with a user-defined format.
      | 
      | This uses the C strftime() function
      | to format this time as a string. To save
      | you looking it up, these are the escape
      | codes that strftime uses (other codes
      | might work on some platforms and not
      | others, but these are the common ones):
      | 
      | - %a is replaced by the locale's abbreviated
      | weekday name.
      | 
      | - %A is replaced by the locale's full
      | weekday name.
      | 
      | - %b is replaced by the locale's abbreviated
      | month name.
      | 
      | - %B is replaced by the locale's full
      | month name.
      | 
      | - %c is replaced by the locale's appropriate
      | date and time representation.
      | 
      | - %d is replaced by the day of the month
      | as a decimal number [01,31].
      | 
      | - %H is replaced by the hour (24-hour
      | clock) as a decimal number [00,23].
      | 
      | - %I is replaced by the hour (12-hour
      | clock) as a decimal number [01,12].
      | 
      | - %j is replaced by the day of the year
      | as a decimal number [001,366].
      | 
      | - %m is replaced by the month as a decimal
      | number [01,12].
      | 
      | - %M is replaced by the minute as a decimal
      | number [00,59].
      | 
      | - %p is replaced by the locale's equivalent
      | of either a.m. or p.m.
      | 
      | - %S is replaced by the second as a decimal
      | number [00,60].
      | 
      | - %U is replaced by the week number of
      | the year (Sunday as the first day of the
      | week) as a decimal number [00,53].
      | 
      | - %w is replaced by the weekday as a decimal
      | number [0,6], with 0 representing Sunday.
      | 
      | - %W is replaced by the week number of
      | the year (Monday as the first day of the
      | week) as a decimal number [00,53]. All
      | days in a new year preceding the first
      | Monday are considered to be in week 0.
      | 
      | - %x is replaced by the locale's appropriate
      | date representation.
      | 
      | - %X is replaced by the locale's appropriate
      | time representation.
      | 
      | - %y is replaced by the year without century
      | as a decimal number [00,99].
      | 
      | - %Y is replaced by the year with century
      | as a decimal number.
      | 
      | - %Z is replaced by the timezone name
      | or abbreviation, or by no bytes if no
      | timezone information exists.
      | 
      | - %% is replaced by %.
      | 
      | @see toString
      |
      */
    pub fn formatted(&self, format: &String) -> String {
        
        todo!();
        /*
            std::tm t (TimeHelpers::millisToLocal (millisSinceEpoch));
        return TimeHelpers::formatString (format, &t);
        */
    }
    
    /**
      | Returns the year (in this machine's
      | local timezone).
      | 
      | A 4-digit format is used, e.g. 2004.
      |
      */
    pub fn get_year(&self) -> i32 {
        
        todo!();
        /*
            return TimeHelpers::millisToLocal (millisSinceEpoch).tm_year + 1900;
        */
    }
    
    /**
      | Returns the number of the month (in this
      | machine's local timezone).
      | 
      | The value returned is in the range 0 to
      | 11. @see getMonthName
      |
      */
    pub fn get_month(&self) -> i32 {
        
        todo!();
        /*
            return TimeHelpers::millisToLocal (millisSinceEpoch).tm_mon;
        */
    }
    
    /**
      | Returns the number of the day of the year
      | (in this machine's local timezone).
      | 
      | The value returned is in the range 0 to
      | 365.
      |
      */
    pub fn get_day_of_year(&self) -> i32 {
        
        todo!();
        /*
            return TimeHelpers::millisToLocal (millisSinceEpoch).tm_yday;
        */
    }
    
    /**
      | Returns the day of the month (in this
      | machine's local timezone).
      | 
      | The value returned is in the range 1 to
      | 31.
      |
      */
    pub fn get_day_of_month(&self) -> i32 {
        
        todo!();
        /*
            return TimeHelpers::millisToLocal (millisSinceEpoch).tm_mday;
        */
    }
    
    /**
      | Returns the number of the day of the week
      | (in this machine's local timezone).
      | 
      | The value returned is in the range 0 to
      | 6 (0 = sunday, 1 = monday, etc).
      |
      */
    pub fn get_day_of_week(&self) -> i32 {
        
        todo!();
        /*
            return TimeHelpers::millisToLocal (millisSinceEpoch).tm_wday;
        */
    }
    
    /**
      | Returns the number of hours since midnight
      | (in this machine's local timezone).
      | 
      | This is in 24-hour clock format, in the
      | range 0 to 23. @see getHoursInAmPmFormat,
      | isAfternoon
      |
      */
    pub fn get_hours(&self) -> i32 {
        
        todo!();
        /*
            return TimeHelpers::millisToLocal (millisSinceEpoch).tm_hour;
        */
    }
    
    /**
      | Returns the number of minutes, 0 to 59
      | (in this machine's local timezone).
      |
      */
    pub fn get_minutes(&self) -> i32 {
        
        todo!();
        /*
            return TimeHelpers::millisToLocal (millisSinceEpoch).tm_min;
        */
    }
    
    /**
      | Returns the number of seconds, 0 to 59.
      |
      */
    pub fn get_seconds(&self) -> i32 {
        
        todo!();
        /*
            return TimeHelpers::extendedModulo (millisSinceEpoch / 1000, 60);
        */
    }
    
    /**
      | Returns the number of milliseconds,
      | 0 to 999.
      | 
      | Unlike toMilliseconds(), this just
      | returns the position within the current
      | second rather than the total number
      | since the epoch.
      | 
      | @see toMilliseconds
      |
      */
    pub fn get_milliseconds(&self) -> i32 {
        
        todo!();
        /*
            return TimeHelpers::extendedModulo (millisSinceEpoch, 1000);
        */
    }
    
    /**
      | Returns the hours in 12-hour clock format
      | (in this machine's local timezone).
      | 
      | This will return a value 1 to 12 - use isAfternoon()
      | to find out whether this is in the afternoon
      | or morning. @see getHours, isAfternoon
      |
      */
    pub fn get_hours_in_am_pm_format(&self) -> i32 {
        
        todo!();
        /*
            auto hours = getHours();

        if (hours == 0)  return 12;
        if (hours <= 12) return hours;

        return hours - 12;
        */
    }
    
    /**
      | Returns true if the time is in the afternoon
      | (in this machine's local timezone).
      | 
      | -----------
      | @return
      | 
      | true for "PM", false for "AM". @see getHoursInAmPmFormat,
      | getHours
      |
      */
    pub fn is_afternoon(&self) -> bool {
        
        todo!();
        /*
            return getHours() >= 12;
        */
    }
    
    /**
      | Returns true if the local timezone uses
      | a daylight saving correction.
      |
      */
    pub fn is_daylight_saving_time(&self) -> bool {
        
        todo!();
        /*
            return TimeHelpers::millisToLocal (millisSinceEpoch).tm_isdst != 0;
        */
    }
    
    /**
      | Returns a 3-character string to indicate
      | the local timezone.
      |
      */
    pub fn get_time_zone(&self) -> String {
        
        todo!();
        /*
            String zone[2];

      #if ALOE_WINDOWS
       #if ALOE_MSVC || ALOE_CLANG
        _tzset();

        for (int i = 0; i < 2; ++i)
        {
            char name[128] = { 0 };
            size_t length;
            _get_tzname (&length, name, sizeof (name) - 1, i);
            zone[i] = name;
        }
       #else
        #warning "Can't find a replacement for tzset on mingw - ideas welcome!"
       #endif
      #else
        tzset();

        auto zonePtr = (const char**) tzname;
        zone[0] = zonePtr[0];
        zone[1] = zonePtr[1];
      #endif

        if (isDaylightSavingTime())
        {
            zone[0] = zone[1];

            if (zone[0].length() > 3
                 && zone[0].containsIgnoreCase ("daylight")
                 && zone[0].contains ("GMT"))
                zone[0] = "BST";
        }

        return zone[0].substring (0, 3);
        */
    }
    
    /**
      | Returns the local timezone offset from
      | UTC in seconds.
      |
      */
    pub fn get_utc_offset_seconds(&self) -> i32 {
        
        todo!();
        /*
            return TimeHelpers::getUTCOffsetSeconds (millisSinceEpoch);
        */
    }
    
    /**
      | Returns a string to indicate the offset
      | of the local timezone from UTC.
      | 
      | -----------
      | @param includeDividerCharacters
      | 
      | whether to include or omit the ":" divider
      | in the string
      | 
      | -----------
      | @return
      | 
      | "+XX:XX", "-XX:XX" or "Z"
      |
      */
    pub fn get_utc_offset_string(&self, include_semi_colon: bool) -> String {
        
        todo!();
        /*
            if (auto seconds = getUTCOffsetSeconds())
        {
            auto minutes = seconds / 60;

            return String::formatted (includeSemiColon ? "%+03d:%02d"
                                                       : "%+03d%02d",
                                      minutes / 60,
                                      minutes % 60);
        }

        return "Z";
        */
    }
    
    /**
      | Returns a fully described string of
      | this date and time in ISO-8601 format
      | (using the local timezone).
      | 
      | -----------
      | @param includeDividerCharacters
      | 
      | whether to include or omit the "-" and
      | ":" dividers in the string
      |
      */
    pub fn toiso8601(&self, include_divider_characters: bool) -> String {
        
        todo!();
        /*
            return String::formatted (includeDividerCharacters ? "%04d-%02d-%02dT%02d:%02d:%06.03f"
                                                           : "%04d%02d%02dT%02d%02d%06.03f",
                                  getYear(),
                                  getMonth() + 1,
                                  getDayOfMonth(),
                                  getHours(),
                                  getMinutes(),
                                  getSeconds() + getMilliseconds() / 1000.0)
                + getUTCOffsetString (includeDividerCharacters);
        */
    }
    
    /**
      | Parses an ISO-8601 string and returns
      | it as a Time.
      |
      */
    pub fn fromiso8601(&mut self, iso: &str) -> Time {
        
        todo!();
        /*
            auto t = iso.text;
        auto year = parseFixedSizeIntAndSkip (t, 4, '-');

        if (year < 0)
            return {};

        auto month = parseFixedSizeIntAndSkip (t, 2, '-');

        if (month < 0)
            return {};

        auto day = parseFixedSizeIntAndSkip (t, 2, 0);

        if (day < 0)
            return {};

        int hours = 0, minutes = 0, milliseconds = 0;

        if (*t == 'T')
        {
            ++t;
            hours = parseFixedSizeIntAndSkip (t, 2, ':');

            if (hours < 0)
                return {};

            minutes = parseFixedSizeIntAndSkip (t, 2, ':');

            if (minutes < 0)
                return {};

            auto seconds = parseFixedSizeIntAndSkip (t, 2, 0);

            if (seconds < 0)
                 return {};

            if (*t == '.' || *t == ',')
            {
                ++t;
                milliseconds = parseFixedSizeIntAndSkip (t, 3, 0);

                if (milliseconds < 0)
                    return {};
            }

            milliseconds += 1000 * seconds;
        }

        auto nextChar = t.getAndAdvance();

        if (nextChar == '-' || nextChar == '+')
        {
            auto offsetHours = parseFixedSizeIntAndSkip (t, 2, ':');

            if (offsetHours < 0)
                return {};

            auto offsetMinutes = parseFixedSizeIntAndSkip (t, 2, 0);

            if (offsetMinutes < 0)
                return {};

            auto offsetMs = (offsetHours * 60 + offsetMinutes) * 60 * 1000;
            milliseconds += nextChar == '-' ? offsetMs : -offsetMs; // NB: this seems backwards but is correct!
        }
        else if (nextChar != 0 && nextChar != 'Z')
        {
            return {};
        }

        return Time (year, month - 1, day, hours, minutes, 0, milliseconds, false);
        */
    }
    
    /**
      | Returns the name of the month (in this
      | machine's local timezone).
      | 
      | -----------
      | @param threeLetterVersion
      | 
      | if true, it'll be a 3-letter abbreviation,
      | e.g. "Jan"; if false it'll return the
      | long form, e.g. "January" @see getMonth
      |
      */
    pub fn get_month_name(&self, three_letter_version: bool) -> String {
        
        todo!();
        /*
            return getMonthName (getMonth(), threeLetterVersion);
        */
    }
    
    /**
      | Returns the name of the weekday (in this
      | machine's local timezone).
      | 
      | -----------
      | @param threeLetterVersion
      | 
      | if true, it'll return a 3-letter abbreviation,
      | e.g. "Tue"; if false, it'll return the
      | full version, e.g. "Tuesday".
      |
      */
    pub fn get_weekday_name_from_three_letter_version(&self, three_letter_version: bool) -> String {
        
        todo!();
        /*
            return getWeekdayName (getDayOfWeek(), threeLetterVersion);
        */
    }
    
    /**
      | Returns the name of one of the months.
      | 
      | -----------
      | @param monthNumber
      | 
      | the month, 0 to 11
      | ----------
      | @param threeLetterVersion
      | 
      | if true, it'll be a 3-letter abbreviation,
      | e.g. "Jan"; if false it'll return the
      | long form, e.g. "January"
      |
      */
    pub fn get_month_name_from_month_number(
        &mut self, 
        month_number:         i32,
        three_letter_version: bool
    ) -> String 
    {
        todo!();
        /*
            monthNumber %= 12;

        return TRANS (threeLetterVersion ? shortMonthNames [monthNumber]
                                         : longMonthNames [monthNumber]);
        */
    }
    
    /**
      | Returns the name of a day of the week.
      | 
      | -----------
      | @param dayNumber
      | 
      | the day, 0 to 6 (0 = sunday, 1 = monday,
      | etc)
      | ----------
      | @param threeLetterVersion
      | 
      | if true, it'll return a 3-letter abbreviation,
      | e.g. "Tue"; if false, it'll return the
      | full version, e.g. "Tuesday".
      |
      */
    pub fn get_weekday_name(
        &mut self, 
        day:                  i32,
        three_letter_version: bool) -> String {
        
        todo!();
        /*
            static const char* const shortDayNames[] = { "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat" };
        static const char* const longDayNames[]  = { "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday" };

        day %= 7;

        return TRANS (threeLetterVersion ? shortDayNames [day]
                                         : longDayNames [day]);
        */
    }
    
    /**
      | Returns a Time based on the value of the
      | __DATE__ macro when this module was
      | compiled
      |
      */
    pub fn get_compilation_date(&mut self) -> Time {
        
        todo!();
        /*
            StringArray dateTokens, timeTokens;

        dateTokens.addTokens (__DATE__, true);
        dateTokens.removeEmptyStrings (true);

        timeTokens.addTokens (__TIME__, ":", "");

        return Time (dateTokens[2].getIntValue(),
                     getMonthNumberForCompileDate (dateTokens[0]),
                     dateTokens[1].getIntValue(),
                     timeTokens[0].getIntValue(),
                     timeTokens[1].getIntValue());
        */
    }
}


pub mod time_helpers {
    use super::*;

    pub fn millis_to_local(millis: i64) -> libc::tm {
        
        todo!();
        /*
            #if ALOE_WINDOWS && ALOE_MINGW
                auto now = (time_t) (millis / 1000);
                return *localtime (&now);

               #elif ALOE_WINDOWS
                std::tm result;
                millis /= 1000;

                if (_localtime64_s (&result, &millis) != 0)
                    zerostruct (result);

                return result;

               #else
                std::tm result;
                auto now = (time_t) (millis / 1000);

                if (localtime_r (&now, &result) == nullptr)
                    zerostruct (result);

                return result;
               #endif
        */
    }

    pub fn millis_toutc(millis: i64) -> libc::tm {
        
        todo!();
        /*
            #if ALOE_WINDOWS && ALOE_MINGW
                auto now = (time_t) (millis / 1000);
                return *gmtime (&now);

               #elif ALOE_WINDOWS
                std::tm result;
                millis /= 1000;

                if (_gmtime64_s (&result, &millis) != 0)
                    zerostruct (result);

                return result;

               #else
                std::tm result;
                auto now = (time_t) (millis / 1000);

                if (gmtime_r (&now, &result) == nullptr)
                    zerostruct (result);

                return result;
               #endif
        */
    }

    pub fn get_utc_offset_seconds(millis: i64) -> i32 {
        
        todo!();
        /*
            auto utc = millisToUTC (millis);
                utc.tm_isdst = -1;  // Treat this UTC time as local to find the offset

                return (int) ((millis / 1000) - (int64) mktime (&utc));
        */
    }

    pub fn extended_modulo(
            value:  i64,
            modulo: i32) -> i32 {
        
        todo!();
        /*
            return (int) (value >= 0 ? (value % modulo)
                                         : (value - ((value / modulo) + 1) * modulo));
        */
    }

    pub fn format_string(
            format: &String,
            tm:     *const libc::tm) -> String {
        
        todo!();
        /*
            #if ALOE_ANDROID
                using StringType = CharPointer_UTF8;
               #elif ALOE_WINDOWS
                using StringType = CharPointer_UTF16;
               #else
                using StringType = CharPointer_UTF32;
               #endif

               #ifdef ALOE_MSVC
                if (tm->tm_year < -1900 || tm->tm_year > 8099)
                    return {};   // Visual Studio's library can only handle 0 -> 9999 AD
                #endif

                for (size_t bufferSize = 256; ; bufferSize += 256)
                {
                    HeapBlock<StringType::CharType> buffer (bufferSize);

                    auto numChars =
                               #if ALOE_ANDROID
                                strftime (buffer, bufferSize - 1, format.toUTF8(), tm);
                               #elif ALOE_WINDOWS
                                wcsftime (buffer, bufferSize - 1, format.toWideCharPointer(), tm);
                               #else
                                wcsftime (buffer, bufferSize - 1, format.toUTF32(), tm);
                               #endif

                    if (numChars > 0 || format.isEmpty())
                        return String (StringType (buffer),
                                       StringType (buffer) + (int) numChars);
                }
        */
    }

    pub fn is_leap_year(year: i32) -> bool {
        
        todo!();
        /*
            return (year % 400 == 0) || ((year % 100 != 0) && (year % 4 == 0));
        */
    }

    pub fn days_from_jan1(
            year:  i32,
            month: i32) -> i32 {
        
        todo!();
        /*
            const short dayOfYear[] = { 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334,
                                            0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335 };

                return dayOfYear [(isLeapYear (year) ? 12 : 0) + month];
        */
    }

    pub fn days_from_year0(year: i32) -> i64 {
        
        todo!();
        /*
            --year;
                return 365 * year + (year / 400) - (year / 100) + (year / 4);
        */
    }

    pub fn days_from1970_by_year(year: i32) -> i64 {
        
        todo!();
        /*
            return daysFromYear0 (year) - daysFromYear0 (1970);
        */
    }

    pub fn days_from1970(
            year:  i32,
            month: i32) -> i64 {
        
        todo!();
        /*
            if (month > 11)
                {
                    year += month / 12;
                    month %= 12;
                }
                else if (month < 0)
                {
                    auto numYears = (11 - month) / 12;
                    year -= numYears;
                    month += 12 * numYears;
                }

                return daysFrom1970 (year) + daysFromJan1 (year, month);
        */
    }

    /**
       There's no posix function that does a UTC
       version of mktime, so annoyingly we need to
       implement this manually..
      */
    pub fn mktime_utc(t: &libc::tm) -> i64 {
        
        todo!();
        /*
            return 24 * 3600 * (daysFrom1970 (t.tm_year + 1900, t.tm_mon) + (t.tm_mday - 1))
                        + 3600 * t.tm_hour
                        + 60 * t.tm_min
                        + t.tm_sec;
        */
    }

    lazy_static!{
        /*
        static Atomic<uint32> lastMSCounterValue { (uint32) 0 };
        */
    }
}

#[cfg(target_os="linux")]
pub fn aloe_milliseconds_since_startup() -> u32 {
    
    todo!();
    /*
        return (uint32) (Time::getHighResolutionTicks() / 1000);
    */
}

pub fn parse_fixed_size_int_and_skip(
    t:            &mut CharPointerType,
    num_chars:    i32,
    char_to_skip: u8) -> i32 {
    
    todo!();
    /*
        int n = 0;

        for (int i = numChars; --i >= 0;)
        {
            auto digit = (int) (*t - '0');

            if (! isPositiveAndBelow (digit, 10))
                return -1;

            ++t;
            n = n * 10 + digit;
        }

        if (charToSkip != 0 && *t == (aloe_wchar) charToSkip)
            ++t;

        return n;
    */
}

pub const short_month_names: &[&'static str] = &[ "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec" ];
pub const long_month_names:  &[&'static str] = &[ "January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December" ];

impl Default for Time {
    
    /**
      | Creates a Time object.
      | 
      | This default constructor creates a
      | time of midnight Jan 1st 1970 UTC, (which
      | is represented internally as 0ms).
      | 
      | To create a time object representing
      | the current time, use getCurrentTime().
      | @see getCurrentTime
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl AddAssign<&RelativeTime> for Time {
    
    /**
      | Adds a RelativeTime to this time.
      |
      */
    #[inline] fn add_assign(&mut self, other: &RelativeTime) {
        todo!();
        /*
            millisSinceEpoch += delta.inMilliseconds(); return *this;
        */
    }
}

impl SubAssign<RelativeTime> for Time {
    
    /**
      | Subtracts a RelativeTime from this
      | time.
      |
      */
    #[inline] fn sub_assign(&mut self, delta: RelativeTime) {
        todo!();
        /*
            millisSinceEpoch -= delta.inMilliseconds(); return *this;
        */
    }
}

impl Add<&Time> for Time {

    type Output = Self;

    /**
      | Adds a RelativeTime to a Time.
      |
      */
    #[inline] fn add(self, other: &Time) -> Self::Output {
        todo!();
        /*
            Time t (time); return t += delta;
        */
    }
}

impl Sub<RelativeTime> for Time {

    type Output = Time;

    /**
      | Subtracts a RelativeTime from a Time.
      |
      */
    #[inline] fn sub(self, other: RelativeTime) -> Self::Output {
        todo!();
        /*
            Time t (time); return t -= delta;
        */
    }
}

impl Add<&RelativeTime> for Time {

    type Output = Self;

    /**
      | Adds a RelativeTime to a Time.
      |
      */
    #[inline] fn add(self, other: &RelativeTime) -> Self::Output {
        todo!();
        /*
            Time t (time); return t += delta;
        */
    }
}

impl Sub<Time> for Time {

    type Output = RelativeTime;
    
    /**
      | Returns the relative time difference
      | between two times.
      |
      */
    #[inline] fn sub(self, other: Time) -> Self::Output {
        todo!();
        /*
            return RelativeTime::milliseconds (time1.toMilliseconds() - time2.toMilliseconds());
        */
    }
}

impl PartialEq<Time> for Time {
    
    #[inline] fn eq(&self, other: &Time) -> bool {
        todo!();
        /*
            return time1.toMilliseconds() == time2.toMilliseconds();
        */
    }
}

impl Eq for Time {}

impl Ord for Time {
    
    #[inline] fn cmp(&self, other: &Time) -> std::cmp::Ordering {
        todo!();
        /*
            return time1.toMilliseconds() <  time2.toMilliseconds();
        */
    }
}

impl PartialOrd<Time> for Time {
    #[inline] fn partial_cmp(&self, other: &Time) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn get_month_number_for_compile_date(m: &String) -> i32 {
    
    todo!();
    /*
        for (int i = 0; i < 12; ++i)
            if (m.equalsIgnoreCase (shortMonthNames[i]))
                return i;

        // If you hit this because your compiler has an unusual __DATE__
        // format, let us know so we can add support for it!
        jassertfalse;
        return 0;
    */
}

#[cfg(target_arch = "wasm32")]
impl Time {
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_high_resolution_ticks(&mut self) -> i64 {
        
        todo!();
        /*
            return static_cast<int64> (emscripten_get_now() * 1000.0);
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_high_resolution_ticks_per_second(&mut self) -> i64 {
        
        todo!();
        /*
            return 1000000;  // (microseconds)
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_millisecond_counter_hi_res(&mut self) -> f64 {
        
        todo!();
        /*
            return emscripten_get_now();
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn set_system_time_to_this_time(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

#[cfg(target_os="linux")]
impl Time {
    
    #[cfg(target_os="linux")]
    pub fn get_high_resolution_ticks(&mut self) -> i64 {
        
        todo!();
        /*
            timespec t;

       #if ALOE_BELA
        if (cobalt_thread_mode() == 0x200 /*XNRELAX*/)
            clock_gettime (CLOCK_MONOTONIC, &t);
        else
            __wrap_clock_gettime (CLOCK_MONOTONIC, &t);
       #else
        clock_gettime (CLOCK_MONOTONIC, &t);
       #endif

        return (t.tv_sec * (int64) 1000000) + (t.tv_nsec / 1000);
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_high_resolution_ticks_per_second(&mut self) -> i64 {
        
        todo!();
        /*
            return 1000000;  // (microseconds)
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_millisecond_counter_hi_res(&mut self) -> f64 {
        
        todo!();
        /*
            return (double) getHighResolutionTicks() * 0.001;
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn set_system_time_to_this_time(&self) -> bool {
        
        todo!();
        /*
            timeval t;
        t.tv_sec = decltype (timeval::tv_sec) (millisSinceEpoch / 1000);
        t.tv_usec = decltype (timeval::tv_usec) ((millisSinceEpoch - t.tv_sec * 1000) * 1000);

        return settimeofday (&t, nullptr) == 0;
        */
    }
}
