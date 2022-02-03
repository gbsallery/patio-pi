# patio-pi

To compile the C parts:

`gcc -Wall -o rpi_pixleds rpi_pixleds.c rpi_dma_utils.c`

Then run with e.g.:

`sudo ./rpi_pixleds -n 80 -t`

To compile Rust for Raspberry Pi, install the `armv7-unknown-linux-gnueabihf` target architecture. Also 
`brew install arm-linux-gnueabihf-binutils` may be helpful.