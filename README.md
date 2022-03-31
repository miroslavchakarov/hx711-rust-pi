# hx711-rust-pi
Running a loadcell with ADC hx711 on Rust


Running via an SPI library


update: added the tara /calibration function

To be created:
- convert raw readings to grams/kg.
- raise an event when readings change in order to calculate the difference before vs. after
- show client's data on the 2004 lcd display (See other repository)
