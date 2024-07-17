crate::ix!();

#[cfg(target_os="android")]
impl Process {
    
    pub fn open_email_with_attachments(&mut self, 
        target_email_address: &String,
        email_subject:        &String,
        body_text:            &String,
        files_to_attach:      &Vec<String>) -> bool {
        
        todo!();
        /*
            // TODO
        return false;
        */
    }

    /**
      | sets the process to 0=low priority,
      | 1=normal, 2=high, 3=realtime
      |
      */
    pub fn set_priority(&mut self, prior: ProcessPriority)  {
        
        todo!();
        /*
            // TODO

        struct sched_param param;
        int policy, maxp, minp;

        const int p = (int) prior;

        if (p <= 1)
            policy = SCHED_OTHER;
        else
            policy = SCHED_RR;

        minp = sched_get_priority_min (policy);
        maxp = sched_get_priority_max (policy);

        if (p < 2)
            param.sched_priority = 0;
        else if (p == 2 )
            // Set to middle of lower realtime priority range
            param.sched_priority = minp + (maxp - minp) / 4;
        else
            // Set to middle of higher realtime priority range
            param.sched_priority = minp + (3 * (maxp - minp) / 4);

        pthread_setschedparam (pthread_self(), policy, &param);
        */
    }
    
    pub fn raise_privilege(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn lower_privilege(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
