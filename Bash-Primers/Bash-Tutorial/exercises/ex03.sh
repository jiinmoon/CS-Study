#!/bin/bash
sleep 5 &

PID=$!
echo "PID: $PID"
wait $PID
echo "DONE! $?"
