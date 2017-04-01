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
test_build vk_1_0_21
test_build vk_1_0_22
test_build vk_1_0_23
test_build vk_1_0_24
test_build vk_1_0_25
test_build vk_1_0_26
test_build vk_1_0_27
test_build vk_1_0_28
test_build vk_1_0_29
test_build vk_1_0_30
test_build vk_1_0_31
test_build vk_1_0_32
test_build vk_1_0_33
test_build vk_1_0_34
test_build vk_1_0_35
test_build vk_1_0_36
test_build vk_1_0_37
test_build vk_1_0_38
test_build vk_1_0_39
test_build vk_1_0_40
test_build vk_1_0_41
test_build vk_1_0_42
