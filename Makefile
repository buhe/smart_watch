flash:
	esptool.py --chip esp32 elf2image target/xtensa-esp32-espidf/release/smart_watch
	esptool.py --chip esp32 -p /dev/cu.usbserial-0001 -b 460800 --before=default_reset --after=hard_reset write_flash --flash_mode dio --flash_freq 40m --flash_size detect 0x10000 target/xtensa-esp32-espidf/release/smart_watch.bin
look:
	espmonitor /dev/cu.usbserial-0001