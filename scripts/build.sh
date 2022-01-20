#!/bin/bash

# Get the path of the shell script
SCRIPT_LOCATION=`dirname $(realpath $0)`

# This script will build the docker image
echo 'Building notify image'
docker build --tag notify $SCRIPT_LOCATION/../.