use std::env;

fn infer_radix_and_parse_num(input: &str) -> Result<(u64, u32), Box<dyn std::error::Error>> {
    let (prefix, num) = if input.starts_with("0x") {
        ("0x", &input[2..]) 
    } else if input.starts_with("0o") {
        ("0o", &input[2..]) 
    } else if input.starts_with("0b") {
        ("0b", &input[2..]) 
    } else if input.chars().all(|c| c.is_digit(10)) {
        ("", input)
    } else {
        return Err("invalid number format".into());
    };

    let base = match prefix.to_lowercase().as_str() {
        "0x" => 16,
        "0o" => 8,
        "0b" => 2,
        "" => 10,
        _ => unreachable!("check is done in the if else if block")
    };

    let parsed_num = u64::from_str_radix(num, base)?;
    Ok((parsed_num, base))
}

fn format_to_base(number: u64, output_base: u32) -> Result<String, Box<dyn std::error::Error>> {
    match output_base {
        10 => Ok(number.to_string()),       
        16 => Ok(format!("{:X}", number)),  
        2 => Ok(format!("{:b}", number)),  
        8 => Ok(format!("{:o}", number)),  
        _ => Err("unsupported output base.".into()),
    }
}

fn print_usage(args0: &str) {
    eprintln!(
        "usage: {args0} <input_number> <output_base>
supported formats for <input_number>:
  - hexadecimal: 0x1A
  - binary:      0b1101
  - octal:       0o17
  - decimal:     123
supported output bases:
  - 2  - binary
  - 8  - octal
  - 10 - decimal
  - 16 - hexadecimal
examples:
    {args0} 0x1A 10   -> hex to decimal
    {args0} 26 2      -> decimal to binary"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage(&args[0]);
        return;
    }

    let input_number = &args[1];
    let output_base: u32 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("ERR: base should be 2, 8, 10, 16");
        std::process::exit(1);
    });

    match infer_radix_and_parse_num(input_number) {
        Ok((number, _input_base)) => {
            match format_to_base(number, output_base) {
                Ok(result) => println!("{result}"),
                Err(e) => eprintln!("ERR: {e}"),
            }
        }
        Err(e) => eprintln!("ERR: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexadecimal() {
        let result = infer_radix_and_parse_num("0x1A").unwrap();
        assert_eq!(result, (26, 16));
    }

    #[test]
    fn test_binary() {
        let result = infer_radix_and_parse_num("0b101").unwrap();
        assert_eq!(result, (5, 2));
    }

    #[test]
    fn test_octal() {
        let result = infer_radix_and_parse_num("0o17").unwrap();
        assert_eq!(result, (15, 8));
    }

    #[test]
    fn test_decimal() {
        let result = infer_radix_and_parse_num("123").unwrap();
        assert_eq!(result, (123, 10));
    }

    #[test]
    fn test_invalid_prefix() {
        let result = infer_radix_and_parse_num("0z123");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "invalid number format");
    }

    #[test]
    fn test_empty_input() {
        let result = infer_radix_and_parse_num("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_number_for_base() {
        let result = infer_radix_and_parse_num("0b123"); 
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid digit found in string"
        ); // error happens in `u64::from_str_radix` with error message from ParseIntError at https://doc.rust-lang.org/src/core/num/error.rs.html#137
    }
}