#!/bin/bash

# This script will remove notify container
echo 'Removing notify container'
docker stop notify
docker rm notify