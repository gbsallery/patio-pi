#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@192.168.1.183
readonly TARGET_PATH=/home/pi/patio-pi/patio-pi
readonly TARGET_ARCH=arm-unknown-linux-musleabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/patio-pi

cargo build --release --target=${TARGET_ARCH}
rsync --progress ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
#ssh -t ${TARGET_HOST} ${TARGET_PATH}
