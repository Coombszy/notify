#!/bin/bash

# This script will start a notify container using latest
echo 'Starting notify container'
docker run -d --name notify notify:latest