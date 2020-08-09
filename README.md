# co2esp32tg

## What is this?

This is the co2 sensor data transfer from odroid (initially the esp32 was planned to use) to telegram bot. There are several ideas //

![There should be gif animation from telegram](doc/telegram.gif "Telegram")

There are two systemd-services, one is launched on odroid device with connected sensors and sends readings to the second one, which is launched in a cloud (I used a digitalocean virtual machine). The web application takes the data, saves it into database and expects for requests from telegram bot. There are two main requests available now: the last sensors readings and charts with historical data.

![There should be scheme of system](doc/scheme.png "Scheme")

## Details

Two sensors, MH-z19 (co2) and bme280 (humidity, pressure, temperature), are connected to an odroid device where a simple python script tries to send data to specified urls. Web application is exppected for sensors readings and telegram bot requests. Readings are saved into database and used for drawing charts and pushing them to user response.

Internal data format (json) is quite simple:
```
{
    "timestamp": value,
    "co2": value,
    "humidity": value,
    "temperature": value,
    "pressure": value
}
```

## Usage

You might want to launch your own bot implementation, so follow the steps:
1) Buy odroid (or raspberry) device, MH-z19 and bme280 sensors. Fill servicemd template and enable service.
2) Create a virtual machine with public IP, fill `server` systemd confing (ssl-sertificates, working directory, etc), and enable it.
3) Create a new telegram bot and set web hook for your virtual host.

