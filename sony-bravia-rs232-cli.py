#!/usr/bin/env python3
import argparse

import serial


DEV = '/dev/serial/by-id/usb-FTDI_FT232R_USB_UART_B001CPKZ-if00-port0'


class Sony:
    command_interval = 0.5  # Sony recommends 0.5, but 0.15 works reliably

    CONTROL_REQUEST = 0x8c
    QUERY_REQUEST = 0x83

    CATEGORY = 0x00

    POWER_FUNCTION = 0x00
    VOLUME_CONTROL_FUNCTION = 0x05
    MUTING_FUNCTION = 0x06

    def __init__(self, serial_port):
        self._conn = serial.Serial(serial_port)

    def _cmd(self, *command):
        if command[0] not in [self.CONTROL_REQUEST,
                              self.QUERY_REQUEST]:
            raise ValueError('invalid command')

        cmd = bytearray(command)
        cmd.append(self._checksum(cmd))

        self._conn.write(cmd)
        self._conn.flushOutput()

        if command[0] == self.QUERY_REQUEST:
            header, answer, return_size = self._conn.read(3)
            response = bytearray(self._conn.read(return_size))
            response_checksum = response.pop()
            expected_checksum = self._checksum(
                [header, answer, return_size, *response])
            data = bytes(response)
        else:
            header, answer, response_checksum = self._conn.read(3)
            expected_checksum = self._checksum([header, answer])
            data = None

        if expected_checksum != response_checksum:
            raise RuntimeError('invalid response checksum')

        if header != 0x70:
            raise RuntimeError('invalid response header')

        if answer != 0x00:
            raise RuntimeError(f'invalid answer ({answer})')

        return data

    @staticmethod
    def _checksum(parts):
        return sum(parts) % 256

    def power_on(self):
        self._cmd(
            self.CONTROL_REQUEST,
            self.CATEGORY,
            self.POWER_FUNCTION,
            0x02,  # length
            0x01,  # power on
        )

    def power_off(self):
        self._cmd(
            self.CONTROL_REQUEST,
            self.CATEGORY,
            self.POWER_FUNCTION,
            0x02,  # length
            0x00,  # power off
        )

    def is_powered_on(self):
        value = self._cmd(
            self.QUERY_REQUEST,
            self.CATEGORY,
            self.POWER_FUNCTION,
            0xff,  # query
            0xff,  # query
        )
        return bool(ord(value))

    def volume_up(self):
        return self._cmd(
            self.CONTROL_REQUEST,
            self.CATEGORY,
            self.VOLUME_CONTROL_FUNCTION,
            0x03,  # length
            0x00,  # relative volume
            0x00,  # volume up
        )

    def volume_down(self):
        return self._cmd(
            self.CONTROL_REQUEST,
            self.CATEGORY,
            self.VOLUME_CONTROL_FUNCTION,
            0x03,  # length
            0x00,  # relative volume
            0x01,  # volume down
        )

    def mute_toggle(self):
        return self._cmd(
            self.CONTROL_REQUEST,
            self.CATEGORY,
            self.MUTING_FUNCTION,
            0x02,  # length
            0x00,  # toggle
        )

    def power_toggle(self):
        if self.is_powered_on():
            self.power_off()
        else:
            self.power_on()


def main():
    tv = Sony(DEV)

    COMMANDS = {
        'on': tv.power_on,
        'off': tv.power_off,
        'power': tv.power_toggle,
        'volume-up': tv.volume_up,
        'volume-down': tv.volume_down,
        'mute': tv.mute_toggle,
    }

    parser = argparse.ArgumentParser()
    parser.add_argument('command', choices=COMMANDS)
    args = parser.parse_args()

    COMMANDS[args.command]()


if __name__ == '__main__':
    main()
