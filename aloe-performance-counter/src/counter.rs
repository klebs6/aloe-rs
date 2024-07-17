crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/time/aloe_PerformanceCounter.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_core/time/aloe_PerformanceCounter.cpp]

/** 
  | A timer for measuring performance of code and
  | dumping the results to a file.
  |
  |    e.g. @code
  |
  |        PerformanceCounter pc ("fish", 50, "/temp/myfishlog.txt");
  |
  |        for (;;)
  |        {
  |            pc.start();
  |
  |            doSomethingFishy();
  |
  |            pc.stop();
  |        }
  |    @endcode
  |
  | In this example, the time of each period
  | between calling start/stop will be measured
  | and averaged over 50 runs, and the results
  | printed to a file every 50 times round the
  | loop.
  |
  | @tags{Core}
  */
#[no_copy]
#[leak_detector]
pub struct PerformanceCounter {
    stats:          performance_counter::Statistics,
    runs_per_print: i64,
    start_time:     i64,
    output_file:    File,
}

pub mod performance_counter {

    use super::*;

    /**
      | Holds the current statistics.
      |
      */
    #[derive(Default)]
    pub struct Statistics
    {
        name:            String,
        average_seconds: f64,
        maximum_seconds: f64,
        minimum_seconds: f64,
        total_seconds:   f64,
        num_runs:        i64,
    }

    impl Statistics {
        
        pub fn new() -> Self {
        
            todo!();
            /*
            : average_seconds(),
            : maximum_seconds(),
            : minimum_seconds(),
            : total_seconds(),
            : num_runs(),

            
            */
        }
        
        pub fn clear(&mut self)  {
            
            todo!();
            /*
                averageSeconds = maximumSeconds = minimumSeconds = totalSeconds = 0;
            numRuns = 0;
            */
        }
        
        pub fn add_result(&mut self, elapsed: f64)  {
            
            todo!();
            /*
                if (numRuns == 0)
            {
                maximumSeconds = elapsed;
                minimumSeconds = elapsed;
            }
            else
            {
                maximumSeconds = jmax (maximumSeconds, elapsed);
                minimumSeconds = jmin (minimumSeconds, elapsed);
            }

            ++numRuns;
            totalSeconds += elapsed;
            */
        }
        
        pub fn to_string(&self) -> String {
            
            todo!();
            /*
                MemoryOutputStream s;

            s << "Performance count for \"" << name << "\" over " << numRuns << " run(s)" << newLine
              << "Average = "   << timeToString (averageSeconds)
              << ", minimum = " << timeToString (minimumSeconds)
              << ", maximum = " << timeToString (maximumSeconds)
              << ", total = "   << timeToString (totalSeconds);

            return s.toString();
            */
        }
    }
}

impl Drop for PerformanceCounter {

    fn drop(&mut self) {
        todo!();
        /* 
        if (stats.numRuns > 0)
            printStatistics();
         */
    }
}

impl PerformanceCounter {
    
    /**
      | Creates a PerformanceCounter object.
      | 
      | -----------
      | @param counterName
      | 
      | the name used when printing out the statistics
      | ----------
      | @param runsPerPrintout
      | 
      | the number of start/stop iterations
      | before calling printStatistics()
      | ----------
      | @param loggingFile
      | 
      | a file to dump the results to - if this
      | is File(), the results are just written
      | to the debugger output
      |
      */
    pub fn new(
        name:              &String,
        runs_per_printout: Option<i32>,
        logging_file:      Option<&File>) -> Self {

        let runs_per_printout: i32 =
            runs_per_printout.unwrap_or(100);

        let logging_file = logging_file.unwrap_or(&File::default());
    
        todo!();
        /*
        : runs_per_print(runsPerPrintout),
        : start_time(0),
        : output_file(loggingFile),

            stats.name = name;
        appendToFile (outputFile, "**** Counter for \"" + name + "\" started at: " + Time::getCurrentTime().toString (true, true));
        */
    }

    /**
      | Starts timing. @see stop
      |
      */
    pub fn start(&mut self)  {
        
        todo!();
        /*
            startTime = Time::getHighResolutionTicks();
        */
    }
    
    /**
      | Stops timing and prints out the results.
      | 
      | The number of iterations before doing
      | a printout of the results is set in the
      | constructor.
      | 
      | @see start
      |
      */
    pub fn stop(&mut self) -> bool {
        
        todo!();
        /*
            stats.addResult (Time::highResolutionTicksToSeconds (Time::getHighResolutionTicks() - startTime));

        if (stats.numRuns < runsPerPrint)
            return false;

        printStatistics();
        return true;
        */
    }
    
    /**
      | Dumps the current metrics to the debugger
      | output and to a file.
      | 
      | As well as using Logger::outputDebugString
      | to print the results, this will write
      | then to the file specified in the constructor
      | (if this was valid).
      |
      */
    pub fn print_statistics(&mut self)  {
        
        todo!();
        /*
            const String desc (getStatisticsAndReset().toString());

        Logger::writeToLog (desc);
        appendToFile (outputFile, desc);
        */
    }
    
    /**
      | Returns a copy of the current stats,
      | and resets the internal counter.
      |
      */
    pub fn get_statistics_and_reset(&mut self) -> performance_counter::Statistics {
        
        todo!();
        /*
            Statistics s (stats);
        stats.clear();

        if (s.numRuns > 0)
            s.averageSeconds = s.totalSeconds / (float) s.numRuns;

        return s;
        */
    }
}

pub fn append_to_file(
        f: &File,
        s: &String)  {
    
    todo!();
    /*
        if (f.getFullPathName().isNotEmpty())
        {
            FileOutputStream out (f);

            if (! out.failedToOpen())
                out << s << newLine;
        }
    */
}
