#!/usr/bin/env bash
set -e
SERVER_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
echo $SERVER_DIR
(cd $SERVER_DIR && cargo build --release --color=always)
systemctl stop co2esp32tg
cp $SERVER_DIR/target/release/co2esp32tg /usr/local/bin/co2esp32tg
systemctl start co2esp32tg

