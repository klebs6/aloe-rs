/*!
 function: single-block PCM synthesis
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/synthesis.c]

pub fn vorbis_synthesis(
        vb: *mut VorbisBlock,
        op: *mut OggPacket) -> i32 {
    
    todo!();
        /*
            vorbis_dsp_state     *vd= vb ? vb->vd : 0;
      private_state        *b= vd ? (private_state*)vd->backend_state : 0;
      vorbis_info          *vi= vd ? vd->vi : 0;
      codec_setup_info     *ci= vi ? (codec_setup_info*)vi->codec_setup : 0;
      oggpack_buffer       *opb=vb ? &vb->opb : 0;
      int                   type,mode,i;

      if (!vd || !b || !vi || !ci || !opb) {
        return OV_EBADPACKET;
      }

      /* first things first.  Make sure decode is ready */
      _vorbis_block_ripcord(vb);
      oggpack_readinit(opb,op->packet,op->bytes);

      /* Check the packet type */
      if(oggpack_read(opb,1)!=0){
        /* Oops.  This is not an audio data packet */
        return(OV_ENOTAUDIO);
      }

      /* read our mode and pre/post windowsize */
      mode=oggpack_read(opb,b->modebits);
      if(mode==-1){
        return(OV_EBADPACKET);
      }

      vb->mode=mode;
      if(!ci->mode_param[mode]){
        return(OV_EBADPACKET);
      }

      vb->W=ci->mode_param[mode]->blockflag;
      if(vb->W){

        /* this doesn;t get mapped through mode selection as it's used
           only for window selection */
        vb->lW=oggpack_read(opb,1);
        vb->nW=oggpack_read(opb,1);
        if(vb->nW==-1){
          return(OV_EBADPACKET);
        }
      }else{
        vb->lW=0;
        vb->nW=0;
      }

      /* more setup */
      vb->granulepos=op->granulepos;
      vb->sequence=op->packetno;
      vb->eofflag=op->e_o_s;

      /* alloc pcm passback storage */
      vb->pcmend=ci->blocksizes[vb->W];
      vb->pcm=(float**)_vorbis_block_alloc(vb,sizeof(*vb->pcm)*vi->channels);
      for(i=0;i<vi->channels;i++)
        vb->pcm[i]=(float*)_vorbis_block_alloc(vb,vb->pcmend*sizeof(*vb->pcm[i]));

      /* unpack_header enforces range checking */
      type=ci->map_type[ci->mode_param[mode]->mapping];

      return(_mapping_P[type]->inverse(vb,ci->map_param[ci->mode_param[mode]->
                                                       mapping]));
        */
}

/**
  | used to track pcm position without actually
  | performing decode.
  | 
  | Useful for sequential 'fast forward'
  |
  */
pub fn vorbis_synthesis_trackonly(
        vb: *mut VorbisBlock,
        op: *mut OggPacket) -> i32 {
    
    todo!();
        /*
            vorbis_dsp_state     *vd=vb->vd;
      private_state        *b=(private_state*)vd->backend_state;
      vorbis_info          *vi=vd->vi;
      codec_setup_info     *ci=(codec_setup_info*)vi->codec_setup;
      oggpack_buffer       *opb=&vb->opb;
      int                   mode;

      /* first things first.  Make sure decode is ready */
      _vorbis_block_ripcord(vb);
      oggpack_readinit(opb,op->packet,op->bytes);

      /* Check the packet type */
      if(oggpack_read(opb,1)!=0){
        /* Oops.  This is not an audio data packet */
        return(OV_ENOTAUDIO);
      }

      /* read our mode and pre/post windowsize */
      mode=oggpack_read(opb,b->modebits);
      if(mode==-1)return(OV_EBADPACKET);

      vb->mode=mode;
      if(!ci->mode_param[mode]){
        return(OV_EBADPACKET);
      }

      vb->W=ci->mode_param[mode]->blockflag;
      if(vb->W){
        vb->lW=oggpack_read(opb,1);
        vb->nW=oggpack_read(opb,1);
        if(vb->nW==-1)   return(OV_EBADPACKET);
      }else{
        vb->lW=0;
        vb->nW=0;
      }

      /* more setup */
      vb->granulepos=op->granulepos;
      vb->sequence=op->packetno;
      vb->eofflag=op->e_o_s;

      /* no pcm */
      vb->pcmend=0;
      vb->pcm=NULL;

      return(0);
        */
}

pub fn vorbis_packet_blocksize(
        vi: *mut VorbisInfo,
        op: *mut OggPacket) -> i64 {
    
    todo!();
        /*
            codec_setup_info     *ci=(codec_setup_info*)vi->codec_setup;
      oggpack_buffer       opb;
      int                  mode;

      if(ci==NULL || ci->modes<=0){
        /* codec setup not properly intialized */
        return(OV_EFAULT);
      }

      oggpack_readinit(&opb,op->packet,op->bytes);

      /* Check the packet type */
      if(oggpack_read(&opb,1)!=0){
        /* Oops.  This is not an audio data packet */
        return(OV_ENOTAUDIO);
      }

      /* read our mode and pre/post windowsize */
      mode=oggpack_read(&opb,ov_ilog(ci->modes-1));
      if(mode==-1 || !ci->mode_param[mode])return(OV_EBADPACKET);
      return(ci->blocksizes[ci->mode_param[mode]->blockflag]);
        */
}

pub fn vorbis_synthesis_halfrate(
        vi:   *mut VorbisInfo,
        flag: i32) -> i32 {
    
    todo!();
        /*
            /* set / clear half-sample-rate mode */
      codec_setup_info     *ci=(codec_setup_info*)vi->codec_setup;

      /* right now, our MDCT can't handle < 64 sample windows. */
      if(ci->blocksizes[0]<=64 && flag)return -1;
      ci->halfrate_flag=(flag?1:0);
      return 0;
        */
}

pub fn vorbis_synthesis_halfrate_p(vi: *mut VorbisInfo) -> i32 {
    
    todo!();
        /*
            codec_setup_info     *ci=(codec_setup_info*)vi->codec_setup;
      return ci->halfrate_flag;
        */
}
