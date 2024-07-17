/*!
  function: floor backend 0 implementation
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/floor0.c]

pub struct VorbisLookFloor0 {
    ln:        i32,
    m:         i32,
    linearmap: *mut *mut i32,
    n:         [i32; 2],
    vi:        *mut VorbisInfoFloor0,
    bits:      i64,
    frames:    i64,
}

pub fn floor0_free_info(i: *mut VorbisInfoFloor)  {
    
    todo!();
    /*
        vorbis_info_floor0 *info=(vorbis_info_floor0 *)i;
      if(info){
        memset(info,0,sizeof(*info));
        _ogg_free(info);
      }
    */
}

pub fn floor0_free_look(i: *mut VorbisLookFloor)  {
    
    todo!();
    /*
        vorbis_look_floor0 *look=(vorbis_look_floor0 *)i;
      if(look){

        if(look->linearmap){

          if(look->linearmap[0])_ogg_free(look->linearmap[0]);
          if(look->linearmap[1])_ogg_free(look->linearmap[1]);

          _ogg_free(look->linearmap);
        }
        memset(look,0,sizeof(*look));
        _ogg_free(look);
      }
    */
}

pub fn floor0_unpack(
        vi:  *mut VorbisInfo,
        opb: *mut OggPackBuffer) -> *mut VorbisInfoFloor {
    
    todo!();
    /*
        codec_setup_info     *ci=(codec_setup_info*)vi->codec_setup;
      int j;

      vorbis_info_floor0 *info=(vorbis_info_floor0*)_ogg_malloc(sizeof(*info));
      info->order=oggpack_read(opb,8);
      info->rate=oggpack_read(opb,16);
      info->barkmap=oggpack_read(opb,16);
      info->ampbits=oggpack_read(opb,6);
      info->ampdB=oggpack_read(opb,8);
      info->numbooks=oggpack_read(opb,4)+1;

      if(info->order<1)goto err_out;
      if(info->rate<1)goto err_out;
      if(info->barkmap<1)goto err_out;
      if(info->numbooks<1)goto err_out;

      for(j=0;j<info->numbooks;j++){
        info->books[j]=oggpack_read(opb,8);
        if(info->books[j]<0 || info->books[j]>=ci->books)goto err_out;
        if(ci->book_param[info->books[j]]->maptype==0)goto err_out;
        if(ci->book_param[info->books[j]]->dim<1)goto err_out;
      }
      return(info);

     err_out:
      floor0_free_info(info);
      return(NULL);
    */
}

/**
  | initialize Bark scale and normalization
  | lookups. We could do this with static
  | tables, but Vorbis allows a number of
  | possible combinations, so it's best
  | to do it computationally.
  | 
  | The below is authoritative in terms
  | of defining scale mapping. Note that
  | the scale depends on the sampling rate
  | as well as the linear block and mapping
  | sizes
  |
  */
pub fn floor0_map_lazy_init(
        vb:    *mut VorbisBlock,
        infox: *mut VorbisInfoFloor,
        look:  *mut VorbisLookFloor0)  {
    
    todo!();
    /*
        if(!look->linearmap[vb->W]){
        vorbis_dsp_state   *vd=vb->vd;
        vorbis_info        *vi=vd->vi;
        codec_setup_info   *ci=(codec_setup_info*)vi->codec_setup;
        vorbis_info_floor0 *info=(vorbis_info_floor0 *)infoX;
        int W=vb->W;
        int n=ci->blocksizes[W]/2,j;

        /* we choose a scaling constant so that:
           floor(bark(rate/2-1)*C)=mapped-1
         floor(bark(rate/2)*C)=mapped */
        float scale=look->ln/toBARK(info->rate/2.f);

        /* the mapping from a linear scale to a smaller bark scale is
           straightforward.  We do *not* make sure that the linear mapping
           does not skip bark-scale bins; the decoder simply skips them and
           the encoder may do what it wishes in filling them.  They're
           necessary in some mapping combinations to keep the scale spacing
           accurate */
        look->linearmap[W]=(int*)_ogg_malloc((n+1)*sizeof(**look->linearmap));
        for(j=0;j<n;j++){
          int val=floor( toBARK((info->rate/2.f)/n*j)
                         *scale); /* bark numbers represent band edges */
          if(val>=look->ln)val=look->ln-1; /* guard against the approximation */
          look->linearmap[W][j]=val;
        }
        look->linearmap[W][j]=-1;
        look->n[W]=n;
      }
    */
}


pub fn floor0_look(
        vd: *mut VorbisDspState,
        i:  *mut VorbisInfoFloor) -> *mut VorbisLookFloor {
    
    todo!();
    /*
        vorbis_info_floor0 *info=(vorbis_info_floor0 *)i;
      vorbis_look_floor0 *look=(vorbis_look_floor0*)_ogg_calloc(1,sizeof(*look));
      look->m=info->order;
      look->ln=info->barkmap;
      look->vi=info;

      look->linearmap=(int**)_ogg_calloc(2,sizeof(*look->linearmap));

      return look;
    */
}

pub fn floor0_inverse1(
        vb: *mut VorbisBlock,
        i:  *mut VorbisLookFloor)  {
    
    todo!();
    /*
        vorbis_look_floor0 *look=(vorbis_look_floor0 *)i;
      vorbis_info_floor0 *info=look->vi;
      int j,k;

      int ampraw=oggpack_read(&vb->opb,info->ampbits);
      if(ampraw>0){ /* also handles the -1 out of data case */
        long maxval=(1<<info->ampbits)-1;
        float amp=(float)ampraw/maxval*info->ampdB;
        int booknum=oggpack_read(&vb->opb,ov_ilog(info->numbooks));

        if(booknum!=-1 && booknum<info->numbooks){ /* be paranoid */
          codec_setup_info  *ci=(codec_setup_info *)vb->vd->vi->codec_setup;
          codebook *b=ci->fullbooks+info->books[booknum];
          float last=0.f;

          /* the additional b->dim is a guard against any possible stack
             smash; b->dim is provably more than we can overflow the
             vector */
          float *lsp=(float*)_vorbis_block_alloc(vb,sizeof(*lsp)*(look->m+b->dim+1));

          if(vorbis_book_decodev_set(b,lsp,&vb->opb,look->m)==-1)goto eop;
          for(j=0;j<look->m;){
            for(k=0;j<look->m && k<b->dim;k++,j++)lsp[j]+=last;
            last=lsp[j-1];
          }

          lsp[look->m]=amp;
          return(lsp);
        }
      }
     eop:
      return(NULL);
    */
}

pub fn floor0_inverse2(
        vb:   *mut VorbisBlock,
        i:    *mut VorbisLookFloor,
        memo: *mut c_void,
        out:  *mut f32) -> i32 {
    
    todo!();
    /*
        vorbis_look_floor0 *look=(vorbis_look_floor0 *)i;
      vorbis_info_floor0 *info=look->vi;

      floor0_map_lazy_init(vb,info,look);

      if(memo){
        float *lsp=(float *)memo;
        float amp=lsp[look->m];

        /* take the coefficients back to a spectral envelope curve */
        vorbis_lsp_to_curve(out,
                            look->linearmap[vb->W],
                            look->n[vb->W],
                            look->ln,
                            lsp,look->m,amp,(float)info->ampdB);
        return(1);
      }
      memset(out,0,sizeof(*out)*look->n[vb->W]);
      return(0);
    */
}

/** 
  export hooks 
  */
lazy_static!{
    /*
    const vorbis_func_floor floor0_exportbundle={
      NULL,&floor0_unpack,&floor0_look,&floor0_free_info,
      &floor0_free_look,&floor0_inverse1,&floor0_inverse2
    };
    */
}
