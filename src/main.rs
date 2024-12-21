use std::env;

fn radixshift(number: &str, base_from: f64, base_to: f64) -> Result<String, String> {
    if base_from < 1.0 || base_to < 1.0 {
        return Err("bases must be greater than or equal to 1".to_string());
    }

    let parts: Vec<&str> = number.split('.').collect();
    if parts.len() > 2 {
        return Err("invalid number".to_string());
    }

    let integer_part = parts[0];
    let fractional_part = if parts.len() == 2 { parts[1] } else { "" };

    let decimal_value = to_decimal(integer_part, fractional_part, base_from)?;
    let converted_value = from_decimal(decimal_value, base_to)?;

    Ok(converted_value)
}

fn to_decimal(integer_part: &str, fractional_part: &str, base_from: f64) -> Result<f64, String> {
    let integer_value = integer_part
        .chars()
        .rev()
        .enumerate()
        .try_fold(0.0, |sum, (i, c)| {
            c.to_digit(36)
                .map(|digit| sum + (digit as f64) * base_from.powi(i as i32))
                .ok_or("Invalid digit in integer part.".to_string())
        })?;

    let fractional_value = fractional_part
        .chars()
        .enumerate()
        .try_fold(0.0, |sum, (i, c)| {
            c.to_digit(36)
                .map(|digit| sum + (digit as f64) * base_from.powi(-(i as i32 + 1)))
                .ok_or("Invalid digit in fractional part.".to_string())
        })?;

    Ok(integer_value + fractional_value)
}

fn from_decimal(decimal_value: f64, base_to: f64) -> Result<String, String> {
    let mut result = String::new();

    let integer_part = decimal_value.floor() as u64;
    let mut integer_result = String::new();
    let mut temp = integer_part;
    while temp > 0 {
        let digit = (temp % base_to as u64) as u32;
        integer_result.insert(0, std::char::from_digit(digit, 36).unwrap());
        temp /= base_to as u64;
    }

    if integer_result.is_empty() {
        integer_result.push('0');
    }
    result.push_str(&integer_result);

    let mut fractional_part = decimal_value - decimal_value.floor();
    if fractional_part > 0.0 {
        result.push('.');
        for _ in 0..10 {
            fractional_part *= base_to;
            let digit = fractional_part.floor() as u32;
            result.push(std::char::from_digit(digit, 36).unwrap());
            fractional_part -= fractional_part.floor();
            if fractional_part.abs() < 1e-10 {
                break;
            }
        }
    }

    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("usage: {{num}} {{base from}} {{base to}}");
        return;
    }

    let base_from: f64 = match args[2].parse() {
        Ok(base) if base > 1.0 => base,
        _ => {
            eprintln!("err: BASE_FROM must be a valid number greater than 1.");
            return;
        }
    };

    let base_to: f64 = match args[3].parse() {
        Ok(base) if base >= 1.0 => base,
        _ => {
            eprintln!("err: BASE_TO must be a valid number greater than or equal to 1.");
            return;
        }
    };

    match radixshift(&args[1], base_from, base_to) {
        Ok(converted) => println!(
            "{} (base {:.2}) -> {} (base {:.2})",
            &args[1], base_from, converted, base_to
        ),
        Err(e) => eprintln!("err: {}", e),
    }

}

#[cfg(test)]
mod tests {
    
}