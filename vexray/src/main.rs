use vexray::run;

fn main() {
	let now = std::time::Instant::now();

    run("render.ppm", 1920, 1080);

	println!("Time taken: {:.2?}", now.elapsed());
}
