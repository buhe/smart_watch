## Up
### Hardware wire
Left is st7789,right is esp32.
- gnd-gnd
- vcc-3v3
- scl-gpio18
- sda-gpio19
- reset-gpio23
- ao-gpio16
- cs-gpio5
- bl-gpio4
### Compile
1. Install rust.

2. Add Xtensa architecture support.

3. Get your mid.
brower visit : http://api.bilibili.com/x/space/myinfo get mid feild.
```
export SSID="You wifi name"
export PASS="You wifi password"
export MID="You bilibili mid"
cargo build --release
```
### Flash
#### Install toolchain
```
pip install esptool
```
#### Use it
```
esptool.py --chip esp32 elf2image target/xtensa-esp32-espidf/release/up
esptool.py --chip esp32 -p /dev/cu.usbserial-0001 -b 460800 --before=default_reset --after=hard_reset write_flash --flash_mode dio --flash_freq 40m --flash_size detect 0x10000 target/xtensa-esp32-espidf/release/up.bin
```
-p {set your dev port}
### View
![E5B9D164-DBD5-4673-BD3D-71A6EEBC8EDC_1_105_c](https://tva1.sinaimg.cn/large/e6c9d24egy1gzizfgd8bvj20wu0ih0wa.jpg)