crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_linux_Threads.cpp]

pub fn swap_user_and_effective_user() -> bool {
    
    todo!();
    /*
        auto result1 = setreuid (geteuid(), getuid());
        auto result2 = setregid (getegid(), getgid());
        return result1 == 0 && result2 == 0;
    */
}

/**
  | @note
  | 
  | a lot of methods that you'd expect to
  | find in this file actually live in aloe_posix_SharedCode.h!
  |
  */
impl Process {
    
    /**
      | Changes the current process's priority.
      | 
      | -----------
      | @param priority
      | 
      | the process priority, where 0=low,
      | 1=normal, 2=high, 3=realtime
      |
      */
    pub fn set_priority(&mut self, prior: ProcessPriority)  {
        
        todo!();
        /*
            auto policy = (prior <= NormalPriority) ? SCHED_OTHER : SCHED_RR;
        auto minp = sched_get_priority_min (policy);
        auto maxp = sched_get_priority_max (policy);

        struct sched_param param;

        switch (prior)
        {
            case LowPriority:
            case NormalPriority:    param.sched_priority = 0; break;
            case HighPriority:      param.sched_priority = minp + (maxp - minp) / 4; break;
            case RealtimePriority:  param.sched_priority = minp + (3 * (maxp - minp) / 4); break;
            default:                jassertfalse; break;
        }

        pthread_setschedparam (pthread_self(), policy, &param);
        */
    }
}
