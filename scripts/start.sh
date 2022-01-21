#!/bin/bash

# Get the path of the shell script
SCRIPT_LOCATION=`dirname $(realpath $0)`

# This script will start a notify container using latest
echo 'Starting notify container'
docker run -d --name notify --mount type=bind,source=$SCRIPT_LOCATION/../testing/config,target=/app/config --mount type=bind,source=$SCRIPT_LOCATION/../testing/notifications,target=/app/notifications notify:latest 