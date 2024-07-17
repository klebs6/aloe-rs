/*!
  | Speaker Arrangement Definitions (SpeakerArrangement)
  |
  */
crate::ix!();

/**
  | Speaker Arrangement Definitions.
  | \ingroup speakerArrangements
  |
  */
pub const SPEAKER_ARR_EMPTY:           SpeakerArrangement = 0; // empty arrangement
pub const SPEAKER_ARR_MONO:            SpeakerArrangement = SPEAKER_M; // M
pub const SPEAKER_ARR_STEREO:          SpeakerArrangement = SPEAKER_L   | SPEAKER_R; // L R
pub const SPEAKER_ARR_STEREO_SURROUND: SpeakerArrangement = SPEAKER_LS  | SPEAKER_RS; // Ls Rs
pub const SPEAKER_ARR_STEREO_CENTER:   SpeakerArrangement = SPEAKER_LC  | SPEAKER_RC; // Lc Rc
pub const SPEAKER_ARR_STEREO_SIDE:     SpeakerArrangement = SPEAKER_SL  | SPEAKER_SR; // Sl Sr
pub const SPEAKER_ARR_STEREO_CLFE:     SpeakerArrangement = SPEAKER_C   | SPEAKER_LFE; // C Lfe
pub const SPEAKER_ARR_STEREOTF:        SpeakerArrangement = SPEAKER_TFL | SPEAKER_TFR; // Tfl Tfr
pub const SPEAKER_ARR_STEREOTS:        SpeakerArrangement = SPEAKER_TSL | SPEAKER_TSR; // Tsl Tsr
pub const SPEAKER_ARR_STEREOTR:        SpeakerArrangement = SPEAKER_TRL | SPEAKER_TRR; // Trl Trr
pub const SPEAKER_ARR_STEREOBF:        SpeakerArrangement = SPEAKER_BFL | SPEAKER_BFR; // Bfl Bfr
pub const SPEAKER_ARR_CINE_FRONT:      SpeakerArrangement = SPEAKER_L   | SPEAKER_R | SPEAKER_C | SPEAKER_LC | SPEAKER_RC; // L R C Lc Rc

/**
  | L R C
  |
  */
pub const SPEAKER_ARR_K_30CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C;

/**
  | L R C Lfe
  |
  */
pub const SPEAKER_ARR_K_31CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE;

/**
  | L R S
  |
  */
pub const SPEAKER_ARR_K_30MUSIC: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_CS;

/**
  | L R Lfe S
  |
  */
pub const SPEAKER_ARR_K_31MUSIC: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_LFE 
    | SPEAKER_CS;

/**
  | L R C S (LCRS)
  |
  */
pub const SPEAKER_ARR_K_40CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_CS;

/**
  | L R C Lfe S (LCRS+Lfe)
  |
  */
pub const SPEAKER_ARR_K_41CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE 
    | SPEAKER_CS;

/**
  | L R Ls Rs (Quadro)
  |
  */
pub const SPEAKER_ARR_K_40MUSIC: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_LS  
    | SPEAKER_RS;

/**
  | L R Lfe Ls Rs (Quadro+Lfe)
  |
  */
pub const SPEAKER_ARR_K_41MUSIC: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_LFE 
    | SPEAKER_LS  
    | SPEAKER_RS;

/**
  | L R C Ls Rs
  |
  */
pub const SPEAKER_ARR_K50: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LS  
    | SPEAKER_RS;

/**
  | L R C Lfe Ls Rs
  |
  */
pub const SPEAKER_ARR_K51: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS;

/**
  | L R C Ls Rs Cs
  |
  */
pub const SPEAKER_ARR_K_60CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_CS;

/**
  | L R C Lfe Ls Rs Cs
  |
  */
pub const SPEAKER_ARR_K_61CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_CS;

/**
  | L R Ls Rs Sl Sr
  |
  */
pub const SPEAKER_ARR_K_60MUSIC: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_LS  
    | SPEAKER_RS  
    | SPEAKER_SL 
    | SPEAKER_SR;

/**
  | L R Lfe Ls Rs Sl Sr
  |
  */
pub const SPEAKER_ARR_K_61MUSIC: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_LFE 
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR;

/**
  | L R C Ls Rs Lc Rc
  |
  */
pub const SPEAKER_ARR_K_70CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC;

/**
  | L R C Lfe Ls Rs Lc Rc
  |
  */
pub const SPEAKER_ARR_K_71CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC;

pub const SPEAKER_ARR_K_71CINE_FULL_FRONT: SpeakerArrangement = 
    SPEAKER_ARR_K_71CINE;

/**
  | L R C Ls Rs Sl Sr
  |
  */
pub const SPEAKER_ARR_K_70MUSIC: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR;

/**
  | L R C Lfe Ls Rs Sl Sr
  |
  */
pub const SPEAKER_ARR_K_71MUSIC: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR;

/**
  | L R C Lfe Ls Rs Lcs Rcs
  |
  */
pub const SPEAKER_ARR_K_71CINE_FULL_REAR: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_LCS 
    | SPEAKER_RCS;

pub const SPEAKER_ARR_K_71CINE_SIDE_FILL: SpeakerArrangement = 
    SPEAKER_ARR_K_71MUSIC;

/**
  | L R C Lfe Ls Rs Pl Pr
  |
  */
pub const SPEAKER_ARR_K_71PROXIMITY: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_PL 
    | SPEAKER_PR;

/**
  | L R C Ls Rs Lc Rc Cs
  |
  */
pub const SPEAKER_ARR_K_80CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_CS;

/**
  | L R C Lfe Ls Rs Lc Rc Cs
  |
  */
pub const SPEAKER_ARR_K_81CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_CS;

/**
  | L R C Ls Rs Cs Sl Sr
  |
  */
pub const SPEAKER_ARR_K_80MUSIC: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_CS 
    | SPEAKER_SL 
    | SPEAKER_SR;

/**
  | L R C Lfe Ls Rs Cs Sl Sr
  |
  */
pub const SPEAKER_ARR_K_81MUSIC: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_CS 
    | SPEAKER_SL 
    | SPEAKER_SR;

/**
  | L R C Ls Rs Lc Rc Sl Sr
  |
  */
pub const SPEAKER_ARR_K_90CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_SL 
    | SPEAKER_SR;

/**
  | L R C Lfe Ls Rs Lc Rc Sl Sr
  |
  */
pub const SPEAKER_ARR_K_91CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_SL 
    | SPEAKER_SR;

/**
  | L R C Ls Rs Lc Rc Cs Sl Sr
  |
  */
pub const SPEAKER_ARR_K_100CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_CS 
    | SPEAKER_SL 
    | SPEAKER_SR;

/**
  | L R C Lfe Ls Rs Lc Rc Cs Sl Sr
  |
  */
pub const SPEAKER_ARR_K_101CINE: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C   
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_CS 
    | SPEAKER_SL 
    | SPEAKER_SR;

/**
  | First-Order with Ambisonic Channel
  | Number (ACN) ordering and SN3D normalization
  |
  */
pub const SPEAKER_ARR_K_AMBI1ST_ORDERACN: SpeakerArrangement = 
    SPEAKER_ACN0 
    | SPEAKER_ACN1 
    | SPEAKER_ACN2 
    | SPEAKER_ACN3;

/**
  | Second-Order with Ambisonic Channel
  | Number (ACN) ordering and SN3D normalization
  |
  */
pub const SPEAKER_ARR_K_AMBI2CD_ORDERACN: SpeakerArrangement = 
    SPEAKER_ARR_K_AMBI1ST_ORDERACN 
    | SPEAKER_ACN4 
    | SPEAKER_ACN5 
    | SPEAKER_ACN6 
    | SPEAKER_ACN7 
    | SPEAKER_ACN8;

/**
  | Third-Order with Ambisonic Channel
  | Number (ACN) ordering and SN3D normalization
  |
  */
pub const SPEAKER_ARR_K_AMBI3RD_ORDERACN: SpeakerArrangement = 
    SPEAKER_ARR_K_AMBI2CD_ORDERACN 
    | SPEAKER_ACN9 
    | SPEAKER_ACN10 
    | SPEAKER_ACN11 
    | SPEAKER_ACN12 
    | SPEAKER_ACN13 
    | SPEAKER_ACN14 
    | SPEAKER_ACN15;

/* ------------------ 3D formats  ------------------ */

/**
  | L R Ls Rs Tfl Tfr Trl Trr
  |
  */
// 4.0.4
pub const SPEAKER_ARR_K_80CUBE: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_LS 
    | SPEAKER_RS  
    | SPEAKER_TFL
    | SPEAKER_TFR
    | SPEAKER_TRL 
    | SPEAKER_TRR;

/**
  | L R C Lfe Ls Rs Cs Tc
  |
  */
// 6.1.1
pub const SPEAKER_ARR_K_71CINE_TOP_CENTER: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C  
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_CS  
    | SPEAKER_TC;

/**
  | L R C Lfe Ls Rs Cs Tfc
  |
  */
// 6.1.1
pub const SPEAKER_ARR_K_71CINE_CENTER_HIGH: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C  
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_CS  
    | SPEAKER_TFC;

/**
  | L R C Lfe Ls Rs Tfl Tfr
  |
  */
// 5.1.2
pub const SPEAKER_ARR_K_71CINE_FRONT_HIGH: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C  
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_TFL 
    | SPEAKER_TFR;

pub const SPEAKER_ARR_K71MPEG3D: SpeakerArrangement = 
SPEAKER_ARR_K_71CINE_FRONT_HIGH;

/**
  | L R C Lfe Ls Rs Tsl Tsr
  |
  */
// 5.1.2
pub const SPEAKER_ARR_K_71CINE_SIDE_HIGH: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C  
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_TSL 
    | SPEAKER_TSR;

/**
  | L R Lfe Ls Rs Tfl Tfc Tfr Bfc
  |
  */
// 4.1.4
pub const SPEAKER_ARR_K81MPEG3D: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_TFL 
    | SPEAKER_TFC 
    | SPEAKER_TFR 
    | SPEAKER_BFC;

/** L R C Ls Rs Tfl Tfr Trl Trr */                      
// 5.0.4
pub const SPEAKER_ARR_K90: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_TFL
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

pub const SPEAKER_ARR_K50_4: SpeakerArrangement = SPEAKER_ARR_K90;

/** L R C Lfe Ls Rs Tfl Tfr Trl Trr */                  
// 5.1.4
pub const SPEAKER_ARR_K91: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS  
    | SPEAKER_TFL
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

pub const SPEAKER_ARR_K51_4: SpeakerArrangement = SPEAKER_ARR_K91;

/** L R C Ls Rs Sl Sr Tsl Tsr */                        
// 7.0.2
pub const SPEAKER_ARR_K70_2: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TSL 
    | SPEAKER_TSR;

/** L R C Lfe Ls Rs Sl Sr Tsl Tsr */                    
// 7.1.2
pub const SPEAKER_ARR_K71_2: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TSL 
    | SPEAKER_TSR;

pub const SPEAKER_ARR_K_91ATMOS: SpeakerArrangement = SPEAKER_ARR_K71_2; // 9.1 Dolby Atmos (3D)

/** L R C Ls Rs Sl Sr Tfl Tfr Trl Trr */                
// 7.0.4
pub const SPEAKER_ARR_K70_4: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TFL 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

/** L R C Lfe Ls Rs Sl Sr Tfl Tfr Trl Trr */            
// 7.1.4
pub const SPEAKER_ARR_K71_4: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TFL 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

pub const SPEAKER_ARR_K111MPEG3D: SpeakerArrangement = SPEAKER_ARR_K71_4;

/** L R C Ls Rs Sl Sr Tfl Tfr Trl Trr Tsl Tsr */        
// 7.0.6
pub const SPEAKER_ARR_K70_6: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TFL 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR 
    | SPEAKER_TSL 
    | SPEAKER_TSR;

/** L R C Lfe Ls Rs Sl Sr Tfl Tfr Trl Trr Tsl Tsr */    
// 7.1.6
pub const SPEAKER_ARR_K71_6: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TFL 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR 
    | SPEAKER_TSL 
    | SPEAKER_TSR;

/** L R C Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr */          
// 9.0.4
pub const SPEAKER_ARR_K90_4: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TFL 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

/** L R C Lfe Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr */      
// 9.1.4
pub const SPEAKER_ARR_K91_4: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TFL 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

/** L R C Lfe Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr Tsl Tsr */ 
// 9.0.6
pub const SPEAKER_ARR_K90_6: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TFL 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR 
    | SPEAKER_TSL 
    | SPEAKER_TSR;

/** L R C Lfe Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr Tsl Tsr */ 
// 9.1.6
pub const SPEAKER_ARR_K91_6: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TFL 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR 
    | SPEAKER_TSL 
    | SPEAKER_TSR;

/** L R C Ls Rs Tc Tfl Tfr Trl Trr */                   
// 5.0.5
pub const SPEAKER_ARR_K100: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_TC 
    | SPEAKER_TFL 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

/** L R C Lfe Ls Rs Tc Tfl Tfr Trl Trr */               
// 5.1.5
pub const SPEAKER_ARR_K101: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS  
    | SPEAKER_TC 
    | SPEAKER_TFL 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

pub const SPEAKER_ARR_K101MPEG3D: SpeakerArrangement = SPEAKER_ARR_K101;

/** L R C Lfe Ls Rs Tfl Tfc Tfr Trl Trr Lfe2 */         
// 5.2.5
pub const SPEAKER_ARR_K102: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C  
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS  
    | SPEAKER_TFL
    | SPEAKER_TFC 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR 
    | SPEAKER_LFE2;

/** L R C Ls Rs Tc Tfl Tfc Tfr Trl Trr */               
// 5.0.6
pub const SPEAKER_ARR_K110: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_TC 
    | SPEAKER_TFL 
    | SPEAKER_TFC 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

/** L R C Lfe Ls Rs Tc Tfl Tfc Tfr Trl Trr */           
// 5.1.6
pub const SPEAKER_ARR_K111: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_TC 
    | SPEAKER_TFL 
    | SPEAKER_TFC 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

/** L R C Lfe Ls Rs Lc Rc Tfl Tfc Tfr Trl Trr Lfe2 */   
// 7.2.5
pub const SPEAKER_ARR_K122: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C  
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS  
    | SPEAKER_LC  
    | SPEAKER_RC 
    | SPEAKER_TFL
    | SPEAKER_TFC 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR 
    | SPEAKER_LFE2;

/** L R C Ls Rs Sl Sr Tc Tfl Tfc Tfr Trl Trr */         
// 7.0.6
pub const SPEAKER_ARR_K130: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LS  
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TC 
    | SPEAKER_TFL 
    | SPEAKER_TFC 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

/** L R C Lfe Ls Rs Sl Sr Tc Tfl Tfc Tfr Trl Trr */     
// 7.1.6
pub const SPEAKER_ARR_K131: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C 
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TC 
    | SPEAKER_TFL 
    | SPEAKER_TFC 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR;

/** L R Ls Rs Sl Sr Tfl Tfr Trl Trr Bfl Bfr Brl Brr  */ 
// 6.0.4.4
pub const SPEAKER_ARR_K140: SpeakerArrangement = 
    SPEAKER_L 
    | SPEAKER_R 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TFL 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRR 
    | SPEAKER_BFL 
    | SPEAKER_BFR 
    | SPEAKER_BRL 
    | SPEAKER_BRR;

/** L R C Ls Rs Lc Rc Cs Sl Sr Tc Tfl Tfc Tfr Trl Trc Trr Tsl Tsr Bfl Bfc Bfr */            
// 10.0.9.3
pub const SPEAKER_ARR_K220: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C  
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_CS 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TC 
    | SPEAKER_TFL 
    | SPEAKER_TFC 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRC 
    | SPEAKER_TRR 
    | SPEAKER_TSL 
    | SPEAKER_TSR 
    | SPEAKER_BFL
    | SPEAKER_BFC 
    | SPEAKER_BFR;

/** L R C Lfe Ls Rs Lc Rc Cs Sl Sr Tc Tfl Tfc Tfr Trl Trc Trr Lfe2 Tsl Tsr Bfl Bfc Bfr */   
// 10.2.9.3
pub const SPEAKER_ARR_K222: SpeakerArrangement = 
    SPEAKER_L  
    | SPEAKER_R 
    | SPEAKER_C  
    | SPEAKER_LFE 
    | SPEAKER_LS 
    | SPEAKER_RS 
    | SPEAKER_LC 
    | SPEAKER_RC 
    | SPEAKER_CS 
    | SPEAKER_SL 
    | SPEAKER_SR 
    | SPEAKER_TC 
    | SPEAKER_TFL 
    | SPEAKER_TFC 
    | SPEAKER_TFR 
    | SPEAKER_TRL 
    | SPEAKER_TRC 
    | SPEAKER_TRR 
    | SPEAKER_LFE2 
    | SPEAKER_TSL 
    | SPEAKER_TSR 
    | SPEAKER_BFL
    | SPEAKER_BFC 
    | SPEAKER_BFR;

/** Speaker Arrangement String Representation.
\ingroup speakerArrangements */
pub const SPEAKER_ARR_STRING_EMPTY:              &'static str = "";
pub const SPEAKER_ARR_STRING_MONO:               &'static str = "Mono";
pub const SPEAKER_ARR_STRING_STEREO:             &'static str = "Stereo";
pub const SPEAKER_ARR_STRING_STEREOR:            &'static str = "Stereo (Ls Rs)";
pub const SPEAKER_ARR_STRING_STEREOC:            &'static str = "Stereo (Lc Rc)";
pub const SPEAKER_ARR_STRING_STEREO_SIDE:        &'static str = "Stereo (Sl Sr)";
pub const SPEAKER_ARR_STRING_STEREO_CLFE:        &'static str = "Stereo (C LFE)";
pub const SPEAKER_ARR_STRING_STEREOTF:           &'static str = "Stereo (Tfl Tfr)";
pub const SPEAKER_ARR_STRING_STEREOTS:           &'static str = "Stereo (Tsl Tsr)";
pub const SPEAKER_ARR_STRING_STEREOTR:           &'static str = "Stereo (Trl Trr)";
pub const SPEAKER_ARR_STRING_STEREOBF:           &'static str = "Stereo (Bfl Bfr)";
pub const SPEAKER_ARR_STRING_CINE_FRONT:         &'static str = "Cine Front";
pub const SPEAKER_ARR_STRING_30CINE:             &'static str = "LRC";
pub const SPEAKER_ARR_STRING_30MUSIC:            &'static str = "LRS";
pub const SPEAKER_ARR_STRING_31CINE:             &'static str = "LRC+LFE";
pub const SPEAKER_ARR_STRING_31MUSIC:            &'static str = "LRS+LFE";
pub const SPEAKER_ARR_STRING_40CINE:             &'static str = "LRCS";
pub const SPEAKER_ARR_STRING_40MUSIC:            &'static str = "Quadro";
pub const SPEAKER_ARR_STRING_41CINE:             &'static str = "LRCS+LFE";
pub const SPEAKER_ARR_STRING_41MUSIC:            &'static str = "Quadro+LFE";
pub const SPEAKER_ARR_STRING50:                  &'static str = "5.0";
pub const SPEAKER_ARR_STRING51:                  &'static str = "5.1";
pub const SPEAKER_ARR_STRING_60CINE:             &'static str = "6.0 Cine";
pub const SPEAKER_ARR_STRING_60MUSIC:            &'static str = "6.0 Music";
pub const SPEAKER_ARR_STRING_61CINE:             &'static str = "6.1 Cine";
pub const SPEAKER_ARR_STRING_61MUSIC:            &'static str = "6.1 Music";
pub const SPEAKER_ARR_STRING_70CINE:             &'static str = "7.0 SDDS";
pub const SPEAKER_ARR_STRING_70CINE_OLD:         &'static str = "7.0 Cine (SDDS)";
pub const SPEAKER_ARR_STRING_70MUSIC:            &'static str = "7.0";
pub const SPEAKER_ARR_STRING_70MUSIC_OLD:        &'static str = "7.0 Music (Dolby)";
pub const SPEAKER_ARR_STRING_71CINE:             &'static str = "7.1 SDDS";
pub const SPEAKER_ARR_STRING_71CINE_OLD:         &'static str = "7.1 Cine (SDDS)";
pub const SPEAKER_ARR_STRING_71MUSIC:            &'static str = "7.1";
pub const SPEAKER_ARR_STRING_71MUSIC_OLD:        &'static str = "7.1 Music (Dolby)";
pub const SPEAKER_ARR_STRING_71CINE_TOP_CENTER:  &'static str = "7.1 Cine Top Center";
pub const SPEAKER_ARR_STRING_71CINE_CENTER_HIGH: &'static str = "7.1 Cine Center High";
pub const SPEAKER_ARR_STRING_71CINE_FRONT_HIGH:  &'static str = "7.1 Cine Front High";
pub const SPEAKER_ARR_STRING_71CINE_SIDE_HIGH:   &'static str = "7.1 Cine Side High";
pub const SPEAKER_ARR_STRING_71CINE_FULL_REAR:   &'static str = "7.1 Cine Full Rear";
pub const SPEAKER_ARR_STRING_71PROXIMITY:        &'static str = "7.1 Proximity";
pub const SPEAKER_ARR_STRING_80CINE:             &'static str = "8.0 Cine";
pub const SPEAKER_ARR_STRING_80MUSIC:            &'static str = "8.0 Music";
pub const SPEAKER_ARR_STRING_80CUBE:             &'static str = "8.0 Cube";
pub const SPEAKER_ARR_STRING_81CINE:             &'static str = "8.1 Cine";
pub const SPEAKER_ARR_STRING_81MUSIC:            &'static str = "8.1 Music";
pub const SPEAKER_ARR_STRING_90CINE:             &'static str = "9.0 Cine";
pub const SPEAKER_ARR_STRING_91CINE:             &'static str = "9.1 Cine";
pub const SPEAKER_ARR_STRING_100CINE:            &'static str = "10.0 Cine";
pub const SPEAKER_ARR_STRING_101CINE:            &'static str = "10.1 Cine";
pub const SPEAKER_ARR_STRING102:                 &'static str = "10.2 Experimental";
pub const SPEAKER_ARR_STRING122:                 &'static str = "12.2";
pub const SPEAKER_ARR_STRING50_4:                &'static str = "5.0.4";
pub const SPEAKER_ARR_STRING51_4:                &'static str = "5.1.4";
pub const SPEAKER_ARR_STRING70_2:                &'static str = "7.0.2";
pub const SPEAKER_ARR_STRING71_2:                &'static str = "7.1.2";
pub const SPEAKER_ARR_STRING70_4:                &'static str = "7.0.4";
pub const SPEAKER_ARR_STRING71_4:                &'static str = "7.1.4";
pub const SPEAKER_ARR_STRING70_6:                &'static str = "7.0.6";
pub const SPEAKER_ARR_STRING71_6:                &'static str = "7.1.6";
pub const SPEAKER_ARR_STRING90_4:                &'static str = "9.0.4";
pub const SPEAKER_ARR_STRING91_4:                &'static str = "9.1.4";
pub const SPEAKER_ARR_STRING90_6:                &'static str = "9.0.6";
pub const SPEAKER_ARR_STRING91_6:                &'static str = "9.1.6";
pub const SPEAKER_ARR_STRING100:                 &'static str = "10.0 Auro-3D";
pub const SPEAKER_ARR_STRING101:                 &'static str = "10.1 Auro-3D";
pub const SPEAKER_ARR_STRING110:                 &'static str = "11.0 Auro-3D";
pub const SPEAKER_ARR_STRING111:                 &'static str = "11.1 Auro-3D";
pub const SPEAKER_ARR_STRING130:                 &'static str = "13.0 Auro-3D";
pub const SPEAKER_ARR_STRING131:                 &'static str = "13.1 Auro-3D";
pub const SPEAKER_ARR_STRING81MPEG:              &'static str = "8.1 MPEG";
pub const SPEAKER_ARR_STRING140:                 &'static str = "14.0";
pub const SPEAKER_ARR_STRING222:                 &'static str = "22.2";
pub const SPEAKER_ARR_STRING220:                 &'static str = "22.0";
pub const SPEAKER_ARR_STRING_AMBI1ST_ORDER:      &'static str = "1st Order Ambisonics";
pub const SPEAKER_ARR_STRING_AMBI2CD_ORDER:      &'static str = "2nd Order Ambisonics";
pub const SPEAKER_ARR_STRING_AMBI3RD_ORDER:      &'static str = "3rd Order Ambisonics";

/**
  | Speaker Arrangement String Representation
  | with Speakers Name. \ingroup speakerArrangements
  |
  */
pub const SPEAKER_ARR_STRING_MONOS:               &'static str = "M";
pub const SPEAKER_ARR_STRING_STEREOS:             &'static str = "L R";
pub const SPEAKER_ARR_STRING_STEREORS:            &'static str = "Ls Rs";
pub const SPEAKER_ARR_STRING_STEREOCS:            &'static str = "Lc Rc";
pub const SPEAKER_ARR_STRING_STEREOSS:            &'static str = "Sl Sr";
pub const SPEAKER_ARR_STRING_STEREO_CLFES:        &'static str = "C LFE";
pub const SPEAKER_ARR_STRING_STEREOTFS:           &'static str = "Tfl Tfr";
pub const SPEAKER_ARR_STRING_STEREOTSS:           &'static str = "Tsl Tsr";
pub const SPEAKER_ARR_STRING_STEREOTRS:           &'static str = "Trl Trr";
pub const SPEAKER_ARR_STRING_STEREOBFS:           &'static str = "Bfl Bfr";
pub const SPEAKER_ARR_STRING_CINE_FRONTS:         &'static str = "L R C Lc Rc";
pub const SPEAKER_ARR_STRING_30CINES:             &'static str = "L R C";
pub const SPEAKER_ARR_STRING_30MUSICS:            &'static str = "L R S";
pub const SPEAKER_ARR_STRING_31CINES:             &'static str = "L R C LFE";
pub const SPEAKER_ARR_STRING_31MUSICS:            &'static str = "L R LFE S";
pub const SPEAKER_ARR_STRING_40CINES:             &'static str = "L R C S";
pub const SPEAKER_ARR_STRING_40MUSICS:            &'static str = "L R Ls Rs";
pub const SPEAKER_ARR_STRING_41CINES:             &'static str = "L R C LFE S";
pub const SPEAKER_ARR_STRING_41MUSICS:            &'static str = "L R LFE Ls Rs";
pub const SPEAKER_ARR_STRING50S:                  &'static str = "L R C Ls Rs";
pub const SPEAKER_ARR_STRING51S:                  &'static str = "L R C LFE Ls Rs";
pub const SPEAKER_ARR_STRING_60CINES:             &'static str = "L R C Ls Rs Cs";
pub const SPEAKER_ARR_STRING_60MUSICS:            &'static str = "L R Ls Rs Sl Sr";
pub const SPEAKER_ARR_STRING_61CINES:             &'static str = "L R C LFE Ls Rs Cs";
pub const SPEAKER_ARR_STRING_61MUSICS:            &'static str = "L R LFE Ls Rs Sl Sr";
pub const SPEAKER_ARR_STRING_70CINES:             &'static str = "L R C Ls Rs Lc Rc";
pub const SPEAKER_ARR_STRING_70MUSICS:            &'static str = "L R C Ls Rs Sl Sr";
pub const SPEAKER_ARR_STRING_71CINES:             &'static str = "L R C LFE Ls Rs Lc Rc";
pub const SPEAKER_ARR_STRING_71MUSICS:            &'static str = "L R C LFE Ls Rs Sl Sr";
pub const SPEAKER_ARR_STRING_80CINES:             &'static str = "L R C Ls Rs Lc Rc Cs";
pub const SPEAKER_ARR_STRING_80MUSICS:            &'static str = "L R C Ls Rs Cs Sl Sr";
pub const SPEAKER_ARR_STRING_81CINES:             &'static str = "L R C LFE Ls Rs Lc Rc Cs";
pub const SPEAKER_ARR_STRING_81MUSICS:            &'static str = "L R C LFE Ls Rs Cs Sl Sr";
pub const SPEAKER_ARR_STRING_80CUBES:             &'static str = "L R Ls Rs Tfl Tfr Trl Trr";
pub const SPEAKER_ARR_STRING_71CINE_TOP_CENTERS:  &'static str = "L R C LFE Ls Rs Cs Tc";
pub const SPEAKER_ARR_STRING_71CINE_CENTER_HIGHS: &'static str = "L R C LFE Ls Rs Cs Tfc";
pub const SPEAKER_ARR_STRING_71CINE_FRONT_HIGHS:  &'static str = "L R C LFE Ls Rs Tfl Tfl";
pub const SPEAKER_ARR_STRING_71CINE_SIDE_HIGHS:   &'static str = "L R C LFE Ls Rs Tsl Tsl";
pub const SPEAKER_ARR_STRING_71CINE_FULL_REARS:   &'static str = "L R C LFE Ls Rs Lcs Rcs";
pub const SPEAKER_ARR_STRING_71PROXIMITYS:        &'static str = "L R C LFE Ls Rs Pl Pr";
pub const SPEAKER_ARR_STRING_90CINES:             &'static str = "L R C Ls Rs Lc Rc Sl Sr";
pub const SPEAKER_ARR_STRING_91CINES:             &'static str = "L R C Lfe Ls Rs Lc Rc Sl Sr";
pub const SPEAKER_ARR_STRING_100CINES:            &'static str = "L R C Ls Rs Lc Rc Cs Sl Sr";
pub const SPEAKER_ARR_STRING_101CINES:            &'static str = "L R C Lfe Ls Rs Lc Rc Cs Sl Sr";
pub const SPEAKER_ARR_STRING50_4S:                &'static str = "L R C Ls Rs Tfl Tfr Trl Trr";
pub const SPEAKER_ARR_STRING51_4S:                &'static str = "L R C LFE Ls Rs Tfl Tfr Trl Trr";
pub const SPEAKER_ARR_STRING70_2S:                &'static str = "L R C Ls Rs Sl Sr Tsl Tsr";
pub const SPEAKER_ARR_STRING71_2S:                &'static str = "L R C LFE Ls Rs Sl Sr Tsl Tsr";
pub const SPEAKER_ARR_STRING70_4S:                &'static str = "L R C Ls Rs Sl Sr Tfl Tfr Trl Trr";
pub const SPEAKER_ARR_STRING71_4S:                &'static str = "L R C LFE Ls Rs Sl Sr Tfl Tfr Trl Trr";
pub const SPEAKER_ARR_STRING70_6S:                &'static str = "L R C Ls Rs Sl Sr Tfl Tfr Trl Trr Tsl Tsr";
pub const SPEAKER_ARR_STRING71_6S:                &'static str = "L R C LFE Ls Rs Sl Sr Tfl Tfr Trl Trr Tsl Tsr";
pub const SPEAKER_ARR_STRING90_4S:                &'static str = "L R C Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr";
pub const SPEAKER_ARR_STRING91_4S:                &'static str = "L R C LFE Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr";
pub const SPEAKER_ARR_STRING90_6S:                &'static str = "L R C Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr Tsl Tsr";
pub const SPEAKER_ARR_STRING91_6S:                &'static str = "L R C LFE Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr Tsl Tsr";
pub const SPEAKER_ARR_STRING100S:                 &'static str = "L R C Ls Rs Tc Tfl Tfr Trl Trr";
pub const SPEAKER_ARR_STRING101S:                 &'static str = "L R C LFE Ls Rs Tc Tfl Tfr Trl Trr";
pub const SPEAKER_ARR_STRING110S:                 &'static str = "L R C Ls Rs Tc Tfl Tfc Tfr Trl Trr";
pub const SPEAKER_ARR_STRING111S:                 &'static str = "L R C LFE Ls Rs Tc Tfl Tfc Tfr Trl Trr";
pub const SPEAKER_ARR_STRING130S:                 &'static str = "L R C Ls Rs Sl Sr Tc Tfl Tfc Tfr Trl Trr";
pub const SPEAKER_ARR_STRING131S:                 &'static str = "L R C LFE Ls Rs Sl Sr Tc Tfl Tfc Tfr Trl Trr";
pub const SPEAKER_ARR_STRING102S:                 &'static str = "L R C LFE Ls Rs Tfl Tfc Tfr Trl Trr LFE2";
pub const SPEAKER_ARR_STRING122S:                 &'static str = "L R C LFE Ls Rs Lc Rc Tfl Tfc Tfr Trl Trr LFE2";
pub const SPEAKER_ARR_STRING81MPEGS:              &'static str = "L R LFE Ls Rs Tfl Tfc Tfr Bfc";
pub const SPEAKER_ARR_STRING140S:                 &'static str = "L R Ls Rs Sl Sr Tfl Tfr Trl Trr Bfl Bfr Brl Brr";
pub const SPEAKER_ARR_STRING222S:                 &'static str = "L R C LFE Ls Rs Lc Rc Cs Sl Sr Tc Tfl Tfc Tfr Trl Trc Trr LFE2 Tsl Tsr Bfl Bfc Bfr";
pub const SPEAKER_ARR_STRING220S:                 &'static str = "L R C Ls Rs Lc Rc Cs Sl Sr Tc Tfl Tfc Tfr Trl Trc Trr Tsl Tsr Bfl Bfc Bfr";
pub const SPEAKER_ARR_STRING_AMBI1ST_ORDERS:      &'static str = "0 1 2 3";
pub const SPEAKER_ARR_STRING_AMBI2CD_ORDERS:      &'static str = "0 1 2 3 4 5 6 7 8";
pub const SPEAKER_ARR_STRING_AMBI3RD_ORDERS:      &'static str = "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15";

/**
  | Returns number of channels used in speaker
  | arrangement. \ingroup speakerArrangements
  |
  */
#[inline] pub fn speaker_arr_get_channel_count(arr: SpeakerArrangement) -> i32 {
    
    todo!();
        /*
            int32 count = 0;
        while (arr)
        {
            if (arr & (SpeakerArrangement)1)
                ++count;
            arr >>= 1;
        }
        return count;
        */
}

/**
  | Returns the index of a given speaker
  | in a speaker arrangement (-1 if speaker
  | is not part of the arrangement).
  |
  */
#[inline] pub fn speaker_arr_get_speaker_index(
        speaker:     Speaker,
        arrangement: SpeakerArrangement) -> i32 {
    
    todo!();
        /*
            // check if speaker is present in arrangement
        if ((arrangement & speaker) == 0)
            return -1;

        int32 result = 0;
        Speaker i = 1;
        while (i < speaker)
        {
            if (arrangement & i)
                result++;
            i <<= 1;
        }

        return result;
        */
}

/**
  | Returns the speaker for a given index
  | in a speaker arrangement (return 0 when
  | out of range).
  |
  */
#[inline] pub fn speaker_arr_get_speaker(
        arr:   &SpeakerArrangement,
        index: i32) -> Speaker {
    
    todo!();
        /*
            SpeakerArrangement arrTmp = arr;

        int32 index2 = -1;
        int32 pos = -1;
        while (arrTmp)
        {
            if (arrTmp & 0x1)
                index2++;
            pos++;
            if (index2 == index)
                return (Speaker)1 << pos;
            
            arrTmp = arrTmp >> 1;
        }
        return 0;
        */
}

/**
  | Returns true if arrSubSet is a subset
  | speaker of arr (means each speaker of
  | arrSubSet is included in arr).
  |
  */
#[inline] pub fn speaker_arr_is_subset_of(
        arr_sub_set: &SpeakerArrangement,
        arr:         &SpeakerArrangement) -> bool {
    
    todo!();
        /*
            return (arrSubSet == (arrSubSet & arr));
        */
}

/**
  | Returns true if arrangement is a Auro
  | configuration.
  |
  */
#[inline] pub fn speaker_arr_is_auro(arr: &SpeakerArrangement) -> bool {
    
    todo!();
        /*
            if (arr == k90 || arr == k91 || arr == k100 || arr == k101 || arr == k110 || arr == k111 ||
            arr == k130 || arr == k131)
        {
            return true;
        }
        return false;
        */
}

/**
  | Returns true if arrangement contains
  | top (upper layer) speakers
  |
  */
#[inline] pub fn speaker_arr_has_top_speakers(arr: &SpeakerArrangement) -> bool {
    
    todo!();
        /*
            if (arr & SPEAKER_TC || arr & SPEAKER_TFL || arr & SPEAKER_TFC || arr & SPEAKER_TFR ||
            arr & SPEAKER_TRL || arr & SPEAKER_TRC || arr & SPEAKER_TRR || arr & SPEAKER_TSL ||
            arr & SPEAKER_TSR)
            return true;
        return false;
        */
}

/**
  | Returns true if arrangement contains
  | bottom (lower layer) speakers
  |
  */
#[inline] pub fn speaker_arr_has_bottom_speakers(arr: &SpeakerArrangement) -> bool {
    
    todo!();
        /*
            if (arr & SPEAKER_BFL || arr & SPEAKER_BFC || arr & SPEAKER_BFL || arr & SPEAKER_BFC ||
            arr & SPEAKER_BFR)
            return true;
        return false;
        */
}

/**
  | Returns true if arrangement contains
  | middle layer (at ears level) speakers
  |
  */
#[inline] pub fn speaker_arr_has_middle_speakers(arr: &SpeakerArrangement) -> bool {
    
    todo!();
        /*
            if (arr & SPEAKER_L || arr & SPEAKER_R || arr & SPEAKER_C || arr & SPEAKER_LS ||
            arr & SPEAKER_RS || arr & SPEAKER_LC || arr & SPEAKER_RC || arr & SPEAKER_CS ||
            arr & SPEAKER_SL || arr & SPEAKER_SR || arr & SPEAKER_M || arr & SPEAKER_PL ||
            arr & SPEAKER_PR || arr & SPEAKER_LCS || arr & SPEAKER_RCS)
            return true;
        return false;
        */
}

/**
  | Returns true if arrangement contains
  | LFE speakers
  |
  */
#[inline] pub fn speaker_arr_has_lfe(arr: &SpeakerArrangement) -> bool {
    
    todo!();
        /*
            if (arr & SPEAKER_LFE || arr & SPEAKER_LFE2)
            return true;
        return false;
        */
}

/**
  | Returns true if arrangement is a 3D configuration
  | ((top or bottom) and middle)
  |
  */
#[inline] pub fn speaker_arr_is3d(arr: &SpeakerArrangement) -> bool {
    
    todo!();
        /*
            bool top = hasTopSpeakers (arr);
        bool bottom = hasBottomSpeakers (arr);
        bool middle = hasMiddleSpeakers (arr);

        if (((top || bottom) && middle) || (top && bottom))
            return true;
        return false;
        */
}

/**
  | Returns true if arrangement is a Auro
  | configuration.
  |
  */
#[inline] pub fn speaker_arr_is_ambisonics(arr: &SpeakerArrangement) -> bool {
    
    todo!();
        /*
            if (arr == kAmbi1stOrderACN || arr == kAmbi2cdOrderACN || arr == kAmbi3rdOrderACN)
        {
            return true;
        }
        return false;
        */
}

/**
  | Returns the speaker arrangement associated
  | to a string representation.
  | 
  | Returns kEmpty if no associated arrangement
  | is known.
  |
  */
#[inline] pub fn speaker_arr_get_speaker_arrangement_from_string(arr_str: &'static str) -> SpeakerArrangement {
    
    todo!();
        /*
            if (!strcmp8 (arrStr, kStringMono))
            return kMono;
        if (!strcmp8 (arrStr, kStringStereo))
            return kStereo;
        if (!strcmp8 (arrStr, kStringStereoR))
            return kStereoSurround;
        if (!strcmp8 (arrStr, kStringStereoC))
            return kStereoCenter;
        if (!strcmp8 (arrStr, kStringStereoSide))
            return kStereoSide;
        if (!strcmp8 (arrStr, kStringStereoCLfe))
            return kStereoCLfe;
        if (!strcmp8 (arrStr, kStringStereoTF))
            return kStereoTF;
        if (!strcmp8 (arrStr, kStringStereoTS))
            return kStereoTS;
        if (!strcmp8 (arrStr, kStringStereoTR))
            return kStereoTR;
        if (!strcmp8 (arrStr, kStringStereoBF))
            return kStereoBF;
        if (!strcmp8 (arrStr, kStringCineFront))
            return kCineFront; 
        if (!strcmp8 (arrStr, kString30Cine))
            return k30Cine;
        if (!strcmp8 (arrStr, kString30Music))
            return k30Music;
        if (!strcmp8 (arrStr, kString31Cine))
            return k31Cine;
        if (!strcmp8 (arrStr, kString31Music))
            return k31Music;
        if (!strcmp8 (arrStr, kString40Cine))
            return k40Cine;
        if (!strcmp8 (arrStr, kString40Music))
            return k40Music;
        if (!strcmp8 (arrStr, kString41Cine))
            return k41Cine;
        if (!strcmp8 (arrStr, kString41Music))
            return k41Music;
        if (!strcmp8 (arrStr, kString50))
            return k50;
        if (!strcmp8 (arrStr, kString51))
            return k51;
        if (!strcmp8 (arrStr, kString60Cine))
            return k60Cine;
        if (!strcmp8 (arrStr, kString60Music))
            return k60Music;
        if (!strcmp8 (arrStr, kString61Cine))
            return k61Cine;
        if (!strcmp8 (arrStr, kString61Music))
            return k61Music;
        if (!strcmp8 (arrStr, kString70Cine) || !strcmp8 (arrStr, kString70CineOld))
            return k70Cine;
        if (!strcmp8 (arrStr, kString70Music) || !strcmp8 (arrStr, kString70MusicOld))
            return k70Music;
        if (!strcmp8 (arrStr, kString71Cine) || !strcmp8 (arrStr, kString71CineOld))
            return k71Cine;
        if (!strcmp8 (arrStr, kString71Music) || !strcmp8 (arrStr, kString71MusicOld))
            return k71Music;
        if (!strcmp8 (arrStr, kString71Proximity))
            return k71Proximity;
        if (!strcmp8 (arrStr, kString80Cine))
            return k80Cine;
        if (!strcmp8 (arrStr, kString80Music))
            return k80Music;
        if (!strcmp8 (arrStr, kString81Cine))
            return k81Cine;
        if (!strcmp8 (arrStr, kString81Music))
            return k81Music;
        if (!strcmp8 (arrStr, kString102))
            return k102;
        if (!strcmp8 (arrStr, kString122))
            return k122;
        if (!strcmp8 (arrStr, kString80Cube))
            return k80Cube;
        if (!strcmp8 (arrStr, kString71CineTopCenter))
            return k71CineTopCenter;
        if (!strcmp8 (arrStr, kString71CineCenterHigh))
            return k71CineCenterHigh;
        if (!strcmp8 (arrStr, kString71CineFrontHigh))
            return k71CineFrontHigh;
        if (!strcmp8 (arrStr, kString71CineSideHigh))
            return k71CineSideHigh;
        if (!strcmp8 (arrStr, kString71CineFullRear))
            return k71CineFullRear;
        if (!strcmp8 (arrStr, kString90Cine))
            return k90Cine; 
        if (!strcmp8 (arrStr, kString91Cine))
            return k91Cine;
        if (!strcmp8 (arrStr, kString100Cine))
            return k100Cine; 
        if (!strcmp8 (arrStr, kString101Cine))
            return k101Cine;
        if (!strcmp8 (arrStr, kString50_4))
            return k50_4;
        if (!strcmp8 (arrStr, kString51_4))
            return k51_4;
        if (!strcmp8 (arrStr, kString81MPEG))
            return k81MPEG3D;
        if (!strcmp8 (arrStr, kString70_2))
            return k70_2; 
        if (!strcmp8 (arrStr, kString71_2))
            return k71_2;
        if (!strcmp8 (arrStr, kString70_4))
            return k70_4;
        if (!strcmp8 (arrStr, kString71_4))
            return k71_4;
        if (!strcmp8 (arrStr, kString70_6))
            return k70_6;
        if (!strcmp8 (arrStr, kString71_6))
            return k71_6;
        if (!strcmp8 (arrStr, kString90_4))
            return k90_4; 
        if (!strcmp8 (arrStr, kString91_4))
            return k91_4;
        if (!strcmp8 (arrStr, kString90_6))
            return k90_6;
        if (!strcmp8 (arrStr, kString91_6))
            return k91_6;
        if (!strcmp8 (arrStr, kString100))
            return k100;
        if (!strcmp8 (arrStr, kString101))
            return k101;
        if (!strcmp8 (arrStr, kString110))
            return k110;
        if (!strcmp8 (arrStr, kString111))
            return k111;
        if (!strcmp8 (arrStr, kString130))
            return k130;
        if (!strcmp8 (arrStr, kString131))
            return k131;
        if (!strcmp8 (arrStr, kString140))
            return k140;
        if (!strcmp8 (arrStr, kString222))
            return k222;
        if (!strcmp8 (arrStr, kString220))
            return k220;
        if (!strcmp8 (arrStr, kStringAmbi1stOrder))
            return kAmbi1stOrderACN;
        if (!strcmp8 (arrStr, kStringAmbi2cdOrder))
            return kAmbi2cdOrderACN;
        if (!strcmp8 (arrStr, kStringAmbi3rdOrder))
            return kAmbi3rdOrderACN;
        return kEmpty;
        */
}

/**
  | Returns the string representation
  | of a given speaker arrangement.
  | 
  | Returns kStringEmpty if arr is unknown.
  |
  */
#[inline] pub fn speaker_arr_get_speaker_arrangement_string(
        arr:                SpeakerArrangement,
        with_speakers_name: bool) -> &'static str {
    
    todo!();
        /*
            switch (arr)
        {
            case kMono:             return withSpeakersName ? kStringMonoS      : kStringMono;
            case kStereo:           return withSpeakersName ? kStringStereoS    : kStringStereo;
            case kStereoSurround:   return withSpeakersName ? kStringStereoRS   : kStringStereoR;
            case kStereoCenter:     return withSpeakersName ? kStringStereoCS   : kStringStereoC;
            case kStereoSide:       return withSpeakersName ? kStringStereoSS   : kStringStereoSide;
            case kStereoCLfe:       return withSpeakersName ? kStringStereoCLfeS: kStringStereoCLfe;
            case kStereoTF:         return withSpeakersName ? kStringStereoTFS  : kStringStereoTF;
            case kStereoTS:         return withSpeakersName ? kStringStereoTSS  : kStringStereoTS;
            case kStereoTR:         return withSpeakersName ? kStringStereoTRS  : kStringStereoTR;
            case kStereoBF:         return withSpeakersName ? kStringStereoBFS  : kStringStereoBF;
            case kCineFront:        return withSpeakersName ? kStringCineFrontS : kStringCineFront;
            case k30Cine:           return withSpeakersName ? kString30CineS    : kString30Cine;
            case k30Music:          return withSpeakersName ? kString30MusicS   : kString30Music;
            case k31Cine:           return withSpeakersName ? kString31CineS    : kString31Cine;
            case k31Music:          return withSpeakersName ? kString31MusicS   : kString31Music;
            case k40Cine:           return withSpeakersName ? kString40CineS    : kString40Cine;
            case k40Music:          return withSpeakersName ? kString40MusicS   : kString40Music;
            case k41Cine:           return withSpeakersName ? kString41CineS    : kString41Cine;
            case k41Music:          return withSpeakersName ? kString41MusicS   : kString41Music;
            case k50:               return withSpeakersName ? kString50S        : kString50;
            case k51:               return withSpeakersName ? kString51S        : kString51;
            case k60Cine:           return withSpeakersName ? kString60CineS    : kString60Cine;
            case k60Music:          return withSpeakersName ? kString60MusicS   : kString60Music;
            case k61Cine:           return withSpeakersName ? kString61CineS    : kString61Cine;
            case k61Music:          return withSpeakersName ? kString61MusicS   : kString61Music;
            case k70Cine:           return withSpeakersName ? kString70CineS    : kString70Cine;
            case k70Music:          return withSpeakersName ? kString70MusicS   : kString70Music;
            case k71Cine:           return withSpeakersName ? kString71CineS    : kString71Cine;
            case k71Music:          return withSpeakersName ? kString71MusicS   : kString71Music;
            case k71Proximity:      return withSpeakersName ? kString71ProximityS : kString71Proximity;
            case k80Cine:           return withSpeakersName ? kString80CineS    : kString80Cine;
            case k80Music:          return withSpeakersName ? kString80MusicS   : kString80Music;
            case k81Cine:           return withSpeakersName ? kString81CineS    : kString81Cine;
            case k81Music:          return withSpeakersName ? kString81MusicS   : kString81Music;
            case k81MPEG3D:         return withSpeakersName ? kString81MPEGS    : kString81MPEG;
            case k102:              return withSpeakersName ? kString102S       : kString102;
            case k122:              return withSpeakersName ? kString122S       : kString122;
            case k80Cube:           return withSpeakersName ? kString80CubeS    : kString80Cube;
            case k71CineTopCenter:  return withSpeakersName ? kString71CineTopCenterS   : kString71CineTopCenter;
            case k71CineCenterHigh: return withSpeakersName ? kString71CineCenterHighS  : kString71CineCenterHigh;
            case k71CineFrontHigh:  return withSpeakersName ? kString71CineFrontHighS   : kString71CineFrontHigh;
            case k71CineSideHigh:   return withSpeakersName ? kString71CineSideHighS    : kString71CineSideHigh;
            case k71CineFullRear:   return withSpeakersName ? kString71CineFullRearS    : kString71CineFullRear;
            case k90Cine:           return withSpeakersName ? kString90CineS    : kString90Cine; 
            case k91Cine:           return withSpeakersName ? kString91CineS    : kString91Cine;
            case k100Cine:          return withSpeakersName ? kString100CineS   : kString100Cine; 
            case k101Cine:          return withSpeakersName ? kString101CineS   : kString101Cine;
            case k100:              return withSpeakersName ? kString100S       : kString100;
            case k101:              return withSpeakersName ? kString101S       : kString101;
            case k110:              return withSpeakersName ? kString110S       : kString110;
            case k111:              return withSpeakersName ? kString111S       : kString111;

            case k50_4:             return withSpeakersName ? kString50_4S      : kString50_4;
            case k51_4:             return withSpeakersName ? kString51_4S      : kString51_4;
            case k70_2:             return withSpeakersName ? kString70_2S      : kString70_2;
            case k71_2:             return withSpeakersName ? kString71_2S      : kString71_2;
            case k70_4:             return withSpeakersName ? kString70_4S      : kString70_4;
            case k71_4:             return withSpeakersName ? kString71_4S      : kString71_4;
            case k70_6:             return withSpeakersName ? kString70_6S      : kString70_6;
            case k71_6:             return withSpeakersName ? kString71_6S      : kString71_6;
            case k90_4:             return withSpeakersName ? kString90_4S      : kString90_4;
            case k91_4:             return withSpeakersName ? kString91_4S      : kString91_4;
            case k90_6:             return withSpeakersName ? kString90_6S      : kString90_6;
            case k91_6:             return withSpeakersName ? kString91_6S      : kString91_6;
            case k130:              return withSpeakersName ? kString130S       : kString130;
            case k131:              return withSpeakersName ? kString131S       : kString131;
            case k140:              return withSpeakersName ? kString140S       : kString140;
            case k222:              return withSpeakersName ? kString222S       : kString222;
            case k220:              return withSpeakersName ? kString220S       : kString220;
                break;
        }

        if (arr == kAmbi1stOrderACN)
            return withSpeakersName ? kStringAmbi1stOrderS : kStringAmbi1stOrder;
        if (arr == kAmbi2cdOrderACN)
            return withSpeakersName ? kStringAmbi2cdOrderS : kStringAmbi2cdOrder;
        if (arr == kAmbi3rdOrderACN)
            return withSpeakersName ? kStringAmbi3rdOrderS : kStringAmbi3rdOrder;

        return kStringEmpty;
        */
}

/**
  | Returns a &'static str representation of
  | a given speaker in a given arrangement
  |
  */
#[inline] pub fn speaker_arr_get_speaker_short_name(
        arr:   &SpeakerArrangement,
        index: i32) -> &'static str {
    
    todo!();
        /*
            SpeakerArrangement arrTmp = arr;

        bool found = false;
        int32 index2 = -1;
        int32 pos = -1;
        while (arrTmp)
        {
            if (arrTmp & 0x1)
                index2++;
            pos++;
            if (index2 == index)
            {
                found = true;
                break;
            }
            arrTmp = arrTmp >> 1;
        }

        if (!found)
            return "";

        Speaker speaker = (Speaker)1 << pos;
        if (speaker == SPEAKER_L)
            return "L";
        if (speaker == SPEAKER_R)
            return "R";
        if (speaker == SPEAKER_C)
            return "C";
        if (speaker == SPEAKER_LFE)
            return "LFE";
        if (speaker == SPEAKER_LS)
            return "Ls";
        if (speaker == SPEAKER_RS)
            return "Rs";
        if (speaker == SPEAKER_LC)
            return "Lc";
        if (speaker == SPEAKER_RC)
            return "Rc";
        if (speaker == SPEAKER_CS)
            return "S";
        if (speaker == SPEAKER_SL)
            return "Sl";
        if (speaker == SPEAKER_SR)
            return "Sr";
        if (speaker == SPEAKER_TC)
            return "Tc";
        if (speaker == SPEAKER_TFL)
            return "Tfl";
        if (speaker == SPEAKER_TFC)
            return "Tfc";
        if (speaker == SPEAKER_TFR)
            return "Tfr";
        if (speaker == SPEAKER_TRL)
            return "Trl";
        if (speaker == SPEAKER_TRC)
            return "Trc";
        if (speaker == SPEAKER_TRR)
            return "Trr";
        if (speaker == SPEAKER_LFE2)
            return "LFE2";
        if (speaker == SPEAKER_M)
            return "M";

        if (speaker == SPEAKER_ACN0)
            return "0";
        if (speaker == SPEAKER_ACN1)
            return "1";
        if (speaker == SPEAKER_ACN2)
            return "2";
        if (speaker == SPEAKER_ACN3)
            return "3";
        if (speaker == SPEAKER_ACN4)
            return "4";
        if (speaker == SPEAKER_ACN5)
            return "5";
        if (speaker == SPEAKER_ACN6)
            return "6";
        if (speaker == SPEAKER_ACN7)
            return "7";
        if (speaker == SPEAKER_ACN8)
            return "8";
        if (speaker == SPEAKER_ACN9)
            return "9";
        if (speaker == SPEAKER_ACN10)
            return "10";
        if (speaker == SPEAKER_ACN11)
            return "11";
        if (speaker == SPEAKER_ACN12)
            return "12";
        if (speaker == SPEAKER_ACN13)
            return "13";
        if (speaker == SPEAKER_ACN14)
            return "14";
        if (speaker == SPEAKER_ACN15)
            return "15";

        if (speaker == SPEAKER_TSL)
            return "Tsl";
        if (speaker == SPEAKER_TSR)
            return "Tsr";
        if (speaker == SPEAKER_LCS)
            return "Lcs";
        if (speaker == SPEAKER_RCS)
            return "Rcs";

        if (speaker == SPEAKER_BFL)
            return "Bfl";
        if (speaker == SPEAKER_BFC)
            return "Bfc";
        if (speaker == SPEAKER_BFR)
            return "Bfr";
        if (speaker == SPEAKER_PL)
            return "Pl";
        if (speaker == SPEAKER_PR)
            return "Pr";
        if (speaker == SPEAKER_BSL)
            return "Bsl";
        if (speaker == SPEAKER_BSR)
            return "Bsr";
        if (speaker == SPEAKER_BRL)
            return "Brl";
        if (speaker == SPEAKER_BRC)
            return "Brc";
        if (speaker == SPEAKER_BRR)
            return "Brr";

        return "";
        */
}
