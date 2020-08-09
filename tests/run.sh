#!/bin/bash

# This script is required because we need to start a freeradius mock server to
# run radius integration tests

export LD_LIBRARY_PATH=./build/kqueue/

cp ./tests/authorize build/freeradius/dist/etc/raddb/mods-config/files/authorize

./build/freeradius/dist/sbin/radiusd -f -l stdout &
PID=$!


ps -p $PID

if [ $? -ne 0 ]
then
  echo $PID
  echo "Cannot start mock server"
  exit 1
fi

# Test auth client binary
cargo run --bin radius_auth_client -- -c tests/config.toml -u testing -p password

RES=$?

# Run cargo tests
cargo test

RES=$(($? + RES))


kill $PID

exit $RES
