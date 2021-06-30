#!/bin/bash

#echo "Running $0"

# this starts from $1
if [ $1 ]; then
    for ARG in $@; do
        echo "Your argument: [${ARG}]"
    done
else
    echo "No args"
fi
