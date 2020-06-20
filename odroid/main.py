from bme280 import readBME280All
from datetime import datetime
from mh_z19 import get_co2

import json
import time


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
    print(json.dumps(get_values()))
