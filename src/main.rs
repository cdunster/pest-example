use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammars/csv.pest"]
struct CSVParser;

fn main() {}

#[cfg(test)]
mod tests {
    use super::{CSVParser, Rule};
    use pest::{consumes_to, fails_with, parses_to};

    #[test]
    fn can_parse_int_as_field() {
        parses_to! {
            parser: CSVParser,
            input: "42",
            rule: Rule::field,
            tokens: [field(0, 2)]
        };
    }

    #[test]
    fn can_parse_float_as_field() {
        parses_to! {
            parser: CSVParser,
            input: "123.312",
            rule: Rule::field,
            tokens: [field(0, 7)]
        };
    }

    #[test]
    fn can_parse_negative_int_as_field() {
        parses_to! {
            parser: CSVParser,
            input: "-35",
            rule: Rule::field,
            tokens: [field(0, 3)]
        };
    }

    #[test]
    fn can_parse_negative_float_as_field() {
        parses_to! {
            parser: CSVParser,
            input: "-78.874",
            rule: Rule::field,
            tokens: [field(0, 7)]
        };
    }

    #[test]
    fn cant_parse_string_as_field() {
        fails_with! {
            parser: CSVParser,
            input: "\"string\"",
            rule: Rule::field,
            positives: [Rule::field],
            negatives: [],
            pos: 0
        };
    }

    #[test]
    fn cant_parse_text_as_field() {
        fails_with! {
            parser: CSVParser,
            input: "text",
            rule: Rule::field,
            positives: [Rule::field],
            negatives: [],
            pos: 0
        };
    }

    #[test]
    fn can_parse_record() {
        parses_to! {
            parser: CSVParser,
            input: "1,2, 3, 123, \t -1,-55",
            rule: Rule::record,
            tokens: [
                record(0, 21, [
                    field(0, 1),
                    field(2, 3),
                    field(5, 6),
                    field(8, 11),
                    field(15, 17),
                    field(18, 21),
                ])
            ]
        };
    }

    #[test]
    fn can_parse_record_with_trailing_comma() {
        parses_to! {
            parser: CSVParser,
            input: "1,",
            rule: Rule::record,
            tokens: [
                record(0, 2, [
                    field(0, 1)
                ])
            ]
        };
    }

    #[test]
    fn cant_parse_empty_record_list() {
        fails_with! {
            parser: CSVParser,
            input: "",
            rule: Rule::record_list,
            positives: [Rule::field],
            negatives: [],
            pos: 0
        };
    }

    #[test]
    fn can_parse_single_record_as_record_list() {
        parses_to! {
            parser: CSVParser,
            input: "42, 55",
            rule: Rule::record_list,
            tokens: [
                record_list(0, 6, [
                    record(0, 6, [
                        field(0, 2),
                        field(4, 6)
                    ])
                ])
            ]
        };
    }

    #[test]
    fn can_parse_single_record_with_trailing_newline_as_record_list() {
        parses_to! {
            parser: CSVParser,
            input: "98, 12.123\n",
            rule: Rule::record_list,
            tokens: [
                record_list(0, 11, [
                    record(0, 10, [
                        field(0, 2),
                        field(4, 10)
                    ])
                ])
            ]
        };
    }

    #[test]
    fn can_parse_multiple_records_as_record_list() {
        parses_to! {
            parser: CSVParser,
            input: "1, 2,3\n52",
            rule: Rule::record_list,
            tokens: [
                record_list(0, 9, [
                    record(0, 6, [
                        field(0, 1),
                        field(3, 4),
                        field(5, 6),
                    ]),
                    record(7, 9, [
                        field(7, 9),
                    ]),
                ])
            ]
        };
    }

    #[test]
    fn can_parse_trailing_comma_as_record_list() {
        parses_to! {
            parser: CSVParser,
            input: "1,\n2,",
            rule: Rule::record_list,
            tokens: [
                record_list(0, 5, [
                    record(0, 2, [
                        field(0, 1)
                    ]),
                    record(3, 5, [
                        field(3, 4)
                    ])
                ])
            ]
        };
    }

    #[test]
    fn can_parse_trailing_newline_as_record_list() {
        parses_to! {
            parser: CSVParser,
            input: "1,\n2\n",
            rule: Rule::record_list,
            tokens: [
                record_list(0, 4, [
                    record(0, 2, [
                        field(0, 1)
                    ]),
                    record(3, 4, [
                        field(3, 4)
                    ])
                ])
            ]
        };
    }

    #[test]
    fn can_parse_empty_file() {
        parses_to! {
            parser: CSVParser,
            input: "",
            rule: Rule::file,
            tokens: [
                file(0, 0, [
                    EOI(0, 0),
                ])
            ]
        };
    }

    #[test]
    fn can_parse_file_with_single_field() {
        parses_to! {
            parser: CSVParser,
            input: "1.42",
            rule: Rule::file,
            tokens: [
                file(0, 4, [
                    record_list(0, 4, [
                        record(0, 4, [
                            field(0, 4)
                        ])
                    ]),
                    EOI(4, 4),
                ])
            ]
        };
    }

    #[test]
    fn can_parse_file_with_single_record() {
        parses_to! {
            parser: CSVParser,
            input: "1, 42, 346.1",
            rule: Rule::file,
            tokens: [
                file(0, 12, [
                    record_list(0, 12, [
                        record(0, 12, [
                            field(0, 1),
                            field(3, 5),
                            field(7, 12)
                        ])
                    ]),
                    EOI(12, 12),
                ])
            ]
        };
    }

    #[test]
    fn can_parse_file_with_multiple_records() {
        parses_to! {
            parser: CSVParser,
            input: "1, 42, 346.1\n78, 09",
            rule: Rule::file,
            tokens: [
                file(0, 19, [
                    record_list(0, 19, [
                        record(0, 12, [
                            field(0, 1),
                            field(3, 5),
                            field(7, 12)
                        ]),
                        record(13, 19, [
                            field(13, 15),
                            field(17, 19),
                        ])
                    ]),
                    EOI(19, 19),
                ])
            ]
        };
    }
}
