# sensortheatre

A lightweight drop-in deterministic sensor simulation for embedded targets.

## Usage

The simulation takes a predefined scenario for accelerometer and gyroscope, sensor noise, and a general output data rate. A getter takes system time (in milliseconds, can be uptime or epoch) and lazily evaluates sensor state since previous call.

[Demo](./tests/demo.rs)