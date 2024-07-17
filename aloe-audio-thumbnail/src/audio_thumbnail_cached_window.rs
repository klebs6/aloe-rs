crate::ix!();

pub struct AudioThumbnailCachedWindow {
    data:                  Vec<AudioThumbnailMinMaxValue>,
    cached_start:          f64, // default = 0
    cached_time_per_pixel: f64, // default = 0
    num_channels_cached:   i32, // default = 0
    num_samples_cached:    i32, // default = 0
    cache_needs_refilling: bool, // default = true
}

impl AudioThumbnailCachedWindow {

    pub fn invalidate(&mut self)  {
        
        todo!();
        /*
            cacheNeedsRefilling = true;
        */
    }
    
    pub fn draw_channel(
        &mut self, 
        g:                      &mut Graphics,
        area:                   &Rectangle<i32>,
        start_time:             f64,
        end_time:               f64,
        channel_num:            i32,
        vertical_zoom_factor:   f32,
        rate:                   f64,
        num_chans:              i32,
        samps_per_thumb_sample: i32,
        level_data:             *mut AudioThumbnailLevelDataSource,
        chans:                  &Vec<Box<AudioThumbnailThumbData>>

    ) {
        
        todo!();
        /*
            if (refillCache (area.getWidth(), startTime, endTime, rate,
                                 numChans, sampsPerThumbSample, levelData, chans)
                     && isPositiveAndBelow (channelNum, numChannelsCached))
                {
                    auto clip = g.getClipBounds().getIntersection (area.withWidth (jmin (numSamplesCached, area.getWidth())));

                    if (! clip.isEmpty())
                    {
                        auto topY = (float) area.getY();
                        auto bottomY = (float) area.getBottom();
                        auto midY = (topY + bottomY) * 0.5f;
                        auto vscale = verticalZoomFactor * (bottomY - topY) / 256.0f;

                        auto* cacheData = getData (channelNum, clip.getX() - area.getX());

                        RectangleList<float> waveform;
                        waveform.ensureStorageAllocated (clip.getWidth());

                        auto x = (float) clip.getX();

                        for (int w = clip.getWidth(); --w >= 0;)
                        {
                            if (cacheData->isNonZero())
                            {
                                auto top    = jmax (midY - cacheData->getMaxValue() * vscale - 0.3f, topY);
                                auto bottom = jmin (midY - cacheData->getMinValue() * vscale + 0.3f, bottomY);

                                waveform.addWithoutMerging (Rectangle<float> (x, top, 1.0f, bottom - top));
                            }

                            x += 1.0f;
                            ++cacheData;
                        }

                        g.fillRectList (waveform);
                    }
                }
        */
    }
    
    pub fn refill_cache(
        &mut self, 
        num_samples:            i32,
        start_time:             f64,
        end_time:               f64,
        rate:                   f64,
        num_chans:              i32,
        samps_per_thumb_sample: i32,
        level_data:             *mut AudioThumbnailLevelDataSource,
        chans:                  &Vec<Box<AudioThumbnailThumbData>>

    ) -> bool {
        
        todo!();
        /*
            auto timePerPixel = (endTime - startTime) / numSamples;

                if (numSamples <= 0 || timePerPixel <= 0.0 || rate <= 0)
                {
                    invalidate();
                    return false;
                }

                if (numSamples == numSamplesCached
                     && numChannelsCached == numChans
                     && startTime == cachedStart
                     && timePerPixel == cachedTimePerPixel
                     && ! cacheNeedsRefilling)
                {
                    return ! cacheNeedsRefilling;
                }

                numSamplesCached = numSamples;
                numChannelsCached = numChans;
                cachedStart = startTime;
                cachedTimePerPixel = timePerPixel;
                cacheNeedsRefilling = false;

                ensureSize (numSamples);

                if (timePerPixel * rate <= sampsPerThumbSample && levelData != nullptr)
                {
                    auto sample = roundToInt (startTime * rate);
                    Vec<Range<float>> levels;

                    int i;
                    for (i = 0; i < numSamples; ++i)
                    {
                        auto nextSample = roundToInt ((startTime + timePerPixel) * rate);

                        if (sample >= 0)
                        {
                            if (sample >= levelData->lengthInSamples)
                            {
                                for (int chan = 0; chan < numChannelsCached; ++chan)
                                    *getData (chan, i) = AudioThumbnailMinMaxValue();
                            }
                            else
                            {
                                levelData->getLevels (sample, jmax (1, nextSample - sample), levels);

                                auto totalChans = jmin (levels.size(), numChannelsCached);

                                for (int chan = 0; chan < totalChans; ++chan)
                                    getData (chan, i)->setFloat (levels.getReference (chan));
                            }
                        }

                        startTime += timePerPixel;
                        sample = nextSample;
                    }

                    numSamplesCached = i;
                }
                else
                {
                    jassert (chans.size() == numChannelsCached);

                    for (int channelNum = 0; channelNum < numChannelsCached; ++channelNum)
                    {
                        AudioThumbnailThumbData* channelData = chans.getUnchecked (channelNum);
                        AudioThumbnailMinMaxValue* cacheData = getData (channelNum, 0);

                        auto timeToThumbSampleFactor = rate / (double) sampsPerThumbSample;

                        startTime = cachedStart;
                        auto sample = roundToInt (startTime * timeToThumbSampleFactor);

                        for (int i = numSamples; --i >= 0;)
                        {
                            auto nextSample = roundToInt ((startTime + timePerPixel) * timeToThumbSampleFactor);

                            channelData->getMinMax (sample, nextSample, *cacheData);

                            ++cacheData;
                            startTime += timePerPixel;
                            sample = nextSample;
                        }
                    }
                }

                return true;
        */
    }
    
    pub fn get_data(&mut self, 
        channel_num: i32,
        cache_index: i32) -> *mut AudioThumbnailMinMaxValue {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channelNum, numChannelsCached) && isPositiveAndBelow (cacheIndex, data.size()));

                return data.getRawDataPointer() + channelNum * numSamplesCached
                                                + cacheIndex;
        */
    }
    
    pub fn ensure_size(&mut self, num_samples: i32)  {
        
        todo!();
        /*
            auto itemsRequired = numSamples * numChannelsCached;

                if (data.size() < itemsRequired)
                    data.insertMultiple (-1, AudioThumbnailMinMaxValue(), itemsRequired - data.size());
        */
    }
}
