JLinkExe -Device NRF52840_XXAA -If SWD -AutoConnect 1 -Speed 4000 &
sleep 1
JLinkGDBServerCLExe -if swd -speed 1000 -device nrf52840_XXAA -if swd -speed 1000 -port 2331 &
