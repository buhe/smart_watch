## Smart Watch
### Hardware wire

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