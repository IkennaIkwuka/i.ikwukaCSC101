fn main(){

	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	/*Formular to solve Compound Interest
		A = P*[1 - (R/100)]^n
		And for depreciation.*/

	let a = 1.0 - (r/100.0);

	let a = f64::powf(a,n);
	let amount = p * a;

	println!("Depreciation {}",amount );
}