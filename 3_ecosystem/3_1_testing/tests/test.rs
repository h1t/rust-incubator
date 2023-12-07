#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::cmp::Ordering;
    use std::fmt::Write;

    const BINARY_NAME: &str = "step_3_1";

    #[test]
    fn test_success() {
        check_success_output(5, &[5]);
        check_success_output(5, &[4, 5]);
        check_success_output(5, &[6, 5]);
        check_success_output(5, &[6, 4, 5]);
        check_success_output(5, &[4, 6, 5]);
    }

    #[test]
    fn test_failure_arg() {
        check_failure_arg("");
        check_failure_arg("a");
    }

    #[test]
    fn test_bad_input() {
        let secret = 5;
        let numbers = ["4", "test", "6", "5"];
        let input = numbers.join("\n");
        let numbers = numbers.iter().map(|s| s.parse().ok()).collect::<Vec<_>>();
        let output = generate_output(secret, &numbers);

        let mut cmd = Command::cargo_bin(BINARY_NAME).unwrap();
        let assert = cmd.arg(secret.to_string());

        assert.write_stdin(input);
        assert.assert().success().stdout(output);
    }

    fn check_failure_arg(arg: &str) {
        let mut cmd = Command::cargo_bin(BINARY_NAME).unwrap();
        let assert = cmd.arg(arg);

        assert.assert().failure();
    }

    fn check_success_output(secret: u32, numbers: &[u32]) {
        let mut cmd = Command::cargo_bin(BINARY_NAME).unwrap();
        let assert = cmd.arg(secret.to_string());
        let input = generate_input(numbers);
        let numbers = numbers.iter().copied().map(Some).collect::<Vec<_>>();
        let output = generate_output(secret, &numbers);

        assert.write_stdin(input);
        assert.assert().success().stdout(output);
    }

    fn generate_input(numbers: &[u32]) -> String {
        numbers.iter().fold(String::new(), |mut output, n| {
            let _ = writeln!(output, "{n}");
            output
        })
    }

    fn generate_output(secret: u32, numbers: &[Option<u32>]) -> String {
        let mut res = Vec::with_capacity(numbers.len() * 3 + 1);
        res.push("Guess the number!".to_string());

        for n in numbers {
            res.push("Please input your guess.".to_string());
            if let Some(n) = n {
                res.push(format!("You guessed: {}", n));
                match secret.cmp(n) {
                    Ordering::Greater => res.push("Too small!".to_string()),
                    Ordering::Less => res.push("Too big!".to_string()),
                    Ordering::Equal => res.push("You win!\n".to_string()),
                };
            }
        }
        res.join("\n")
    }
}
