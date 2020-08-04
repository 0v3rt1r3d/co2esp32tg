#!/usr/bin/env bash
set -e
while true; do
    curl -X POST -d '{"timestamp":1596490254,"temperature":23.84,"humidity":42.005,"co2":578,"pressure":982.940}' http://localhost:443/sensors
    curl http://localhost:443
    echo ""
    sleep 5
done
