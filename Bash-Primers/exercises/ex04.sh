#!/bin/bash

if [ ! -e "$1" ]; then
	echo "$1 does not exist."
	exit 1
elif [ -d "$1" ]; then 
	echo "$1 is a directory."
elif [ -f "$1" ]; then
	echo "$1 is a regular file."
fi

exit 0
