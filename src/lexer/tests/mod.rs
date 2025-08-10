use super::{Input, lex};

fn test(input: String, expected: String) {
    let result = lex(Input::of(input));

    if expected.starts_with("<error(") {
        let error = &expected["<error(".len()..expected.len() - ")>\n".len()];

        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(&err.name(), error);

        return;
    }

    let tokens = result.unwrap();

    for (idx, token) in tokens.iter().enumerate() {
        let line = expected.lines().nth(idx);
        assert!(line.is_some());

        let line = line.unwrap();
        let actual = format!("{:?}", token);

        assert_eq!(line, actual);
    }
}

#[test]
fn test_1() {
    test(include_str!("1.berd").to_string(), include_str!("1.ans").to_string())
}

#[test]
fn test_2() {
    test(include_str!("2.berd").to_string(), include_str!("2.ans").to_string())
}

#[test]
fn test_3() {
    test(include_str!("3.berd").to_string(), include_str!("3.ans").to_string())
}
