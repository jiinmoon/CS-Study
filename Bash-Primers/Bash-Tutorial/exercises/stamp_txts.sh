#!/bin/bash
# time stamps all .txt files in current directory

function time_stamp()
{
    local CURR_TIME=$(date +%F)
    echo "Changing $1 to ${CURR_TIME}_$1"
    mv $1 ./${CURR_TIME}_$1
}



for FILE in *.txt; do
    time_stamp $FILE
done
