# Features

libsndfile has the following main features :

- Ability to read and write a large number of [file
  formats](formats.md).
- A simple, elegant and easy to use Applications Programming
  Interface.
- Usable on Unix, Win32, MacOS and others.
- On the fly format conversion, including endian-ness swapping, type
  conversion and bitwidth scaling.
- Optional normalisation when reading floating point data from files
  containing integer data.
- Ability to open files in read/write mode.
- The ability to write the file header without closing the file (only
  on files open for write or read/write).
- Ability to query the library about all supported formats and
  retrieve text strings describing each format.

libsndfile has a comprehensive test suite so that each release is as bug
free as possible. When new bugs are found, new tests are added to the
test suite to ensure that these bugs don't creep back into the code.
When new features are added, tests are added to the test suite to make
sure that these features continue to work correctly even when they are
old features.
