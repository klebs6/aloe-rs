crate::ix!();

pub struct AudioThumbnailLevelDataSource<'a> {
    base:                 TimeSliceClient,
    length_in_samples:    i64, // default = 0
    num_samples_finished: i64, // default = 0
    sample_rate:          f64, // default = 0
    num_channels:         u32, // default = 0
    hash_code:            i64, // default = 0
    owner:                &'a mut AudioThumbnail<'a>,
    source:               Box<dyn Read>,
    reader:               Box<AudioFormatReader<'a>>,
    reader_lock:          CriticalSection,
    last_reader_use_time: Atomic<u32>, // default = 0 
}

pub const LEVEL_DATA_SOURCE_TIME_BEFORE_DELETING_READER: usize = 3000;

impl<'a> Drop for AudioThumbnailLevelDataSource<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                owner.cache.getTimeSliceThread().removeTimeSliceClient (this);
             */
    }
}

impl<'a> AudioThumbnailLevelDataSource<'a> {

    pub fn new_with_hash(
        thumb:      &mut AudioThumbnail,
        new_reader: *mut AudioFormatReader,
        hash:       i64

    ) -> Self {
    
        todo!();
        /*
        : hash_code(hash),
        : owner(thumb),
        : reader(newReader),

        
        */
    }
    
    pub fn new(
        thumb: &mut AudioThumbnail,
        src:   &mut dyn Read

    ) -> Self {
    
        todo!();
        /*


            : hashCode (src->hashCode()), owner (thumb), source (src)
        */
    }
    
    pub fn initialise(&mut self, samples_finished: i64)  {
        
        todo!();
        /*
            const ScopedLock sl (readerLock);

                numSamplesFinished = samplesFinished;

                createReader();

                if (reader != nullptr)
                {
                    lengthInSamples = reader->lengthInSamples;
                    numChannels = reader->numChannels;
                    sampleRate = reader->sampleRate;

                    if (lengthInSamples <= 0 || isFullyLoaded())
                        reader.reset();
                    else
                        owner.cache.getTimeSliceThread().addTimeSliceClient (this);
                }
        */
    }
    
    pub fn get_levels(&mut self, 
        start_sample: i64,
        num_samples:  i32,
        levels:       &mut Vec<Range<f32>>)  {
        
        todo!();
        /*
            const ScopedLock sl (readerLock);

                if (reader == nullptr)
                {
                    createReader();

                    if (reader != nullptr)
                    {
                        lastReaderUseTime = Time::getMillisecondCounter();
                        owner.cache.getTimeSliceThread().addTimeSliceClient (this);
                    }
                }

                if (reader != nullptr)
                {
                    if (levels.size() < (int) reader->numChannels)
                        levels.insertMultiple (0, {}, (int) reader->numChannels - levels.size());

                    reader->readMaxLevels (startSample, numSamples, levels.getRawDataPointer(), (int) reader->numChannels);

                    lastReaderUseTime = Time::getMillisecondCounter();
                }
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (readerLock);
                reader.reset();
        */
    }
    
    pub fn use_time_slice(&mut self) -> i32 {
        
        todo!();
        /*
            if (isFullyLoaded())
                {
                    if (reader != nullptr && source != nullptr)
                    {
                        if (Time::getMillisecondCounter() > lastReaderUseTime + timeBeforeDeletingReader)
                            releaseResources();
                        else
                            return 200;
                    }

                    return -1;
                }

                bool justFinished = false;

                {
                    const ScopedLock sl (readerLock);
                    createReader();

                    if (reader != nullptr)
                    {
                        if (! readNextBlock())
                            return 0;

                        justFinished = true;
                    }
                }

                if (justFinished)
                    owner.cache.storeThumb (owner, hashCode);

                return 200;
        */
    }
    
    pub fn is_fully_loaded(&self) -> bool {
        
        todo!();
        /*
            return numSamplesFinished >= lengthInSamples;
        */
    }
    
    #[inline] pub fn sample_to_thumb_sample(&self, original_sample: i64) -> i32 {
        
        todo!();
        /*
            return (int) (originalSample / owner.samplesPerThumbSample);
        */
    }
    
    pub fn create_reader(&mut self)  {
        
        todo!();
        /*
            if (reader == nullptr && source != nullptr)
                    if (auto* audioFileStream = source->createInputStream())
                        reader.reset (owner.formatManagerToUse.createReaderFor (std::unique_ptr<InputStream> (audioFileStream)));
        */
    }
    
    pub fn read_next_block(&mut self) -> bool {
        
        todo!();
        /*
            jassert (reader != nullptr);

                if (! isFullyLoaded())
                {
                    auto numToDo = (int) jmin (256 * (int64) owner.samplesPerThumbSample, lengthInSamples - numSamplesFinished);

                    if (numToDo > 0)
                    {
                        auto startSample = numSamplesFinished;

                        auto firstThumbIndex = sampleToThumbSample (startSample);
                        auto lastThumbIndex  = sampleToThumbSample (startSample + numToDo);
                        auto numThumbSamps = lastThumbIndex - firstThumbIndex;

                        HeapBlock<AudioThumbnailMinMaxValue> levelData ((unsigned int) numThumbSamps * numChannels);
                        HeapBlock<AudioThumbnailMinMaxValue*> levels (numChannels);

                        for (int i = 0; i < (int) numChannels; ++i)
                            levels[i] = levelData + i * numThumbSamps;

                        HeapBlock<Range<float>> levelsRead (numChannels);

                        for (int i = 0; i < numThumbSamps; ++i)
                        {
                            reader->readMaxLevels ((firstThumbIndex + i) * owner.samplesPerThumbSample,
                                                   owner.samplesPerThumbSample, levelsRead, (int) numChannels);

                            for (int j = 0; j < (int) numChannels; ++j)
                                levels[j][i].setFloat (levelsRead[j]);
                        }

                        {
                            const ScopedUnlock su (readerLock);
                            owner.setLevels (levels, firstThumbIndex, (int) numChannels, numThumbSamps);
                        }

                        numSamplesFinished += numToDo;
                        lastReaderUseTime = Time::getMillisecondCounter();
                    }
                }

                return isFullyLoaded();
        */
    }
}
