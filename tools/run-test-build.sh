#!/bin/sh

SCRIPT=`realpath "${0}"`
SCRIPTPATH=`dirname "${SCRIPT}"`
PROJECTROOT=`realpath "${SCRIPTPATH}"/..`

cargo +nightly run --manifest-path "${PROJECTROOT}"/tools/test-build/Cargo.toml --release -- --repo "${PROJECTROOT}"
