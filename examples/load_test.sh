#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <number_of_requests>"
    exit 1
fi

parallel ./hello.sh ::: $(seq 1 $1)

