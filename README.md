Audio Media processing and playing distributed platform (actix & tokio), composed of:
- a DSP (digital processing library)
- an Audio Player library (controlling the media player)
- CLI (command line interface) for common operations over audio files (.wav supported only at the moment)
- using actos for user & audio player management 

The CLI currently covers the following operations:
- Load audio to CLI
- Upload processed audio to disk
- Copy audio file to
- Gain
- Normalize
- Low Pass Filter - applies low pass filter over a audio file
- High Pass Filter - applies high pass filter over a audio file

It also contains commands for playing audio files:
 - start/stop/pause/seek
