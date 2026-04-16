# pm3-weather
A date/time/weather display for the PROXmobil3, using the [open-meteo.com](open-meteo.com) API for location and weather services.

## Usage
```
pm3-weather <city-name> [state or country name]
```
or, if using the commands for copying below:
```
weather <city-name> [state or country name]
```
## Building
### On Desktop
```
cargo run -- seattle
```
### For ARM (pm3, framebuffer)
```
cargo zigbuild --release --target armv7-unknown-linux-gnueabihf.2.28 --no-default-features --features framebuffer
```
## Deploying
1. Build the ARM binary (see above)
2. Copy to the device over SSH:
```
cat target/armv7-unknown-linux-gnueabihf/release/pm3-weather | ssh root@<DEVICE_IP> 'cat > /init/board/weather && chmod +x /init/board/weather'
```
3. Create the systemd service at `/init/board/weather.service`:
```
[Unit]
Description=Board Weather Display Service
After=network.target

[Service]
Type=simple
ExecStart=/init/board/weather your-city
Restart=always
RestartSec=2
WorkingDirectory=/init/board

[Install]
WantedBy=multi-user.target
```
4. Create the autorun script at `/init/autorun/99-weather.sh` to install the service on boot:
```
#!/bin/sh

mount -o remount,rw /

# Set timezone (adjust as needed)
ln -sf /usr/share/zoneinfo/America/Los_Angeles /etc/localtime

# Disable default PM3 UI
systemctl stop nx
systemctl mask nx
systemctl stop init-abtproxy
systemctl mask init-abtproxy

# Install and start weather service
cp /init/board/weather.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable weather.service

mount -o remount,ro /

/usr/bin/NxExe watchdog 0

systemctl start weather.service
```
* code snippets in this README shamelessly stolen from [haylinmoore](https://github.com/haylinmoore/board)