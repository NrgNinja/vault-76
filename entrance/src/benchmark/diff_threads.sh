#!/bin/bash

k="$1"

echo ----------------------------------------8 Threads---------------------------------------------
/bin/bash generate_multiple.sh "$k" 8

echo ----------------------------------------16 Threads---------------------------------------------
/bin/bash generate_multiple.sh "$k" 16
