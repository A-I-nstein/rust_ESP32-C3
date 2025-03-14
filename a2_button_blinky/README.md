# button controlled blinking

This program lights up an LED and makes it blink continuously. The LED blinking rates will be cycled through based on a button press (using polling).

# Components
- An LED
- A Push Button

# Wiring
- LED Anode (positive/tilted terminal) is connected to gpio4 on the devkit.
- LED Cathode(negative terminal) is connected to the devkit GND.
- A button pin should be connected to gpio0 of the devkit. 
- A pin of the switch will be connected to the devkit GND.