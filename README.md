# patio-pi

To compile the C parts:

gcc rpi_pixleds.c

Then run with e.g.:

sudo ./rpi_pixleds -n 80 -t

To compile Rust for Raspberry Pi, install the `armv7-unknown-linux-gnueabihf` target architecture. Also 
`brew install arm-linux-gnueabihf-binutils` may be helpful.