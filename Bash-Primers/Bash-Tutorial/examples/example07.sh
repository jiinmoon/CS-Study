#!/bin/bash

function say_hi()
{
    local GREETING="Hello, you wonderful diamond!"
    for VAL in $@; do
        echo $GREETING
        echo "Nice to meet you ${VAL}!"
    done
}

say_hi Jack Neil Roy
