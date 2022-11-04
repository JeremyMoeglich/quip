#[cfg(test)]
mod tests {
    use crate::interpreter::interpret_code;
    use pretty_assertions::assert_eq;

    #[test]
    fn reference_array() {
        let tests = vec![(
            r#"
fn main() {
        let a = [5, 6, 7];
        let x = [1, 2, a];
        a[1] = 4;
        x[2][1]
}
                "#,
            "4",
        )];

        for (code, expected) in tests {
            let (_, value) = interpret_code(code, vec![]).unwrap();
            assert_eq!(value.get().to_string(), expected);
        }
    }
}
