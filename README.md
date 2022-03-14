## Smart Watch
### Hardware wire

### Compile
1. Install rust.

2. Add Xtensa architecture support.

```
export SSID="You wifi name"
export PASS="You wifi password"
cargo build --release
```
### Flash
#### Install toolchain
```
pip install esptool
```
#### Use it
```
esptool.py --chip esp32 elf2image target/xtensa-esp32-espidf/release/smart_watch
esptool.py --chip esp32 -p /dev/cu.usbserial-0001 -b 460800 --before=default_reset --after=hard_reset write_flash --flash_mode dio --flash_freq 40m --flash_size detect 0x10000 target/xtensa-esp32-espidf/release/smart_watch.bin
```
-p {set your dev port}

### View

### Design
- https://www.figma.com/file/ibhnqQRiJZMwjifO27FRu5/smart-watch?node-id=0%3A1

### Feature

- [x] time
- [x] weather
- [x] cat play
- [x] distance

### Progress

- [x] time
    - [x] ntp
    - [x] format date and select timezone
    - [x] ntp + cpu frev
- [x] weather
- [x] cat play
    - [x] switch it
- [x] distance
    - [x] switch it
- [ ] soldering chip
- [ ] select power
- [ ] battery
- [ ] flash
- [ ] e-paper as display screen
    - [ ] render time
    - [ ] render weather
    - [ ] render distance
- [ ] network via phone bt
- [x] supabase
- [ ] app based supabase

### onoff

```bash
curl 'https://jcxivsbsjuqmeafnwuwk.supabase.co/rest/v1/onoff?id=eq.1&select=*' \
-H "apikey: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImpjeGl2c2JzanVxbWVhZm53dXdrIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NDcwNjYwOTEsImV4cCI6MTk2MjY0MjA5MX0.YP7o3MKM7sxsNioyuVuVqTIgdgJbKz638njLOnT9DRA" \
-H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImpjeGl2c2JzanVxbWVhZm53dXdrIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NDcwNjYwOTEsImV4cCI6MTk2MjY0MjA5MX0.YP7o3MKM7sxsNioyuVuVqTIgdgJbKz638njLOnT9DRA" \
-H "Range: 0-9"
```