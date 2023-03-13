fn main(){

	let t:f64 = 2.0*450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 3.0*750_000.00;
	let d:f64 = 3.0*2_850_000.00;
	let a:f64 = 250_000.00;

	//To calculate average of values
	let average = (t + m + h + d + a)/5.0;
	println!("Average of values{}",average);

	let sum = t + m + h + d + a;
	println!("Sum of values{}",sum);
}