#!/bin/bash

MSG="Shell scripting can be fun...!"

echo $MSG

HOSTNAME=$(hostname)

echo "Currently, this script is running on $HOSTNAME"

if [[ -e /etc/shells && ! -w /etc/shells ]]; then
    echo "We have something!"
else
    echo "Do we have something?"
fi

PETS="dog cat bird"

for VAL in $PETS; do
    echo $VAL
done
