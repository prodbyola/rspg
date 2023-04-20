use std::error::Error;

/// Convert a decimal unsigned integer to its base16 representation.
/// Author: Ajayi Olamide
pub fn dec_to_hex(input: usize) -> Result<String, Box<dyn Error>> {
    let alpha = ["a", "b", "c", "d", "e", "f"]; // base 16 alphabets
    let mut result = String::new();
    let mut i = input;
    let c = 16; // constant divider
    
    while i > 0 {
        let q = i / c;
        let r = i % c; // remainder of each division

      	// final char to add to result
        let mut char = r.to_string().chars().next().ok_or("Failed to convert char")?;

        // if the char is beyond base10, get its base16 equivalent
        if r > 9 {
            let a = alpha[r - 10];
            char = a.chars().next().ok_or("Failed to convert char")?;
        }

        // insert the char
        result.insert(0, char);

        i = q;
    }
    
    Ok(result)
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = dec_to_hex(62).expect("unable to convert");
        assert_eq!(result.as_str(), "3e");
    }
}
