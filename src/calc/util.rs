use super::CalcResult;

// Shortcut to combine two Results and return a new one
// If both results contain an Ok value, the given function will be applied
// Otherwise, the first error found will be returned
pub fn combine(opt1: CalcResult, opt2: CalcResult, func: |f64, f64| -> f64) -> CalcResult {
    let (v1, v2) = (try!(opt1), try!(opt2));
    Ok(func(v1, v2))
}
