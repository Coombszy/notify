#!/bin/bash

# Get the path of the shell script
SCRIPT_LOCATION=`dirname $(realpath $0)`

# This script will run all the sub scripts to stop container, delete image, build image, start container
echo 'Refreshing all!'
/bin/sh $SCRIPT_LOCATION/stop.sh
/bin/sh $SCRIPT_LOCATION/clean.sh
/bin/sh $SCRIPT_LOCATION/build.sh
/bin/sh $SCRIPT_LOCATION/start_attached.sh
