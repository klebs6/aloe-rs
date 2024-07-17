/*!
  | function: single-block PCM analysis
  | mode dispatch
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/analysis.c]

/** 
  decides between modes, dispatches to the
  appropriate mapping. 
  */
pub fn vorbis_analysis(
        vb: *mut VorbisBlock,
        op: *mut OggPacket) -> i32 {
    
    todo!();
    /*
        int ret,i;
      vorbis_block_internal *vbi=(vorbis_block_internal *)vb->internal;

      vb->glue_bits=0;
      vb->time_bits=0;
      vb->floor_bits=0;
      vb->res_bits=0;

      /* first things first.  Make sure encode is ready */
      for(i=0;i<PACKETBLOBS;i++)
        oggpack_reset(vbi->packetblob[i]);

      /* we only have one mapping type (0), and we let the mapping code
         itself figure out what soft mode to use.  This allows easier
         bitrate management */

      if((ret=_mapping_P[0]->forward(vb)))
        return(ret);

      if(op){
        if(vorbis_bitrate_managed(vb))
          /* The app is using a bitmanaged mode... but not using the
             bitrate management interface. */
          return(OV_EINVAL);

        op->packet=oggpack_get_buffer(&vb->opb);
        op->bytes=oggpack_bytes(&vb->opb);
        op->b_o_s=0;
        op->e_o_s=vb->eofflag;
        op->granulepos=vb->granulepos;
        op->packetno=vb->sequence; /* for sake of completeness */
      }
      return(0);
    */
}

#[cfg(ANALYSIS)]
lazy_static!{
    /*
    int analysis_noisy=1;
    */
}

/** 
  there was no great place to put this.... 
  */
#[cfg(ANALYSIS)]
pub fn analysis_output_always(
        base: *mut u8,
        i:    i32,
        v:    *mut f32,
        n:    i32,
        bark: i32,
        db:   i32,
        off:  i64)  {
    
    todo!();
    /*
        int j;
      FILE *of;
      char buffer[80];

      sprintf(buffer,"%s_%d.m",base,i);
      of=fopen(buffer,"w");

      if(!of)perror("failed to open data dump file");

      for(j=0;j<n;j++){
        if(bark){
          float b=toBARK((4000.f*j/n)+.25);
          fprintf(of,"%f ",b);
        }else
          if(off!=0)
            fprintf(of,"%f ",(double)(j+off)/8000.);
          else
            fprintf(of,"%f ",(double)j);

        if(dB){
          float val;
          if(v[j]==0.)
            val=-140.;
          else
            val=todB(v+j);
          fprintf(of,"%f\n",val);
        }else{
          fprintf(of,"%f\n",v[j]);
        }
      }
      fclose(of);
    */
}

#[cfg(ANALYSIS)]
pub fn analysis_output(
        base: *mut u8,
        i:    i32,
        v:    *mut f32,
        n:    i32,
        bark: i32,
        db:   i32,
        off:  i64)  {
    
    todo!();
    /*
        if(analysis_noisy)_analysis_output_always(base,i,v,n,bark,dB,off);
    */
}
