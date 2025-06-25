import time


class System:
    @staticmethod
    def sleep(*, milliseconds: int):
        if milliseconds <= 0:
            return
        time.sleep(milliseconds / 1000.0)
        return
