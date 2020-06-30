#!/usr/bin/env bash
set -e

TARGET="192.168.1.75"
TARGET_HOME="/home/overtired"

SRC=$(cd $(dirname $0) && pwd -P)

scp -r $SRC/* $TARGET:$TARGET_HOME
