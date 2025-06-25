import time
from kiwi import ScreenClient, Point, RelativePoint, RgbOffset, System, Key

client = ScreenClient()
system = System()

while True:
    data = client.get_mouse_location().data
    print("location:", data, data.x, data.y)
    system.sleep(milliseconds=1000)
