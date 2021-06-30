#!/bin/bash
VAR="This is some var"
set -x
echo "$VAR"
set +x
echo "Another $VAR"

set -x
ls "$VAR"
echo $VAR
