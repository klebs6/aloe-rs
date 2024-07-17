crate::ix!();

pub fn midi_file_convert_ticks_to_seconds(
        time:         f64,
        tempo_events: &MidiMessageSequence,
        time_format:  i32) -> f64 {
    
    todo!();
    /*
        if (timeFormat < 0)
                return time / (-(timeFormat >> 8) * (timeFormat & 0xff));

            double lastTime = 0, correctedTime = 0;
            auto tickLen = 1.0 / (timeFormat & 0x7fff);
            auto secsPerTick = 0.5 * tickLen;
            auto numEvents = tempoEvents.getNumEvents();

            for (int i = 0; i < numEvents; ++i)
            {
                auto& m = tempoEvents.getEventPointer(i)->message;
                auto eventTime = m.getTimeStamp();

                if (eventTime >= time)
                    break;

                correctedTime += (eventTime - lastTime) * secsPerTick;
                lastTime = eventTime;

                if (m.isTempoMetaEvent())
                    secsPerTick = tickLen * m.getTempoSecondsPerQuarterNote();

                while (i + 1 < numEvents)
                {
                    auto& m2 = tempoEvents.getEventPointer(i + 1)->message;

                    if (m2.getTimeStamp() != eventTime)
                        break;

                    if (m2.isTempoMetaEvent())
                        secsPerTick = tickLen * m2.getTempoSecondsPerQuarterNote();

                    ++i;
                }
            }

            return correctedTime + (time - lastTime) * secsPerTick;
    */
}

pub fn midi_file_find_all_matching_events<MethodType>(
        tracks:  &Vec<Box<MidiMessageSequence>>,
        results: &mut MidiMessageSequence,
        method:  MethodType)  {

    todo!();
    /*
        for (auto* track : tracks)
            {
                auto numEvents = track->getNumEvents();

                for (int j = 0; j < numEvents; ++j)
                {
                    auto& m = track->getEventPointer(j)->message;

                    if ((m.*method)())
                        results.addEvent (m);
                }
            }
    */
}

