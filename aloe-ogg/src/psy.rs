/*!
 function: random psychoacoustics (not including preecho)
 */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/psy.h]

#[cfg(not(EHMER_MAX))]
pub const EHMER_MAX: usize = 56;

/* -------------- psychoacoustic setup  -------------- */

/**
  | 62Hz to 16kHz
  |
  */
pub const P_BANDS: usize = 17;

/**
  | 30dB to 100dB
  |
  */
pub const P_LEVELS: usize = 8;

/**
  | 30 dB
  |
  */
pub const P_LEVEL_0: f32 = 30.0;
pub const P_NOISECURVES:        usize = 3;
pub const NOISE_COMPAND_LEVELS: usize = 40;

pub struct VorbisInfoPsy {
    blockflag:        i32,
    ath_adjatt:       f32,
    ath_maxatt:       f32,
    tone_masteratt:   [f32; P_NOISECURVES],
    tone_centerboost: f32,
    tone_decay:       f32,
    tone_abs_limit:   f32,
    toneatt:          [f32; P_BANDS],
    noisemaskp:       i32,
    noisemaxsupp:     f32,
    noisewindowlo:    f32,
    noisewindowhi:    f32,
    noisewindowlomin: i32,
    noisewindowhimin: i32,
    noisewindowfixed: i32,
    noiseoff:         [[f32; P_NOISECURVES]; P_BANDS],
    noisecompand:     [f32; NOISE_COMPAND_LEVELS],
    max_curve_db:     f32,
    normal_p:         i32,
    normal_start:     i32,
    normal_partition: i32,
    normal_thresh:    f64,
}

pub struct VorbisInfoPsyGlobal {
    eighth_octave_lines:   i32,

    /**
      | for block long/short tuning; encode
      | only
      |
      */
    preecho_thresh:        [f32; VE_BANDS],

    postecho_thresh:       [f32; VE_BANDS],
    stretch_penalty:       f32,
    preecho_minenergy:     f32,
    ampmax_att_per_sec:    f32,

    /**
      | channel coupling config
      |
      */
    coupling_pk_hz:        [i32; PACKETBLOBS],

    coupling_pointlimit:   [[i32; 2]; PACKETBLOBS],
    coupling_prepointamp:  [i32; PACKETBLOBS],
    coupling_postpointamp: [i32; PACKETBLOBS],
    sliding_lowpass:       [[i32; 2]; PACKETBLOBS],
}

pub struct VorbisLookPsyGlobal {
    ampmax:              f32,
    channels:            i32,
    gi:                  *mut VorbisInfoPsyGlobal,
    coupling_pointlimit: [[i32; 2]; P_NOISECURVES],
}

pub struct VorbisLookPsy {
    n:                   i32,
    vi:                  *mut VorbisInfoPsy,
    tonecurves:          *mut *mut *mut f32,
    noiseoffset:         *mut *mut f32,
    ath:                 *mut f32,

    /**
      | in n.ocshift format
      |
      */
    octave:              *mut i64,

    bark:                *mut i64,
    firstoc:             i64,
    shiftoc:             i64,

    /**
      | power of two, please
      |
      */
    eighth_octave_lines: i32,

    total_octave_lines:  i32,

    /**
      | cache it
      |
      */
    rate:                i64,


    /**
      | Masking compensation value
      |
      */
    val:                 f32,

}

lazy_static!{
    /*
    extern void   _vp_psy_init(vorbis_look_psy *p,vorbis_info_psy *vi,
                               vorbis_info_psy_global *gi,int n,long rate);
    extern void   _vp_psy_clear(vorbis_look_psy *p);
    extern void  *_vi_psy_dup(void *source);

    extern void   _vi_psy_free(vorbis_info_psy *i);
    extern vorbis_info_psy *_vi_psy_copy(vorbis_info_psy *i);

    extern void _vp_noisemask(vorbis_look_psy *p,
                              float *logmdct,
                              float *logmask);

    extern void _vp_tonemask(vorbis_look_psy *p,
                             float *logfft,
                             float *logmask,
                             float global_specmax,
                             float local_specmax);

    extern void _vp_offset_and_mix(vorbis_look_psy *p,
                                   float *noise,
                                   float *tone,
                                   int offset_select,
                                   float *logmask,
                                   float *mdct,
                                   float *logmdct);

    extern float _vp_ampmax_decay(float amp,vorbis_dsp_state *vd);

    extern void _vp_couple_quantize_normalize(int blobno,
                                              vorbis_info_psy_global *g,
                                              vorbis_look_psy *p,
                                              vorbis_info_mapping0 *vi,
                                              float **mdct,
                                              int   **iwork,
                                              int    *nonzero,
                                              int     sliding_lowpass,
                                              int     ch);
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/psy.c]

pub const NEGINF: f32 = -9999.0;

pub const STEREO_THRESHHOLDS:         &[f32] = &[0.0, 0.5, 1.0, 1.5, 2.5, 4.5, 8.5, 16.5, 9e10];
pub const STEREO_THRESHHOLDS_LIMITED: &[f32] = &[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 4.5, 8.5, 9e10];

pub fn vp_global_look(vi: *mut VorbisInfo) -> *mut VorbisLookPsyGlobal {
    
    todo!();
        /*
            codec_setup_info *ci=(codec_setup_info*)vi->codec_setup;
      vorbis_info_psy_global *gi=&ci->psy_g_param;
      vorbis_look_psy_global *look=(vorbis_look_psy_global*)_ogg_calloc(1,sizeof(*look));

      look->channels=vi->channels;

      look->ampmax=-9999.;
      look->gi=gi;
      return(look);
        */
}

pub fn vp_global_free(look: *mut VorbisLookPsyGlobal)  {
    
    todo!();
        /*
            if(look){
        memset(look,0,sizeof(*look));
        _ogg_free(look);
      }
        */
}

#[inline] pub fn vi_gpsy_free(i: *mut VorbisInfoPsyGlobal)  {
    
    todo!();
        /*
            if(i){
        memset(i,0,sizeof(*i));
        _ogg_free(i);
      }
        */
}

pub fn vi_psy_free(i: *mut VorbisInfoPsy)  {
    
    todo!();
        /*
            if(i){
        memset(i,0,sizeof(*i));
        _ogg_free(i);
      }
        */
}

pub fn min_curve(
        c:  *mut f32,
        c2: *mut f32)  {
    
    todo!();
        /*
            int i;
      for(i=0;i<EHMER_MAX;i++)if(c2[i]<c[i])c[i]=c2[i];
        */
}

pub fn max_curve(
        c:  *mut f32,
        c2: *mut f32)  {
    
    todo!();
        /*
            int i;
      for(i=0;i<EHMER_MAX;i++)if(c2[i]>c[i])c[i]=c2[i];
        */
}

pub fn attenuate_curve(
        c:   *mut f32,
        att: f32)  {
    
    todo!();
        /*
            int i;
      for(i=0;i<EHMER_MAX;i++)
        c[i]+=att;
        */
}

pub fn setup_tone_curves(
        curveatt_db:       [f32; P_BANDS],
        bin_hz:            f32,
        n:                 i32,
        center_boost:      f32,
        center_decay_rate: f32) -> *mut *mut *mut f32 {
    
    todo!();
        /*
            int i,j,k,m;
      float ath[EHMER_MAX];
      float workc[P_BANDS][P_LEVELS][EHMER_MAX];
      float athc[P_LEVELS][EHMER_MAX];
      float *brute_buffer=(float*) alloca(n*sizeof(*brute_buffer));

      float ***ret=(float***) _ogg_malloc(sizeof(*ret)*P_BANDS);

      memset(workc,0,sizeof(workc));

      for(i=0;i<P_BANDS;i++){
        /* we add back in the ATH to avoid low level curves falling off to
           -infinity and unnecessarily cutting off high level curves in the
           curve limiting (last step). */

        /* A half-band's settings must be valid over the whole band, and
           it's better to mask too little than too much */
        int ath_offset=i*4;
        for(j=0;j<EHMER_MAX;j++){
          float min=999.;
          for(k=0;k<4;k++)
            if(j+k+ath_offset<MAX_ATH){
              if(min>ATH[j+k+ath_offset])min=ATH[j+k+ath_offset];
            }else{
              if(min>ATH[MAX_ATH-1])min=ATH[MAX_ATH-1];
            }
          ath[j]=min;
        }

        /* copy curves into working space, replicate the 50dB curve to 30
           and 40, replicate the 100dB curve to 110 */
        for(j=0;j<6;j++)
          memcpy(workc[i][j+2],tonemasks[i][j],EHMER_MAX*sizeof(*tonemasks[i][j]));
        memcpy(workc[i][0],tonemasks[i][0],EHMER_MAX*sizeof(*tonemasks[i][0]));
        memcpy(workc[i][1],tonemasks[i][0],EHMER_MAX*sizeof(*tonemasks[i][0]));

        /* apply centered curve boost/decay */
        for(j=0;j<P_LEVELS;j++){
          for(k=0;k<EHMER_MAX;k++){
            float adj=center_boost+abs(EHMER_OFFSET-k)*center_decay_rate;
            if(adj<0. && center_boost>0)adj=0.;
            if(adj>0. && center_boost<0)adj=0.;
            workc[i][j][k]+=adj;
          }
        }

        /* normalize curves so the driving amplitude is 0dB */
        /* make temp curves with the ATH overlayed */
        for(j=0;j<P_LEVELS;j++){
          attenuate_curve(workc[i][j],curveatt_dB[i]+100.-(j<2?2:j)*10.-P_LEVEL_0);
          memcpy(athc[j],ath,EHMER_MAX*sizeof(**athc));
          attenuate_curve(athc[j],+100.-j*10.f-P_LEVEL_0);
          max_curve(athc[j],workc[i][j]);
        }

        /* Now limit the louder curves.

           the idea is this: We don't know what the playback attenuation
           will be; 0dB SL moves every time the user twiddles the volume
           knob. So that means we have to use a single 'most pessimal' curve
           for all masking amplitudes, right?  Wrong.  The *loudest* sound
           can be in (we assume) a range of ...+100dB] SL.  However, sounds
           20dB down will be in a range ...+80], 40dB down is from ...+60],
           etc... */

        for(j=1;j<P_LEVELS;j++){
          min_curve(athc[j],athc[j-1]);
          min_curve(workc[i][j],athc[j]);
        }
      }

      for(i=0;i<P_BANDS;i++){
        int hi_curve,lo_curve,bin;
        ret[i]=(float**)_ogg_malloc(sizeof(**ret)*P_LEVELS);

        /* low frequency curves are measured with greater resolution than
           the MDCT/FFT will actually give us; we want the curve applied
           to the tone data to be pessimistic and thus apply the minimum
           masking possible for a given bin.  That means that a single bin
           could span more than one octave and that the curve will be a
           composite of multiple octaves.  It also may mean that a single
           bin may span > an eighth of an octave and that the eighth
           octave values may also be composited. */

        /* which octave curves will we be compositing? */
        bin=floor(fromOC(i*.5)/binHz);
        lo_curve=  ceil(toOC(bin*binHz+1)*2);
        hi_curve=  floor(toOC((bin+1)*binHz)*2);
        if(lo_curve>i)lo_curve=i;
        if(lo_curve<0)lo_curve=0;
        if(hi_curve>=P_BANDS)hi_curve=P_BANDS-1;

        for(m=0;m<P_LEVELS;m++){
          ret[i][m]=(float*)_ogg_malloc(sizeof(***ret)*(EHMER_MAX+2));

          for(j=0;j<n;j++)brute_buffer[j]=999.;

          /* render the curve into bins, then pull values back into curve.
             The point is that any inherent subsampling aliasing results in
             a safe minimum */
          for(k=lo_curve;k<=hi_curve;k++){
            int l=0;

            for(j=0;j<EHMER_MAX;j++){
              int lo_bin= fromOC(j*.125+k*.5-2.0625)/binHz;
              int hi_bin= fromOC(j*.125+k*.5-1.9375)/binHz+1;

              if(lo_bin<0)lo_bin=0;
              if(lo_bin>n)lo_bin=n;
              if(lo_bin<l)l=lo_bin;
              if(hi_bin<0)hi_bin=0;
              if(hi_bin>n)hi_bin=n;

              for(;l<hi_bin && l<n;l++)
                if(brute_buffer[l]>workc[k][m][j])
                  brute_buffer[l]=workc[k][m][j];
            }

            for(;l<n;l++)
              if(brute_buffer[l]>workc[k][m][EHMER_MAX-1])
                brute_buffer[l]=workc[k][m][EHMER_MAX-1];

          }

          /* be equally paranoid about being valid up to next half ocatve */
          if(i+1<P_BANDS){
            int l=0;
            k=i+1;
            for(j=0;j<EHMER_MAX;j++){
              int lo_bin= fromOC(j*.125+i*.5-2.0625)/binHz;
              int hi_bin= fromOC(j*.125+i*.5-1.9375)/binHz+1;

              if(lo_bin<0)lo_bin=0;
              if(lo_bin>n)lo_bin=n;
              if(lo_bin<l)l=lo_bin;
              if(hi_bin<0)hi_bin=0;
              if(hi_bin>n)hi_bin=n;

              for(;l<hi_bin && l<n;l++)
                if(brute_buffer[l]>workc[k][m][j])
                  brute_buffer[l]=workc[k][m][j];
            }

            for(;l<n;l++)
              if(brute_buffer[l]>workc[k][m][EHMER_MAX-1])
                brute_buffer[l]=workc[k][m][EHMER_MAX-1];

          }


          for(j=0;j<EHMER_MAX;j++){
            int bin=fromOC(j*.125+i*.5-2.)/binHz;
            if(bin<0){
              ret[i][m][j+2]=-999.;
            }else{
              if(bin>=n){
                ret[i][m][j+2]=-999.;
              }else{
                ret[i][m][j+2]=brute_buffer[bin];
              }
            }
          }

          /* add fenceposts */
          for(j=0;j<EHMER_OFFSET;j++)
            if(ret[i][m][j+2]>-200.f)break;
          ret[i][m][0]=j;

          for(j=EHMER_MAX-1;j>EHMER_OFFSET+1;j--)
            if(ret[i][m][j+2]>-200.f)
              break;
          ret[i][m][1]=j;

        }
      }

      return(ret);
        */
}

pub fn vp_psy_init(
        p:    *mut VorbisLookPsy,
        vi:   *mut VorbisInfoPsy,
        gi:   *mut VorbisInfoPsyGlobal,
        n:    i32,
        rate: i64)  {
    
    todo!();
        /*
            long i,j,lo=-99,hi=1;
      long maxoc;
      memset(p,0,sizeof(*p));

      p->eighth_octave_lines=gi->eighth_octave_lines;
      p->shiftoc=rint(log(gi->eighth_octave_lines*8.f)/log(2.f))-1;

      p->firstoc=toOC(.25f*rate*.5/n)*(1<<(p->shiftoc+1))-gi->eighth_octave_lines;
      maxoc=toOC((n+.25f)*rate*.5/n)*(1<<(p->shiftoc+1))+.5f;
      p->total_octave_lines=maxoc-p->firstoc+1;
      p->ath=(float*)_ogg_malloc(n*sizeof(*p->ath));

      p->octave=(long*)_ogg_malloc(n*sizeof(*p->octave));
      p->bark=(long*)_ogg_malloc(n*sizeof(*p->bark));
      p->vi=vi;
      p->n=n;
      p->rate=rate;

      /* AoTuV HF weighting */
      p->m_val = 1.;
      if(rate < 26000) p->m_val = 0;
      else if(rate < 38000) p->m_val = .94;   /* 32kHz */
      else if(rate > 46000) p->m_val = 1.275; /* 48kHz */

      /* set up the lookups for a given blocksize and sample rate */

      for(i=0,j=0;i<MAX_ATH-1;i++){
        int endpos=rint(fromOC((i+1)*.125-2.)*2*n/rate);
        float base=ATH[i];
        if(j<endpos){
          float delta=(ATH[i+1]-base)/(endpos-j);
          for(;j<endpos && j<n;j++){
            p->ath[j]=base+100.;
            base+=delta;
          }
        }
      }

      for(;j<n;j++){
        p->ath[j]=p->ath[j-1];
      }

      for(i=0;i<n;i++){
        float bark=toBARK(rate/(2*n)*i);

        for(;lo+vi->noisewindowlomin<i &&
              toBARK(rate/(2*n)*lo)<(bark-vi->noisewindowlo);lo++);

        for(;hi<=n && (hi<i+vi->noisewindowhimin ||
              toBARK(rate/(2*n)*hi)<(bark+vi->noisewindowhi));hi++);

        p->bark[i]=((lo-1)<<16)+(hi-1);

      }

      for(i=0;i<n;i++)
        p->octave[i]=toOC((i+.25f)*.5*rate/n)*(1<<(p->shiftoc+1))+.5f;

      p->tonecurves=setup_tone_curves(vi->toneatt,rate*.5/n,n,
                                      vi->tone_centerboost,vi->tone_decay);

      /* set up rolling noise median */
      p->noiseoffset=(float**)_ogg_malloc(P_NOISECURVES*sizeof(*p->noiseoffset));
      for(i=0;i<P_NOISECURVES;i++)
        p->noiseoffset[i]=(float*)_ogg_malloc(n*sizeof(**p->noiseoffset));

      for(i=0;i<n;i++){
        float halfoc=toOC((i+.5)*rate/(2.*n))*2.;
        int inthalfoc;
        float del;

        if(halfoc<0)halfoc=0;
        if(halfoc>=P_BANDS-1)halfoc=P_BANDS-1;
        inthalfoc=(int)halfoc;
        del=halfoc-inthalfoc;

        for(j=0;j<P_NOISECURVES;j++)
          p->noiseoffset[j][i]=
            p->vi->noiseoff[j][inthalfoc]*(1.-del) +
            p->vi->noiseoff[j][inthalfoc+1]*del;

      }
    #if 0
      {
        static int ls=0;
        _analysis_output_always("noiseoff0",ls,p->noiseoffset[0],n,1,0,0);
        _analysis_output_always("noiseoff1",ls,p->noiseoffset[1],n,1,0,0);
        _analysis_output_always("noiseoff2",ls++,p->noiseoffset[2],n,1,0,0);
      }
    #endif
        */
}

pub fn vp_psy_clear(p: *mut VorbisLookPsy)  {
    
    todo!();
        /*
            int i,j;
      if(p){
        if(p->ath)_ogg_free(p->ath);
        if(p->octave)_ogg_free(p->octave);
        if(p->bark)_ogg_free(p->bark);
        if(p->tonecurves){
          for(i=0;i<P_BANDS;i++){
            for(j=0;j<P_LEVELS;j++){
              _ogg_free(p->tonecurves[i][j]);
            }
            _ogg_free(p->tonecurves[i]);
          }
          _ogg_free(p->tonecurves);
        }
        if(p->noiseoffset){
          for(i=0;i<P_NOISECURVES;i++){
            _ogg_free(p->noiseoffset[i]);
          }
          _ogg_free(p->noiseoffset);
        }
        memset(p,0,sizeof(*p));
      }
        */
}


/**
  | octave/(8*eighth_octave_lines)
  | x scale and dB y scale
  |
  */
pub fn seed_curve(
        seed:      *mut f32,
        curves:    *const *const f32,
        amp:       f32,
        oc:        i32,
        n:         i32,
        linesper:  i32,
        d_boffset: f32)  {
    
    todo!();
        /*
            int i,post1;
      int seedptr;
      const float *posts,*curve;

      int choice=(int)((amp+dBoffset-P_LEVEL_0)*.1f);
      choice=max(choice,0);
      choice=min(choice,P_LEVELS-1);
      posts=curves[choice];
      curve=posts+2;
      post1=(int)posts[1];
      seedptr=oc+(posts[0]-EHMER_OFFSET)*linesper-(linesper>>1);

      for(i=posts[0];i<post1;i++){
        if(seedptr>0){
          float lin=amp+curve[i];
          if(seed[seedptr]<lin)seed[seedptr]=lin;
        }
        seedptr+=linesper;
        if(seedptr>=n)break;
      }
        */
}

pub fn seed_loop(
        p:       *mut VorbisLookPsy,
        curves:  *const *const *const f32,
        f:       *const f32,
        flr:     *const f32,
        seed:    *mut f32,
        specmax: f32)  {
    
    todo!();
        /*
            vorbis_info_psy *vi=p->vi;
      long n=p->n,i;
      float dBoffset=vi->max_curve_dB-specmax;

      /* prime the working vector with peak values */

      for(i=0;i<n;i++){
        float max=f[i];
        long oc=p->octave[i];
        while(i+1<n && p->octave[i+1]==oc){
          i++;
          if(f[i]>max)max=f[i];
        }

        if(max+6.f>flr[i]){
          oc=oc>>p->shiftoc;

          if(oc>=P_BANDS)oc=P_BANDS-1;
          if(oc<0)oc=0;

          seed_curve(seed,
                     curves[oc],
                     max,
                     p->octave[i]-p->firstoc,
                     p->total_octave_lines,
                     p->eighth_octave_lines,
                     dBoffset);
        }
      }
        */
}

pub fn seed_chase(
        seeds:    *mut f32,
        linesper: i32,
        n:        i64)  {
    
    todo!();
        /*
            long  *posstack=(long*)alloca(n*sizeof(*posstack));
      float *ampstack=(float*)alloca(n*sizeof(*ampstack));
      long   stack=0;
      long   pos=0;
      long   i;

      for(i=0;i<n;i++){
        if(stack<2){
          posstack[stack]=i;
          ampstack[stack++]=seeds[i];
        }else{
          while(1){
            if(seeds[i]<ampstack[stack-1]){
              posstack[stack]=i;
              ampstack[stack++]=seeds[i];
              break;
            }else{
              if(i<posstack[stack-1]+linesper){
                if(stack>1 && ampstack[stack-1]<=ampstack[stack-2] &&
                   i<posstack[stack-2]+linesper){
                  /* we completely overlap, making stack-1 irrelevant.  pop it */
                  stack--;
                  continue;
                }
              }
              posstack[stack]=i;
              ampstack[stack++]=seeds[i];
              break;

            }
          }
        }
      }

      /* the stack now contains only the positions that are relevant. Scan
         'em straight through */

      for(i=0;i<stack;i++){
        long endpos;
        if(i<stack-1 && ampstack[i+1]>ampstack[i]){
          endpos=posstack[i+1];
        }else{
          endpos=posstack[i]+linesper+1; /* +1 is important, else bin 0 is
                                            discarded in short frames */
        }
        if(endpos>n)endpos=n;
        for(;pos<endpos;pos++)
          seeds[pos]=ampstack[i];
      }

      /* there.  Linear time.  I now remember this was on a problem set I
         had in Grad Skool... I didn't solve it at the time ;-) */
        */
}

/**
  | bleaugh, this is more complicated than
  | it needs to be
  |
  */
pub fn max_seeds(
        p:    *mut VorbisLookPsy,
        seed: *mut f32,
        flr:  *mut f32)  {
    
    todo!();
        /*
            long   n=p->total_octave_lines;
      int    linesper=p->eighth_octave_lines;
      long   linpos=0;
      long   pos;

      seed_chase(seed,linesper,n); /* for masking */

      pos=p->octave[0]-p->firstoc-(linesper>>1);

      while(linpos+1<p->n){
        float minV=seed[pos];
        long end=((p->octave[linpos]+p->octave[linpos+1])>>1)-p->firstoc;
        if(minV>p->vi->tone_abs_limit)minV=p->vi->tone_abs_limit;
        while(pos+1<=end){
          pos++;
          if((seed[pos]>NEGINF && seed[pos]<minV) || minV==NEGINF)
            minV=seed[pos];
        }

        end=pos+p->firstoc;
        for(;linpos<p->n && p->octave[linpos]<=end;linpos++)
          if(flr[linpos]<minV)flr[linpos]=minV;
      }

      {
        float minV=seed[p->total_octave_lines-1];
        for(;linpos<p->n;linpos++)
          if(flr[linpos]<minV)flr[linpos]=minV;
      }
        */
}

pub fn bark_noise_hybridmp(
        n:      i32,
        b:      *const i64,
        f:      *const f32,
        noise:  *mut f32,
        offset: f32,
        fixed:  i32)  {
    
    todo!();
        /*
            float *N=(float*) alloca(n*sizeof(*N));
      float *X=(float*) alloca(n*sizeof(*N));
      float *XX=(float*) alloca(n*sizeof(*N));
      float *Y=(float*) alloca(n*sizeof(*N));
      float *XY=(float*) alloca(n*sizeof(*N));

      float tN, tX, tXX, tY, tXY;
      int i;

      int lo, hi;
      float R=0.f;
      float A=0.f;
      float B=0.f;
      float D=1.f;
      float w, x, y;

      tN = tX = tXX = tY = tXY = 0.f;

      y = f[0] + offset;
      if (y < 1.f) y = 1.f;

      w = y * y * .5;

      tN += w;
      tX += w;
      tY += w * y;

      N[0] = tN;
      X[0] = tX;
      XX[0] = tXX;
      Y[0] = tY;
      XY[0] = tXY;

      for (i = 1, x = 1.f; i < n; i++, x += 1.f) {

        y = f[i] + offset;
        if (y < 1.f) y = 1.f;

        w = y * y;

        tN += w;
        tX += w * x;
        tXX += w * x * x;
        tY += w * y;
        tXY += w * x * y;

        N[i] = tN;
        X[i] = tX;
        XX[i] = tXX;
        Y[i] = tY;
        XY[i] = tXY;
      }

      for (i = 0, x = 0.f; i < n; i++, x += 1.f) {

        lo = b[i] >> 16;
        hi = b[i] & 0xffff;
        if( lo>=0 || -lo>=n ) break;
        if( hi>=n ) break;

        tN = N[hi] + N[-lo];
        tX = X[hi] - X[-lo];
        tXX = XX[hi] + XX[-lo];
        tY = Y[hi] + Y[-lo];
        tXY = XY[hi] - XY[-lo];

        A = tY * tXX - tX * tXY;
        B = tN * tXY - tX * tY;
        D = tN * tXX - tX * tX;
        R = (A + x * B) / D;
        if (R < 0.f) R = 0.f;

        noise[i] = R - offset;
      }

      for ( ; i < n; i++, x += 1.f) {

        lo = b[i] >> 16;
        hi = b[i] & 0xffff;
        if( lo<0 || lo>=n ) break;
        if( hi>=n ) break;

        tN = N[hi] - N[lo];
        tX = X[hi] - X[lo];
        tXX = XX[hi] - XX[lo];
        tY = Y[hi] - Y[lo];
        tXY = XY[hi] - XY[lo];

        A = tY * tXX - tX * tXY;
        B = tN * tXY - tX * tY;
        D = tN * tXX - tX * tX;
        R = (A + x * B) / D;
        if (R < 0.f) R = 0.f;

        noise[i] = R - offset;
      }

      for ( ; i < n; i++, x += 1.f) {

        R = (A + x * B) / D;
        if (R < 0.f) R = 0.f;

        noise[i] = R - offset;
      }

      if (fixed <= 0) return;

      for (i = 0, x = 0.f; i < n; i++, x += 1.f) {
        hi = i + fixed / 2;
        lo = hi - fixed;
        if ( hi>=n ) break;
        if ( lo>=0 ) break;

        tN = N[hi] + N[-lo];
        tX = X[hi] - X[-lo];
        tXX = XX[hi] + XX[-lo];
        tY = Y[hi] + Y[-lo];
        tXY = XY[hi] - XY[-lo];


        A = tY * tXX - tX * tXY;
        B = tN * tXY - tX * tY;
        D = tN * tXX - tX * tX;
        R = (A + x * B) / D;

        if (R - offset < noise[i]) noise[i] = R - offset;
      }
      for ( ; i < n; i++, x += 1.f) {

        hi = i + fixed / 2;
        lo = hi - fixed;
        if ( hi>=n ) break;
        if ( lo<0 ) break;

        tN = N[hi] - N[lo];
        tX = X[hi] - X[lo];
        tXX = XX[hi] - XX[lo];
        tY = Y[hi] - Y[lo];
        tXY = XY[hi] - XY[lo];

        A = tY * tXX - tX * tXY;
        B = tN * tXY - tX * tY;
        D = tN * tXX - tX * tX;
        R = (A + x * B) / D;

        if (R - offset < noise[i]) noise[i] = R - offset;
      }
      for ( ; i < n; i++, x += 1.f) {
        R = (A + x * B) / D;
        if (R - offset < noise[i]) noise[i] = R - offset;
      }
        */
}

pub fn vp_noisemask(
        p:       *mut VorbisLookPsy,
        logmdct: *mut f32,
        logmask: *mut f32)  {
    
    todo!();
        /*
            int i,n=p->n;
      float *work=(float*) alloca(n*sizeof(*work));

      bark_noise_hybridmp(n,p->bark,logmdct,logmask,
                          140.,-1);

      for(i=0;i<n;i++)work[i]=logmdct[i]-logmask[i];

      bark_noise_hybridmp(n,p->bark,work,logmask,0.,
                          p->vi->noisewindowfixed);

      for(i=0;i<n;i++)work[i]=logmdct[i]-work[i];

    #if 0
      {
        static int seq=0;

        float work2[n];
        for(i=0;i<n;i++){
          work2[i]=logmask[i]+work[i];
        }

        if(seq&1)
          _analysis_output("median2R",seq/2,work,n,1,0,0);
        else
          _analysis_output("median2L",seq/2,work,n,1,0,0);

        if(seq&1)
          _analysis_output("envelope2R",seq/2,work2,n,1,0,0);
        else
          _analysis_output("envelope2L",seq/2,work2,n,1,0,0);
        seq++;
      }
    #endif

      for(i=0;i<n;i++){
        int dB=logmask[i]+.5;
        if(dB>=NOISE_COMPAND_LEVELS)dB=NOISE_COMPAND_LEVELS-1;
        if(dB<0)dB=0;
        logmask[i]= work[i]+p->vi->noisecompand[dB];
      }
        */
}

pub fn vp_tonemask(
        p:              *mut VorbisLookPsy,
        logfft:         *mut f32,
        logmask:        *mut f32,
        global_specmax: f32,
        local_specmax:  f32)  {
    
    todo!();
        /*
            int i,n=p->n;

      float *seed=(float*) alloca(sizeof(*seed)*p->total_octave_lines);
      float att=local_specmax+p->vi->ath_adjatt;
      for(i=0;i<p->total_octave_lines;i++)seed[i]=NEGINF;

      /* set the ATH (floating below localmax, not global max by a
         specified att) */
      if(att<p->vi->ath_maxatt)att=p->vi->ath_maxatt;

      for(i=0;i<n;i++)
        logmask[i]=p->ath[i]+att;

      /* tone masking */
      seed_loop(p,(const float ***)p->tonecurves,logfft,logmask,seed,global_specmax);
      max_seeds(p,seed,logmask);
        */
}

pub fn vp_offset_and_mix(
        p:             *mut VorbisLookPsy,
        noise:         *mut f32,
        tone:          *mut f32,
        offset_select: i32,
        logmask:       *mut f32,
        mdct:          *mut f32,
        logmdct:       *mut f32)  {
    
    todo!();
        /*
            int i,n=p->n;
      float de, coeffi, cx;/* AoTuV */
      float toneatt=p->vi->tone_masteratt[offset_select];

      cx = p->m_val;

      for(i=0;i<n;i++){
        float val= noise[i]+p->noiseoffset[offset_select][i];
        if(val>p->vi->noisemaxsupp)val=p->vi->noisemaxsupp;
        logmask[i]=max(val,tone[i]+toneatt);


        /* AoTuV */
        /** @ M1 **
            The following codes improve a noise problem.
            A fundamental idea uses the value of masking and carries out
            the relative compensation of the MDCT.
            However, this code is not perfect and all noise problems cannot be solved.
            by Aoyumi @ 2004/04/18
        */

        if(offset_select == 1) {
          coeffi = -17.2;       /* coeffi is a -17.2dB threshold */
          val = val - logmdct[i];  /* val == mdct line value relative to floor in dB */

          if(val > coeffi){
            /* mdct value is > -17.2 dB below floor */

            de = 1.0-((val-coeffi)*0.005*cx);
            /* pro-rated attenuation:
               -0.00 dB boost if mdct value is -17.2dB (relative to floor)
               -0.77 dB boost if mdct value is 0dB (relative to floor)
               -1.64 dB boost if mdct value is +17.2dB (relative to floor)
               etc... */

            if(de < 0) de = 0.0001;
          }else
            /* mdct value is <= -17.2 dB below floor */

            de = 1.0-((val-coeffi)*0.0003*cx);
          /* pro-rated attenuation:
             +0.00 dB atten if mdct value is -17.2dB (relative to floor)
             +0.45 dB atten if mdct value is -34.4dB (relative to floor)
             etc... */

          mdct[i] *= de;

        }
      }
        */
}

pub fn vp_ampmax_decay(
        amp: f32,
        vd:  *mut VorbisDspState) -> f32 {
    
    todo!();
        /*
            vorbis_info *vi=vd->vi;
      codec_setup_info *ci=(codec_setup_info*)vi->codec_setup;
      vorbis_info_psy_global *gi=&ci->psy_g_param;

      int n=ci->blocksizes[vd->W]/2;
      float secs=(float)n/vi->rate;

      amp+=secs*gi->ampmax_att_per_sec;
      if(amp<-9999)amp=-9999;
      return(amp);
        */
}

pub const FLOOR1_fromdB_LOOKUP: [f32; 256] = [
    1.0649863e-07, 1.1341951e-07, 1.2079015e-07, 1.2863978e-07,
    1.3699951e-07, 1.4590251e-07, 1.5538408e-07, 1.6548181e-07,
    1.7623575e-07, 1.8768855e-07, 1.9988561e-07, 2.128753e-07,
    2.2670913e-07, 2.4144197e-07, 2.5713223e-07, 2.7384213e-07,
    2.9163793e-07, 3.1059021e-07, 3.3077411e-07, 3.5226968e-07,
    3.7516214e-07, 3.9954229e-07, 4.2550680e-07, 4.5315863e-07,
    4.8260743e-07, 5.1396998e-07, 5.4737065e-07, 5.8294187e-07,
    6.2082472e-07, 6.6116941e-07, 7.0413592e-07, 7.4989464e-07,
    7.9862701e-07, 8.5052630e-07, 9.0579828e-07, 9.6466216e-07,
    1.0273513e-06, 1.0941144e-06, 1.1652161e-06, 1.2409384e-06,
    1.3215816e-06, 1.4074654e-06, 1.4989305e-06, 1.5963394e-06,
    1.7000785e-06, 1.8105592e-06, 1.9282195e-06, 2.0535261e-06,
    2.1869758e-06, 2.3290978e-06, 2.4804557e-06, 2.6416497e-06,
    2.8133190e-06, 2.9961443e-06, 3.1908506e-06, 3.3982101e-06,
    3.6190449e-06, 3.8542308e-06, 4.1047004e-06, 4.3714470e-06,
    4.6555282e-06, 4.9580707e-06, 5.2802740e-06, 5.6234160e-06,
    5.9888572e-06, 6.3780469e-06, 6.7925283e-06, 7.2339451e-06,
    7.7040476e-06, 8.2047000e-06, 8.7378876e-06, 9.3057248e-06,
    9.9104632e-06, 1.0554501e-05, 1.1240392e-05, 1.1970856e-05,
    1.2748789e-05, 1.3577278e-05, 1.4459606e-05, 1.5399272e-05,
    1.6400004e-05, 1.7465768e-05, 1.8600792e-05, 1.9809576e-05,
    2.1096914e-05, 2.2467911e-05, 2.3928002e-05, 2.5482978e-05,
    2.7139006e-05, 2.8902651e-05, 3.0780908e-05, 3.2781225e-05,
    3.4911534e-05, 3.7180282e-05, 3.9596466e-05, 4.2169667e-05,
    4.4910090e-05, 4.7828601e-05, 5.0936773e-05, 5.4246931e-05,
    5.7772202e-05, 6.1526565e-05, 6.5524908e-05, 6.9783085e-05,
    7.4317983e-05, 7.9147585e-05, 8.4291040e-05, 8.9768747e-05,
    9.5602426e-05, 0.00010181521, 0.00010843174, 0.00011547824,
    0.00012298267, 0.00013097477, 0.00013948625, 0.00014855085,
    0.00015820453, 0.00016848555, 0.00017943469, 0.00019109536,
    0.00020351382, 0.00021673929, 0.00023082423, 0.00024582449,
    0.00026179955, 0.00027881276, 0.00029693158, 0.00031622787,
    0.00033677814, 0.00035866388, 0.00038197188, 0.00040679456,
    0.00043323036, 0.00046138411, 0.00049136745, 0.00052329927,
    0.00055730621, 0.00059352311, 0.00063209358, 0.00067317058,
    0.00071691700, 0.00076350630, 0.00081312324, 0.00086596457,
    0.00092223983, 0.00098217216, 0.0010459992, 0.0011139742,
    0.0011863665, 0.0012634633, 0.0013455702, 0.0014330129,
    0.0015261382, 0.0016253153, 0.0017309374, 0.0018434235,
    0.0019632195, 0.0020908006, 0.0022266726, 0.0023713743,
    0.0025254795, 0.0026895994, 0.0028643847, 0.0030505286,
    0.0032487691, 0.0034598925, 0.0036847358, 0.0039241906,
    0.0041792066, 0.0044507950, 0.0047400328, 0.0050480668,
    0.0053761186, 0.0057254891, 0.0060975636, 0.0064938176,
    0.0069158225, 0.0073652516, 0.0078438871, 0.0083536271,
    0.0088964928, 0.009474637, 0.010090352, 0.010746080,
    0.011444421, 0.012188144, 0.012980198, 0.013823725,
    0.014722068, 0.015678791, 0.016697687, 0.017782797,
    0.018938423, 0.020169149, 0.021479854, 0.022875735,
    0.024362330, 0.025945531, 0.027631618, 0.029427276,
    0.031339626, 0.033376252, 0.035545228, 0.037855157,
    0.040315199, 0.042935108, 0.045725273, 0.048696758,
    0.051861348, 0.055231591, 0.058820850, 0.062643361,
    0.066714279, 0.071049749, 0.075666962, 0.080584227,
    0.085821044, 0.091398179, 0.097337747, 0.10366330,
    0.11039993, 0.11757434, 0.12521498, 0.13335215,
    0.14201813, 0.15124727, 0.16107617, 0.17154380,
    0.18269168, 0.19456402, 0.20720788, 0.22067342,
    0.23501402, 0.25028656, 0.26655159, 0.28387361,
    0.30232132, 0.32196786, 0.34289114, 0.36517414,
    0.38890521, 0.41417847, 0.44109412, 0.46975890,
    0.50028648, 0.53279791, 0.56742212, 0.60429640,
    0.64356699, 0.68538959, 0.72993007, 0.77736504,
    0.82788260, 0.88168307, 0.9389798, 1.0,
];

/** 
  this is for per-channel noise normalization 
  */
pub fn apsort(
        a: *const c_void,
        b: *const c_void) -> i32 {
    
    todo!();
    /*
        float f1=**(float**)a;
      float f2=**(float**)b;
      return (f1<f2)-(f1>f2);
    */
}

pub fn flag_lossless(
        limit:     i32,
        prepoint:  f32,
        postpoint: f32,
        mdct:      *mut f32,
        floor:     *mut f32,
        flag:      *mut i32,
        i:         i32,
        jn:        i32)  {
    
    todo!();
    /*
        int j;
      for(j=0;j<jn;j++){
        float point = j>=limit-i ? postpoint : prepoint;
        float r = fabs(mdct[j])/floor[j];
        if(r<point)
          flag[j]=0;
        else
          flag[j]=1;
      }
    */
}

/**
  | Overload/Side effect: On input, the
  | *q vector holds either the quantized
  | energy (for elements with the flag set)
  | or the absolute values of the *r vector
  | (for elements with flag unset). On output,
  | q holds the quantized energy for all
  | elements
  |
  */
pub fn noise_normalize(
        p:     *mut VorbisLookPsy,
        limit: i32,
        r:     *mut f32,
        q:     *mut f32,
        f:     *mut f32,
        flags: *mut i32,
        acc:   f32,
        i:     i32,
        n:     i32,
        out:   *mut i32) -> f32 {
    
    todo!();
    /*
        vorbis_info_psy *vi=p->vi;
      float **sort = (float**)alloca(n*sizeof(*sort));
      int j,count=0;
      int start = (vi->normal_p ? vi->normal_start-i : n);
      if(start>n)start=n;

      /* force classic behavior where only energy in the current band is considered */
      acc=0.f;

      /* still responsible for populating *out where noise norm not in
         effect.  There's no need to [re]populate *q in these areas */
      for(j=0;j<start;j++){
        if(!flags || !flags[j]){ /* lossless coupling already quantized.
                                    Don't touch; requantizing based on
                                    energy would be incorrect. */
          float ve = q[j]/f[j];
          if(r[j]<0)
            out[j] = -rint(sqrt(ve));
          else
            out[j] = rint(sqrt(ve));
        }
      }

      /* sort magnitudes for noise norm portion of partition */
      for(;j<n;j++){
        if(!flags || !flags[j]){ /* can't noise norm elements that have
                                    already been loslessly coupled; we can
                                    only account for their energy error */
          float ve = q[j]/f[j];
          /* Despite all the new, more capable coupling code, for now we
             implement noise norm as it has been up to this point. Only
             consider promotions to unit magnitude from 0.  In addition
             the only energy error counted is quantizations to zero. */
          /* also-- the original point code only applied noise norm at > pointlimit */
          if(ve<.25f && (!flags || j>=limit-i)){
            acc += ve;
            sort[count++]=q+j; /* q is fabs(r) for unflagged element */
          }else{
            /* For now: no acc adjustment for nonzero quantization.  populate *out and q as this value is final. */
            if(r[j]<0)
              out[j] = -rint(sqrt(ve));
            else
              out[j] = rint(sqrt(ve));
            q[j] = out[j]*out[j]*f[j];
          }
        }/* else{
            again, no energy adjustment for error in nonzero quant-- for now
            }*/
      }

      if(count){
        /* noise norm to do */
        qsort(sort,count,sizeof(*sort),apsort);
        for(j=0;j<count;j++){
          int k=sort[j]-q;
          if(acc>=vi->normal_thresh){
            out[k]=unitnorm(r[k]);
            acc-=1.f;
            q[k]=f[k];
          }else{
            out[k]=0;
            q[k]=0.f;
          }
        }
      }

      return acc;
    */
}

/**
  | Noise normalization, quantization
  | and coupling are not wholly seperable
  | processes in depth>1 coupling.
  |
  */
pub fn vp_couple_quantize_normalize(
        blobno:          i32,
        g:               *mut VorbisInfoPsyGlobal,
        p:               *mut VorbisLookPsy,
        vi:              *mut VorbisInfoMapping0,
        mdct:            *mut *mut f32,
        iwork:           *mut *mut i32,
        nonzero:         *mut i32,
        sliding_lowpass: i32,
        ch:              i32)  {
    
    todo!();
    /*
        int i;
      int n = p->n;
      int partition=(p->vi->normal_p ? p->vi->normal_partition : 16);
      int limit = g->coupling_pointlimit[p->vi->blockflag][blobno];
      float prepoint=stereo_threshholds[g->coupling_prepointamp[blobno]];
      float postpoint=stereo_threshholds[g->coupling_postpointamp[blobno]];
    #if 0
      float de=0.1*p->m_val; /* a blend of the AoTuV M2 and M3 code here and below */
    #endif

      /* mdct is our raw mdct output, floor not removed. */
      /* inout passes in the ifloor, passes back quantized result */

      /* unquantized energy (negative indicates amplitude has negative sign) */
      float **raw = (float**)alloca(ch*sizeof(*raw));

      /* dual pupose; quantized energy (if flag set), othersize fabs(raw) */
      float **quant = (float**)alloca(ch*sizeof(*quant));

      /* floor energy */
      float **floor = (float**)alloca(ch*sizeof(*floor));

      /* flags indicating raw/quantized status of elements in raw vector */
      int   **flag  = (int**)alloca(ch*sizeof(*flag));

      /* non-zero flag working vector */
      int    *nz    = (int*)alloca(ch*sizeof(*nz));

      /* energy surplus/defecit tracking */
      float  *acc   = (float*)alloca((ch+vi->coupling_steps)*sizeof(*acc));

      /* The threshold of a stereo is changed with the size of n */
      if(n > 1000)
        postpoint=stereo_threshholds_limited[g->coupling_postpointamp[blobno]];

      raw[0]   = (float*)alloca(ch*partition*sizeof(**raw));
      quant[0] = (float*)alloca(ch*partition*sizeof(**quant));
      floor[0] = (float*)alloca(ch*partition*sizeof(**floor));
      flag[0]  = (int*)alloca(ch*partition*sizeof(**flag));

      for(i=1;i<ch;i++){
        raw[i]   = &raw[0][partition*i];
        quant[i] = &quant[0][partition*i];
        floor[i] = &floor[0][partition*i];
        flag[i]  = &flag[0][partition*i];
      }
      for(i=0;i<ch+vi->coupling_steps;i++)
        acc[i]=0.f;

      for(i=0;i<n;i+=partition){
        int k,j,jn = partition > n-i ? n-i : partition;
        int step,track = 0;

        memcpy(nz,nonzero,sizeof(*nz)*ch);

        /* prefill */
        memset(flag[0],0,ch*partition*sizeof(**flag));
        for(k=0;k<ch;k++){
          int *iout = &iwork[k][i];
          if(nz[k]){

            for(j=0;j<jn;j++)
              floor[k][j] = FLOOR1_fromdB_LOOKUP[iout[j]];

            flag_lossless(limit,prepoint,postpoint,&mdct[k][i],floor[k],flag[k],i,jn);

            for(j=0;j<jn;j++){
              quant[k][j] = raw[k][j] = mdct[k][i+j]*mdct[k][i+j];
              if(mdct[k][i+j]<0.f) raw[k][j]*=-1.f;
              floor[k][j]*=floor[k][j];
            }

            acc[track]=noise_normalize(p,limit,raw[k],quant[k],floor[k],NULL,acc[track],i,jn,iout);

          }else{
            for(j=0;j<jn;j++){
              floor[k][j] = 1e-10f;
              raw[k][j] = 0.f;
              quant[k][j] = 0.f;
              flag[k][j] = 0;
              iout[j]=0;
            }
            acc[track]=0.f;
          }
          track++;
        }

        /* coupling */
        for(step=0;step<vi->coupling_steps;step++){
          int Mi = vi->coupling_mag[step];
          int Ai = vi->coupling_ang[step];
          int *iM = &iwork[Mi][i];
          int *iA = &iwork[Ai][i];
          float *reM = raw[Mi];
          float *reA = raw[Ai];
          float *qeM = quant[Mi];
          float *qeA = quant[Ai];
          float *floorM = floor[Mi];
          float *floorA = floor[Ai];
          int *fM = flag[Mi];
          int *fA = flag[Ai];

          if(nz[Mi] || nz[Ai]){
            nz[Mi] = nz[Ai] = 1;

            for(j=0;j<jn;j++){

              if(j<sliding_lowpass-i){
                if(fM[j] || fA[j]){
                  /* lossless coupling */

                  reM[j] = fabs(reM[j])+fabs(reA[j]);
                  qeM[j] = qeM[j]+qeA[j];
                  fM[j]=fA[j]=1;

                  /* couple iM/iA */
                  {
                    int A = iM[j];
                    int B = iA[j];

                    if(abs(A)>abs(B)){
                      iA[j]=(A>0?A-B:B-A);
                    }else{
                      iA[j]=(B>0?A-B:B-A);
                      iM[j]=B;
                    }

                    /* collapse two equivalent tuples to one */
                    if(iA[j]>=abs(iM[j])*2){
                      iA[j]= -iA[j];
                      iM[j]= -iM[j];
                    }

                  }

                }else{
                  /* lossy (point) coupling */
                  if(j<limit-i){
                    /* dipole */
                    reM[j] += reA[j];
                    qeM[j] = fabs(reM[j]);
                  }else{
    #if 0
                    /* AoTuV */
                    /** @ M2 **
                        The boost problem by the combination of noise normalization and point stereo is eased.
                        However, this is a temporary patch.
                        by Aoyumi @ 2004/04/18
                    */
                    float derate = (1.0 - de*((float)(j-limit+i) / (float)(n-limit)));
                    /* elliptical */
                    if(reM[j]+reA[j]<0){
                      reM[j] = - (qeM[j] = (fabs(reM[j])+fabs(reA[j]))*derate*derate);
                    }else{
                      reM[j] =   (qeM[j] = (fabs(reM[j])+fabs(reA[j]))*derate*derate);
                    }
    #else
                    /* elliptical */
                    if(reM[j]+reA[j]<0){
                      reM[j] = - (qeM[j] = fabs(reM[j])+fabs(reA[j]));
                    }else{
                      reM[j] =   (qeM[j] = fabs(reM[j])+fabs(reA[j]));
                    }
    #endif

                  }
                  reA[j]=qeA[j]=0.f;
                  fA[j]=1;
                  iA[j]=0;
                }
              }
              floorM[j]=floorA[j]=floorM[j]+floorA[j];
            }
            /* normalize the resulting mag vector */
            acc[track]=noise_normalize(p,limit,raw[Mi],quant[Mi],floor[Mi],flag[Mi],acc[track],i,jn,iM);
            track++;
          }
        }
      }

      for(i=0;i<vi->coupling_steps;i++){
        /* make sure coupling a zero and a nonzero channel results in two
           nonzero channels. */
        if(nonzero[vi->coupling_mag[i]] ||
           nonzero[vi->coupling_ang[i]]){
          nonzero[vi->coupling_mag[i]]=1;
          nonzero[vi->coupling_ang[i]]=1;
        }
      }
    */
}
