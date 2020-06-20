#!/usr/bin/env python

from bme280 import readBME280All
from datetime import datetime
from mh_z19 import get_co2

import json
import requests
import signal
import sys
import syslog
import time

interrupted = False

def sigint_handler(sig, frame):
    global interrupted
    print("Fetching sensors daemon is stopping...", file=sys.stderr)
    interrupted = True
    syslog.syslog(syslog.LOG_INFO, "Fetching sensors info was stopped")


def get_values():
    timestamp = time.mktime(datetime.now().timetuple())
    (temperature, pressure, humidity) = readBME280All()
    co2 = get_co2()
    return {
        "timestamp": int(timestamp),
        "co2": co2,
        "humidity": humidity,
        "pressure": pressure,
        "temperature" : temperature
    }

if __name__ == "__main__":
    signal.signal(signal.SIGINT, sigint_handler)
    sleep_time_s = 1
    while not interrupted:
        data = json.dumps(get_values())
        response = requests.post(url = "http://192.168.1.66:443/sensors", data = data)
        print(
            "Sent sensors data, http code = {}; body = {}".format(
                response.status_code,
                response.text
            ),
            file=sys.stderr
        )
        time.sleep(sleep_time_s)
