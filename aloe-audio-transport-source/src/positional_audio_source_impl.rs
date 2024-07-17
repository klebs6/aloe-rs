crate::ix!();

impl<'a> PositionableAudioSource for AudioTransportSource<'a> {

    /**
      | Implements the PositionableAudioSource
      | method.
      |
      */
    fn set_next_read_position(&mut self, new_position: i64)  {
        
        todo!();
        /*
            if (positionableSource != nullptr)
        {
            if (sampleRate > 0 && sourceSampleRate > 0)
                newPosition = (int64) ((double) newPosition * sourceSampleRate / sampleRate);

            positionableSource->setNextReadPosition (newPosition);

            if (resamplerSource != nullptr)
                resamplerSource->flushBuffers();

            inputStreamEOF = false;
        }
        */
    }

    /**
      | Implements the PositionableAudioSource
      | method.
      |
      */
    fn get_next_read_position(&self) -> i64 {
        
        todo!();
        /*
            if (positionableSource != nullptr)
        {
            const double ratio = (sampleRate > 0 && sourceSampleRate > 0) ? sampleRate / sourceSampleRate : 1.0;
            return (int64) ((double) positionableSource->getNextReadPosition() * ratio);
        }

        return 0;
        */
    }

    /**
      | Implements the PositionableAudioSource
      | method.
      |
      */
    fn get_total_length(&self) -> i64 {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

        if (positionableSource != nullptr)
        {
            const double ratio = (sampleRate > 0 && sourceSampleRate > 0) ? sampleRate / sourceSampleRate : 1.0;
            return (int64) ((double) positionableSource->getTotalLength() * ratio);
        }

        return 0;
        */
    }

    /**
      | Implements the PositionableAudioSource
      | method.
      |
      */
    fn is_looping(&self) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);
        return positionableSource != nullptr && positionableSource->isLooping();
        */
    }
}
