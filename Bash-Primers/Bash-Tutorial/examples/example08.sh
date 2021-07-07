#!/bin/bash
LN=1

grep is ./test.txt | while read FW SW REST; do
    echo "${LN}: ${FW}"
    echo "${SW} == ${REST}"
    ((LN++))
done
