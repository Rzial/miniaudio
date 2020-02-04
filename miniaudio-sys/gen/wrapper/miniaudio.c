#ifdef DR_FLAC_IMPLEMENTATION
    /* Enables FLAC decoding. */
    #include "../../vendor/extras/dr_flac.h"
#endif

#ifdef DR_MP3_IMPLEMENTATION
    /* Enables MP3 decoding. */
    #include "../../vendor/extras/dr_mp3.h"
#endif

#ifdef MA_VORBIS_DECODER
    /* Enables VORBIS decoding. */
    #include "../../vendor/extras/stb_vorbis.c"
#endif

#ifdef DR_WAV_IMPLEMENTATION
/* Enables WAV decoding. */
    #include "../../vendor/extras/dr_wav.h"
#endif

#define MINIAUDIO_IMPLEMENTATION
#include "../../vendor/miniaudio.h"
