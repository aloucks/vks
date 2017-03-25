#!/bin/sh

SCRIPT=`realpath "${0}"`
SCRIPTPATH=`dirname "${SCRIPT}"`
PROJECTROOT=`realpath "${SCRIPTPATH}"/..`

function test_build() {
    echo "Building feature $1"
    cargo build -v --no-default-features --features $1 || exit 1
    echo ""

    echo "Building feature $1 + clippy"
    cargo build -v --no-default-features --features "$1 clippy" || exit 1
    echo ""
}

cd "${PROJECTROOT}"

test_build vk_1_0_3
test_build vk_1_0_4
test_build vk_1_0_5
test_build vk_1_0_6
test_build vk_1_0_7
