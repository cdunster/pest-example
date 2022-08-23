mod parser;

use parser::{parse_as_csv_file, CSVParseError};
use std::io::Read;

pub fn sum_csv_values<T: Read>(input: &mut T) -> Result<f64, CSVParseError> {
    let mut input_string = String::new();
    input
        .read_to_string(&mut input_string)
        .expect("Failed to read from input");

    let mut sum = 0.0;
    let file = parse_as_csv_file(&input_string)?.next().unwrap();
    for record_list in file.into_inner() {
        match record_list.as_rule() {
            parser::Rule::record_list => {
                for record in record_list.into_inner() {
                    match record.as_rule() {
                        parser::Rule::record => {
                            for field in record.into_inner() {
                                sum += field.as_str().parse::<f64>().unwrap();
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            parser::Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::sum_csv_values;

    #[test]
    fn sum_returns_zero_if_one_field_of_zero() {
        let input = "0";
        let res = sum_csv_values(&mut input.as_bytes());

        assert_eq!(Ok(0.0), res)
    }

    #[test]
    fn can_sum_one_field() {
        let input = "3";
        let res = sum_csv_values(&mut input.as_bytes());

        assert_eq!(Ok(3.0), res)
    }

    #[test]
    fn can_sum_multiple_fields() {
        let input = "42, 18,54.125";
        let res = sum_csv_values(&mut input.as_bytes());

        assert_eq!(Ok(114.125), res)
    }

    #[test]
    fn can_sum_multiple_records() {
        let input = "42, 18,54.125\n0.125,6, 1.5";
        let res = sum_csv_values(&mut input.as_bytes());

        assert_eq!(Ok(121.750), res)
    }
}
