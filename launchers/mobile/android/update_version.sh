#!/bin/bash

VERSIONS=($(echo "$NEW_VERSION" | awk -F'.' '{ for(i=1;i<=NF;i++) print $i }'))

echo -e "VERSION_MAJOR=${VERSIONS[0]}\nVERSION_MINOR=${VERSIONS[1]}\nVERSION_PATCH=${VERSIONS[2]}">$WORKSPACE_ROOT/launchers/mobile/android/version.properties
