crate::ix!();

pub fn to_process_context(
    context:     &mut ProcessContext,
    play_head:   *mut dyn AudioPlayHeadInterface,
    sample_rate: f64

) {
    
    todo!();
        /*
            jassert (sampleRate > 0.0); //Must always be valid, as stated by the Vst3 SDK

        using namespace Vst;

        zerostruct (context);
        context.sampleRate = sampleRate;
        auto& fr = context.frameRate;

        if (playHead != nullptr)
        {
            AudioPlayHeadCurrentPositionInfo position;
            playHead->getCurrentPosition (position);

            context.projectTimeSamples  = position.timeInSamples; // Must always be valid, as stated by the Vst3 SDK
            context.projectTimeMusic    = position.ppqPosition;   // Does not always need to be valid...
            context.tempo               = position.bpm;
            context.timeSigNumerator    = position.timeSigNumerator;
            context.timeSigDenominator  = position.timeSigDenominator;
            context.barPositionMusic    = position.ppqPositionOfLastBarStart;
            context.cycleStartMusic     = position.ppqLoopStart;
            context.cycleEndMusic       = position.ppqLoopEnd;

            switch (position.frameRate)
            {
                case AudioPlayHead::fps23976:    fr.framesPerSecond = 24; fr.flags = FrameRate::kPullDownRate; break;
                case AudioPlayHead::fps24:       fr.framesPerSecond = 24; fr.flags = 0; break;
                case AudioPlayHead::fps25:       fr.framesPerSecond = 25; fr.flags = 0; break;
                case AudioPlayHead::fps2997:     fr.framesPerSecond = 30; fr.flags = FrameRate::kPullDownRate; break;
                case AudioPlayHead::fps2997drop: fr.framesPerSecond = 30; fr.flags = FrameRate::kPullDownRate | FrameRate::kDropRate; break;
                case AudioPlayHead::fps30:       fr.framesPerSecond = 30; fr.flags = 0; break;
                case AudioPlayHead::fps30drop:   fr.framesPerSecond = 30; fr.flags = FrameRate::kDropRate; break;
                case AudioPlayHead::fps60:       fr.framesPerSecond = 60; fr.flags = 0; break;
                case AudioPlayHead::fps60drop:   fr.framesPerSecond = 60; fr.flags = FrameRate::kDropRate; break;
                case AudioPlayHead::fpsUnknown:  break;
                default:                         jassertfalse; break; // New frame rate?
            }

            if (position.isPlaying)     context.state |= ProcessContext::kPlaying;
            if (position.isRecording)   context.state |= ProcessContext::kRecording;
            if (position.isLooping)     context.state |= ProcessContext::kCycleActive;
        }
        else
        {
            context.tempo               = 120.0;
            context.timeSigNumerator    = 4;
            context.timeSigDenominator  = 4;
            fr.framesPerSecond          = 30;
            fr.flags                    = 0;
        }

        if (context.projectTimeMusic >= 0.0)        context.state |= ProcessContext::kProjectTimeMusicValid;
        if (context.barPositionMusic >= 0.0)        context.state |= ProcessContext::kBarPositionValid;
        if (context.tempo > 0.0)                    context.state |= ProcessContext::kTempoValid;
        if (context.frameRate.framesPerSecond > 0)  context.state |= ProcessContext::kSmpteValid;

        if (context.cycleStartMusic >= 0.0
             && context.cycleEndMusic > 0.0
             && context.cycleEndMusic > context.cycleStartMusic)
        {
            context.state |= ProcessContext::kCycleValid;
        }

        if (context.timeSigNumerator > 0 && context.timeSigDenominator > 0)
            context.state |= ProcessContext::kTimeSigValid;
        */
}
