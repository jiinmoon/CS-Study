#!/bin/bash

SCRIPTS=$(ls *.sh)
DATE=$(date +%F)

for SCRIPT in $SCRIPTS; do
    echo "change $SCRIPT to ${DATE}_${SCRIPT}"
    mv $SCRIPT ${DATE}_${SCRIPT} 
done
