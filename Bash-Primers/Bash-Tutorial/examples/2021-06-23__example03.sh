#!/bin/bash

CURR_SHELL="zsh"
echo "I am using $CURR_SHELL"
echo "I am using ${CURR_SHELL}...!"

MY_HOSTNAME=$(hostname)
echo "Script is running on $MY_HOSTNAME"

if [ $MY_HOSTNAME = "THIS SHOULD BE FALSE" ]; then
    echo "THIS SHOULD NOT BE PRINTED"
else
    echo "THIS SHOULD BE PRINTED"
fi

if [ $MY_HOSTNAME = "Franciscos-MacBook-Air.local" ]; then
    echo "YES!"
else
    echo "NO!"
fi

if [ $SHELL = "/bin/zsh" ]
then
    echo "Using ${SHELL}!"
fi

DIRECTIONS="N W E S"
for DIRECTION in $DIRECTIONS
do
    echo "DIRECTION: ${DIRECTION}"
done
