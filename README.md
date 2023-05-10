# bbc-wire-wire

## Project Goals

- Get more comfortable with `no-std` Rust on embedded microcontrollers
    - In this case, the BBC micro:bit v2
- Experiment with whether or not the lsm303agr chip on the micro:bit v2 can be used to detect the magnetic field caused
  by electricity in the wire

## Project Status

A basic implementation that detects the change in the strength of the magnetic field is working.
This system is able to easily detect the magnets in my phone and laptop, however, it is not able to detect wires in the
wall at 120 volts. It is able to read at level 1 when holding it directly against the electrical panel, but not at any
other wiring in the walls. So basically I have created a magnet finder, which unsurprisingly, is not very useful, or at
least I haven't figured out how it could be useful.

## Moving forward

I might be able to better tune this by setting a different scaling factor for the different levels, but looking at the
raw data, I think the wiring near me is too well shielded to register. This is also working out to be a very small
implementation, so I think that instead I will try to find something else to use the microcontroller for that will be
more
useful for me.
