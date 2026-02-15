trait Circular {
	fn get_radius(&self) -> f64;
}

struct RoundHole {
	radius: f64,
}

impl RoundHole {
	fn new(radius: f64) -> Self {
		Self { radius }
	}

	fn fits<T: Circular>(&self, peg: T) -> bool {
		self.radius >= peg.get_radius()
	}
}

struct RoundPeg {
	radius: f64
}

impl Circular for RoundPeg {
	fn get_radius(&self) -> f64 {
		self.radius
	}
}

struct SquarePeg {
	width: f64,
}

impl SquarePeg {
	fn new(width: f64) -> Self {
		Self { width }
	}

	fn get_width(&self) -> f64 {
		self.width
	}
}

struct SquarePegAdapter {
	peg: SquarePeg,
}

impl SquarePegAdapter {
	fn new(peg: SquarePeg) -> Self {
		Self { peg }
	}
}

impl Circular for SquarePegAdapter {
	fn get_radius(&self) -> f64 {
		self.peg.get_width() * f64::sqrt(2.0) / 2.0
	}
}

pub fn run() {
	let hole = RoundHole::new(5.0);

	let rpeg = RoundPeg {radius: 5.0};
	println!("Round peg r5 fits round hole r5: {}", hole.fits(rpeg));


	let small_sqpeg = SquarePeg::new(5.0);
	let small_sqpeg_adapter = SquarePegAdapter::new(small_sqpeg);

	let large_sqpeg = SquarePeg::new(10.0);
	let large_sqpeg_adapter = SquarePegAdapter::new(large_sqpeg);

	println!("Square peg w5 fits round hole r5: {}", hole.fits(small_sqpeg_adapter));
	println!("Square peg w10 doesn't fit round hole r5: {}", !hole.fits(large_sqpeg_adapter));
}