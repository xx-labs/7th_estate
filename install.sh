#!/bin/bash

if [[ $1 ]]; then
    version=$1
else
    version="debug"
fi

echo "Installing $version version"

cargo build
cp target/$version/seventh-estate .
