use std::error::Error;

pub fn dec_to_hex(input: usize) -> Result<String, Box<dyn Error>> {
    let alpha = ["a", "b", "c", "d", "e", "f"]; // base 16 alphabets
    let mut result = String::new();
    let mut i = input;
    let c = 16; // constant divider
    
    while i > 0 {
        let q = i / c;
        let r = i % c; // remainder of each division

        let mut char = r.to_string().chars().next().ok_or("Failed to convert char")?;

        if r > 9 {
            let a = alpha[r - 10];
            char = a.chars().next().ok_or("Failed to convert char")?;
        }

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
        println!("result {}", dec_to_hex(188).expect("unable to convert"));
        assert_eq!(result.as_str(), "3e");
    }
}
