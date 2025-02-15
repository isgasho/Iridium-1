 pub const SYMBOL: [&str;118] = [
  
       "H"  , "He" , "Li" , "Be" , "B"  , "C"  , "N"  , "O"  , "F" ,
       "Ne" , "Na" , "Mg" , "Al" , "Si" , "P"  , "S"  , "Cl" , "Ar",
       "K"  , "Ca" , "Sc" , "Ti" , "V"  , "Cr" , "Mn" , "Fe" , "Co",
       "Ni" , "Cu" , "Zn" , "Ga" , "Ge" , "As" , "Se" , "Br" , "Kr",
       "Rb" , "Sr" , "Y"  , "Zr" , "Nb" , "Mo" , "Tc" , "Ru" , "Rh",
       "Pd" , "Ag" , "Cd" , "In" , "Sn" , "Sb" , "Te" , "I"  , "Xe",
       "Cs" , "Ba" , "La" , "Ce" , "Pr" , "Nd" , "Pm" , "Sm" , "Eu",
       "Gd" , "Tb" , "Dy" , "Ho" , "Er" , "Tm" , "Yb" , "Lu" , "Hf",
       "Ta" , "W"  , "Re" , "Os" , "Ir" , "Pt" , "Au" , "Hg" , "Tl",
       "Pb" , "Bi" , "Po" , "At" , "Rn" , "Fr" , "Ra" , "Ac" , "Th",
       "Pa" , "U"  , "Np" , "Pu" , "Am" , "Cm" , "Bk" , "Cf" , "Es",
       "Fm" , "Md" , "No" , "Lr" , "Rf" , "Db" , "Sg" , "Bh" , "Hs",
       "Mt" , "Ds" , "Rg" , "Cn" , "Nh" , "Fl" , "Mc" , "Lv" , "Ts",
       "Og" ,

];
//
// (start_index, first_isotope,last_isotope)
 pub const SYMBOL_INDEX : [(usize,usize,usize);118] = [
//          H                He               Li               Be       
     (   0,  1,  7) , (   7,  2, 10) , (  16,  3, 13) , (  27,  6, 16) ,
//          B                C                N                O
     (  38,  7, 21) , (  53,  8, 22) , (  68, 10, 25) , (  84, 11, 26) ,
//          F                Ne               Na               Mg
     ( 100, 13, 31) , ( 119, 15, 34) , ( 139, 18, 39) , ( 161, 19, 40) , 
//          Al               Si               P                S 
     ( 183, 22, 43) , ( 205, 22, 44) , ( 228, 25, 47) , ( 251, 27, 49) , 
//          Cl               Ar               K                Ca
     ( 274, 28, 52) , ( 299, 29, 54) , ( 325, 31, 59) , ( 354, 35, 60) ,   
//          Sc               Ti               V                Cr
     ( 380, 39, 61) , ( 403, 38, 64) , ( 429, 40, 65) , ( 455, 42, 67) ,   
//          Mn               Fe               Co               Ni
     ( 481, 44, 69) , ( 507, 45, 72) , ( 535, 47, 75) , ( 564, 48, 80) ,   
//          Cu               Zn               Ga               Ge
     ( 597, 52, 80) , ( 626, 54, 83) , ( 656, 56, 86) , ( 687, 58, 89) ,   
//          As               Se               Br               Kr
     ( 719, 60, 92) , ( 752, 65, 94) , ( 782, 68,101) , ( 816, 69,102) ,   
//          Rb               Sr               Y                Zr                 
     ( 850, 71,106) , ( 886, 73,108) , ( 922, 76,111) , ( 958, 78,114) ,   
//          Nb               Mo               Tc               Ru
     ( 995, 81,117) , (1032, 83,115) , (1065, 85,118) , (1099, 87,120) ,   
//          Rh               Pd               Ag               Cd
     (1133, 89,122) , (1167, 91,129) , (1206, 93,132) , (1246, 95,132) ,   
//          In               Sn               Sb               Te
     (1284, 97,137) , (1325, 99,139) , (1366,103,139) , (1403,104,142) ,    
//          I                Xe               Cs               Ba
     (1442,108,144) , (1479,108,147) , (1519,112,151) , (1559,114,153) ,   
//          La               Ce               Pr               Nd
     (1599,117,155) , (1638,119,157) , (1677,121,159) , (1716,124,161) ,                                  
//          Pm               Sm               Eu               Gd
     (1754,126,163) , (1792,128,165) , (1830,130,167) , (1868,134,169) ,    
//          Tb               Dy               Ho               Er
     (1904,135,171) , (1941,138,173) , (1977,140,175) , (2013,142,177) ,   
//          Tm               Yb               Lu               Hf
     (2049,145,179) , (2084,148,182) , (2119,150,184) , (2154,153,188) ,       
//          Ta               W                Re               Os
     (2190,155,190) , (2226,158,192) , (2261,160,194) , (2296,161,197) ,    
//          Ir               Pt               Au               Hg  
     (2333,164,202) , (2372,165,204) , (2412,169,206) , (2450,170,216) ,   
//          Tl               Pb               Bi               Po  
     (2497,176,216) , (2538,178,218) , (2579,184,220) , (2616,186,222) ,    
//          At               Rn               Fr               Ra    
     (2653,191,224) , (2687,195,231) , (2724,199,232) , (2758,202,234) ,   
//          Ac               Th               Pa               U
     (2791,205,236) , (2823,208,238) , (2854,211,240) , (2884,214,242) ,    
//          Np               Pu               Am               Cm
     (2913,219,244) , (2939,228,247) , (2959,223,247) , (2984,233,251) ,   
//          Bk               Cf               Es               Fm            
     (3003,233,253) , (3024,237,256) , (3044,240,257) , (3062,241,260) ,    
//          Md               No               Lr               Rf 
     (3082,244,260) , (3099,250,262) , (3112,251,266) , (3128,253,270) ,   
//          Db               Sg               Bh               Hs
     (3146,255,270) , (3162,258,271) , (3176,260,278) , (3195,263,277) ,    
//          Mt               Ds               Rg               Cn 
     (3210,266,282) , (3227,267,281) , (3242,272,286) , (3257,277,286) ,   
//          Nh               Fl               Mc               Lv 
     (3267,278,290) , (3280,284,290) , (3287,287,290) , (3291,290,294) ,    
//          Ts              Og 
     (3296,293,294) , (3298,294,294) ,    

  
];
