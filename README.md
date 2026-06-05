# sensortheatre

A drop-in deterministic sensor simulation. Lightweight and target-agnostic.

## Usage

Simulation represents a series of timed points for every sensor or preset of sensors. Each sensor is initialized with output data rate, noise, and IIR smoothing factor. Sensor state is then lazily evaluated at request time. The library only requires time (in milliseconds) to be passed to getters such as `get` and `poll`.

[IMU preset demo](./tests/demo.rs) \ [Simple flight demo](./tests/rocket.rs)

## Adding more sensors

The library is designed to be easily extended to include an arbitrary amount of different sensors without modifying existing code. For the sake of example, assume that a new sensor -- Weirdometer -- needs to be added to the theatre.

### Step 1: add new measurement type to [measm.rs](./src/measm.rs)

```rust
pub(crate) type Weird = (f32, (i16, u16), i8);
```

#### Step 1.1 (recommended): implement [`Noisy`](./src/measm.rs#L12)

Should your sensor produce noisy measurements, implement custom `iir()` and `noise()` for the data type to simulate real output. You can add other methods as well.

#### Step 1.2 (optional): define arithmetic for your type

For your convenience, you can implement [`Vector`](./src/measm.rs#L17) for the data type for later use in sensor logic (see below).

### Step 2: add sensor struct to [sensors.rs](./src/sensors.rs)

```rust
pub struct Weirdometer {
	model: Weird,
	weirdcache: Weird,
	some_necessary_variable: usize,
	some_flag: bool,
}
```

### Step 3: implement `SensorKind<Weird>`

This is the core of your sensor. `step()` function advances sensor measurement over one sampling period `dt` (in seconds). `target` is the next snapshot from user-provided history that the sensor should work towards. `Properties` are the parameters the user initialized your sensor with; they include maximum noise magnitude, smoothing factor, and a random seed for use in pseudo-random generators.

```rust
impl SensorKind<Weird> for Weirdometer {
	fn init(&mut self, value: Weird) {
		// What to do with the initial value?
	}
	fn reading(&self) -> Weird {
		// Where to extract current reading from (and how)?
	}
	fn step(&mut self, target: Weird, p: &mut Properties, dt: f32) {
		// Actual sensor logic
	}
}
```

### Step 4: testing

That's it! You can now add a public alias for your sensor to [lib.rs](./src/lib.rs). Because the sensor is stack-allocated, N is supplied by the user to define the amount of scenario data points.

```rust
pub type Pulsar<const N: usize> = Sensor<N, Weird, Weirdometer>;
```

API, time calculations, and data points management are provided by the library. Optionally, you can combine the new sensor into a preset with another sensor(s) [like this](./src/presets.rs).

```rust
let mut star = Pulsar::<3>::new(40, 0.6, 0.3, (0.0, (0, 0), -10));
star
	.then(600 /* ms */, (92.42, (0, 3), -14))
	.then(1400, (188.21, (1, 4), 2));
	// ...

println!("{:?}", star.get(1000 /* elapsed */).unwrap());
```