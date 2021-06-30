#!/bin/bash

read -p "Type a hostname to reach: " HOST

ping -c 1 $HOST 1>/dev/null

if [ "$?" -ne 0 ]; then
	echo "Host is unreachable"
else
	echo "$HOST can be reached"
fi