#!/bin/bash

if test -z ${1+any}; then
    echo "Argument 1 required: tag"
    exit 1
fi

version="$1"

git tag -a -m "Release ${version}; https://crates.io/crates/secr/${version}" "${version}"
git push --tag

