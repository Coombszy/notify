#!/bin/bash

# Get the path of the shell script
SCRIPT_LOCATION=`dirname $(realpath $0)`

# This script will run all the sub scripts to stop container, delete image
echo 'Stopping and cleaning!'
/bin/sh $SCRIPT_LOCATION/stop.sh
/bin/sh $SCRIPT_LOCATION/clean.sh
