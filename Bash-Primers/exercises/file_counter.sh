#!/bin/bash

# counts number of files under DIR
# and report on terminal STDOUT
function count_files() 
{
    local FILE_COUNT=$(ls -a $1 | wc -l)
    echo "${1}:"; echo $FILE_COUNT
}

for DIR in $@; do
    if [ ! -d $DIR ]; then
        echo "$DIR is not a valid directory."
    else
        count_files $DIR
    fi
done
