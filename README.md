# sensortheatre

A lightweight drop-in deterministic sensor simulation for embedded targets.

## Usage

Simulation represents a series of timed points for accelerometer and gyroscope. Each sensor is initialized with noise and output data rate, and then added to the simulation. Sensor state is lazily evaluated only when a reading is requested. The only resource that the library requires is system time passed to the measurement getter.

[Demo](./tests/demo.rs)