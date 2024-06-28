#!/bin/bash

# Loop to start 100 processes each with a different charger
for i in $(seq -f "%03g" 1 100)
do
    echo "Starting Charger$i"
    ./target/debug/ocpp_conductor_generic Charger$i &
done

# Wait for all background processes to finish
wait
