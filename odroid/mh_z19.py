import serial
import time

ser = serial.Serial('/dev/ttyS2',
                      baudrate=9600,
                      bytesize=serial.EIGHTBITS,
                      parity=serial.PARITY_NONE,
                      stopbits=serial.STOPBITS_ONE,
                      timeout=10.0)

# https://mysku.ru/blog/aliexpress/59397.html
# Function to calculate MH-Z19 crc according to datasheet
def crc8(input):
    crc = 0x00
    count = 1
    b = bytearray(input)
    while count < 8:
        crc += b[count]
        count = count + 1
    # Truncate to 8 bit
    crc %= 256
    # Invert number with xor
    crc = ~crc & 0xFF
    crc += 1
    return crc


ser.write(b"\xFF\x01\x99\x00\x00\x00\x13\x88\xCB")
s=ser.read(9)
z=bytearray(s)
if crc8(s) != z[8]:
    raise EnvironmentError("Can not set correct range")

def get_co2_internal():
    result=ser.write(b"\xff\x01\x86\x00\x00\x00\x00\x00\x79")
    time.sleep(0.1)
    s=ser.read(9)
    z=bytearray(s)
    if crc8(s) == z[8] and s[0] == 0xff and s[1] == 0x86:
        return s[2]*256 + s[3]
    else:
        return -1


def get_co2():
    counter = 4
    co2 = -1
    while co2 == -1 and counter > 0:
        co2 = get_co2_internal()
        counter -= 1
    return co2
