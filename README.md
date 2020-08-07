# co2esp32tg

## What is this?

<screenshot>

That is "co2 sensor data transfer from esp32 to telegram bot". The general scheme of the service is shown below.

<scheme>

Two sensors, MH-z19b and bme280, are connected to an odroid device. A systemd-service tries to send retrieved from sensors data to web-application, which collects data and is awaited for a request from telegram bot. User might request charts for all time or the last sensors readings.

Odroid service is just a python script with while-loop, which reads from sensors, tries to send data to all defined urls and sleep for 5 minutes.

Web-application was written in rust using rocket web framework and plotters library for drawing charts.

Internal data format is quite simple:
```
{
    "timestamp": value,
    "co2": value,
    "humidity": value,
    "temperature": value,
    "pressure": value
}
```

## Usage (TODO)

