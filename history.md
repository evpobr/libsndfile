# History

My first attempt at reading and writing WAV files was in 1990 or so
under Windows 3.1. I started using Linux in early 1995 and contributed
some code to the [wavplay](http://www.vaxxine.com/ve3wwg/gnuwave.html)
program. That contributed code would eventually mutate into this
library. As one of my interests is Digital Signal Processing (DSP) I
decided that as well as reading data from an audio file in the native
format (typically 16 bit short integers) it would also be useful to be
able to have the library do the conversion to floating point numbers for
DSP applications. It then dawned on me that whatever file format
(anything from 8 bit unsigned chars, to 32 bit floating point numbers)
the library should be able to convert the data to whatever format the
library user wishes to use it in. For example, in a sound playback
program, the library caller typically wants the sound data in 16 bit
short integers to dump into a sound card even though the data in the
file may be 32 bit floating point numbers (ie Microsoft's
WAVE\_FORMAT\_IEEE\_FLOAT format). Another example would be someone
doing speech recognition research who has recorded some speech as a 16
bit WAV file but wants to process it as double precision floating point
numbers.

Here is the release history for libsndfile :

- Version 0.0.8 (Feb 15 1999) First official release.
- Version 0.0.28 (Apr 26 2002) Final release of version 0 of
  libsndfile.
- Version 1.0.0rc1 (Jun 24 2002) Release candidate 1 of version 1 of
  libsndfile.
- Version 1.0.0rc6 (Aug 14 2002) MacOS 9 fixes.
- Version 1.0.0 (Aug 16 2002) First 1.0.X release.
- Version 1.0.1 (Sep 14 2002) Added MAT4 and MAT5 file formats.
- Version 1.0.2 (Nov 24 2002) Added VOX ADPCM format.
- Version 1.0.3 (Dec 09 2002) Fixes for Linux on ia64 CPUs.
- Version 1.0.4 (Feb 02 2003) New file formats and functionality.
- Version 1.0.5 (May 03 2003) One new file format and new
  functionality.
- Version 1.0.6 (Feb 08 2004) Large file fix for Linux/Solaris, new
  functionality and Win32 improvements.
- Version 1.0.7 (Feb 24 2004) Fix build problems on MacOS X and fix
  ia64/MIPS etc clip mode detction.
- Version 1.0.8 (Mar 14 2004) Minor bug fixes.
- Version 1.0.9 (Mar 30 2004) Add AVR format. Improve handling of some
  WAV files.
- Version 1.0.10 (Jun 15 2004) Minor bug fixes. Fix support for Win32
  MinGW compiler.
- Version 1.0.11 (Nov 15 2004) Add SD2 file support, reading of loop
  data in WAV and AIFF. Minor bug fixes.
- Version 1.0.12 (Sep 30 2005) Add FLAC and CAF file support, virtual
  I/O interface. Minor bug fixes and cleanups.
- Version 1.0.13 (Jan 21 2006) Add read/write of instrument chunks.
  Minor bug fixes.
- Version 1.0.14 (Feb 19 2006) Minor bug fixes. Start shipping windows
  binary/source ZIP.
- Version 1.0.15 (Mar 16 2006) Minor bug fixes.
- Version 1.0.16 (Apr 30 2006) Add support for RIFX. Other minor
  feature enhancements and bug fixes.
- Version 1.0.17 (Aug 31 2006) Add C++ wrapper sndfile.hh. Minor bug
  fixes and cleanups.
- Version 1.0.18 (Feb 07 2009) Add Ogg/Vorbis suppport, remove captive
  libraries, many new features and bug fixes. Generate Win32 and Win64
  pre-compiled binaries.
- Version 1.0.19 (Mar 02 2009) Fix for CVE-2009-0186. Huge number of
  minor fixes as a result of static analysis.
- Version 1.0.20 (May 14 2009) Fix for potential heap overflow.
- Version 1.0.21 (December 13 2009) Bunch of minor bug fixes.
- Version 1.0.22 (October 04 2010) Bunch of minor bug fixes.
- Version 1.0.23 (October 10 2010) Minor bug fixes.
- Version 1.0.24 (March 23 2011) Minor bug fixes.
- Version 1.0.25 (July 13 2011) Fix for Secunia Advisory SA45125.
  Minor bug fixes and improvements.
- Version 1.0.26 (November 22 2015) Fix for CVE-2014-9496,
  CVE-2014-9756 and CVE-2015-7805. Add ALAC/CAF support. Minor bug
  fixes and improvements.
- Version 1.0.27 (June 19 2016) Fix a seek regression in 1.0.26. Add
  metadata read/write for CAF and RF64. FIx PAF endian-ness issue.
- Version 1.0.28 (April 2 2017) Fix buffer overruns in FLAC and ID3
  handling code. Reduce default header memory requirements. Fix
  detection of Large File Support for 32 bit systems.
- Version 1.0.29 (August 15 2020) Opus support, build system
  improvements and bug fixes.
