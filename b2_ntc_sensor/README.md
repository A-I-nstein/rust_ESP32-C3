# NTC temperature reader

This program measures sure ambient temperature using a 10𝑘Ω (NTC) Thermistor.

# Components
- An NTC Temperature Sensor.

# Wiring
- VCC terminal of the sensor is connected to the devkit 3V3.
- GND terminal of the sensor is connected to the devkit GND.
- The remaining terminal of the sensor is connected gpio4 on the devkit.

# Other Examples
- blink_temp_read: Makes LED blink quicker or slower based on the temperature reading.
- temp_led_array: Makes an LED array light up based on the temperature.