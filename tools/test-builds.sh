#!/bin/sh

SCRIPT=`realpath "${0}"`
SCRIPTPATH=`dirname "${SCRIPT}"`
PROJECTROOT=`realpath "${SCRIPTPATH}"/..`

function test_build() {
    echo "Building feature $1"
    cargo check --no-default-features --features $1 || exit 1
    echo ""

    echo "Building feature $1 + clippy"
    cargo check --no-default-features --features "$1 clippy" || exit 1
    echo ""
}

cd "${PROJECTROOT}"

test_build vk_1_0_3
test_build vk_1_0_4
test_build vk_1_0_5
test_build vk_1_0_6
test_build vk_1_0_7
test_build vk_1_0_8
test_build vk_1_0_9
test_build vk_1_0_10
test_build vk_1_0_11
test_build vk_1_0_12
test_build vk_1_0_13
test_build vk_1_0_14
test_build vk_1_0_15
test_build vk_1_0_16
test_build vk_1_0_17
test_build vk_1_0_18
test_build vk_1_0_19
test_build vk_1_0_20
