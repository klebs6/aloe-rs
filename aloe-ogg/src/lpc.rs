/*!
  function: LPC low level routines
 */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/lpc.h]

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/lpc.c]

/**
  | Autocorrelation LPC coeff generation
  | algorithm invented by N. Levinson in
  | 1947, modified by J. Durbin in 1959.
  | 
  | Input : n elements of time doamin data
  | 
  | Output: m lpc coefficients, excitation
  | energy
  |
  */
pub fn vorbis_lpc_from_data(
        data: *mut f32,
        lpci: *mut f32,
        n:    i32,
        m:    i32) -> f32 {
    
    todo!();
    /*
        double *aut=(double*)alloca(sizeof(*aut)*(m+1));
      double *lpc=(double*)alloca(sizeof(*lpc)*(m));
      double error;
      double epsilon;
      int i,j;

      /* autocorrelation, p+1 lag coefficients */
      j=m+1;
      while(j--){
        double d=0; /* double needed for accumulator depth */
        for(i=j;i<n;i++)d+=(double)data[i]*data[i-j];
        aut[j]=d;
      }

      /* Generate lpc coefficients from autocorr values */

      /* set our noise floor to about -100dB */
      error=aut[0] * (1. + 1e-10);
      epsilon=1e-9*aut[0]+1e-10;

      for(i=0;i<m;i++){
        double r= -aut[i+1];

        if(error<epsilon){
          memset(lpc+i,0,(m-i)*sizeof(*lpc));
          goto done;
        }

        /* Sum up this iteration's reflection coefficient; note that in
           Vorbis we don't save it.  If anyone wants to recycle this code
           and needs reflection coefficients, save the results of 'r' from
           each iteration. */

        for(j=0;j<i;j++)r-=lpc[j]*aut[i-j];
        r/=error;

        /* Update LPC coefficients and total error */

        lpc[i]=r;
        for(j=0;j<i/2;j++){
          double tmp=lpc[j];

          lpc[j]+=r*lpc[i-1-j];
          lpc[i-1-j]+=r*tmp;
        }
        if(i&1)lpc[j]+=lpc[j]*r;

        error*=1.-r*r;

      }

     done:

      /* slightly damp the filter */
      {
        double g = .99;
        double damp = g;
        for(j=0;j<m;j++){
          lpc[j]*=damp;
          damp*=g;
        }
      }

      for(j=0;j<m;j++)lpci[j]=(float)lpc[j];

      /* we need the error value to know how big an impulse to hit the
         filter with later */

      return error;
    */
}

pub fn vorbis_lpc_predict(
        coeff: *mut f32,
        prime: *mut f32,
        m:     i32,
        data:  *mut f32,
        n:     i64)  {
    
    todo!();
    /*
        /* in: coeff[0...m-1] LPC coefficients
             prime[0...m-1] initial values (allocated size of n+m-1)
        out: data[0...n-1] data samples */

      long i,j,o,p;
      float y;
      float *work=(float*)alloca(sizeof(*work)*(m+n));

      if(!prime)
        for(i=0;i<m;i++)
          work[i]=0.f;
      else
        for(i=0;i<m;i++)
          work[i]=prime[i];

      for(i=0;i<n;i++){
        y=0;
        o=i;
        p=m;
        for(j=0;j<m;j++)
          y-=work[o++]*coeff[--p];

        data[i]=work[o]=y;
      }
    */
}

