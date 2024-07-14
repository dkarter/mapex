#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;

    fn load_fixture(name: &str) -> (String, String) {
        let json_path = format!("tests/fixtures/{}.json", name);
        let exs_path = format!("tests/fixtures/{}.exs", name);

        let json_content = fs::read_to_string(json_path).expect("Unable to read JSON fixture");
        let exs_content = fs::read_to_string(exs_path).expect("Unable to read Elixir fixture");

        (json_content, exs_content)
    }

    #[test]
    fn test_mapex_fixtures() {
        let fixtures = ["simple", "nested", "complex"];

        fixtures.iter().for_each(|&fixture| {
            let (json_input, map_output) = load_fixture(fixture);
            let mut cmd = Command::cargo_bin("mapex").unwrap();

            cmd.write_stdin(json_input)
                .assert()
                .success()
                .stdout(predicate::str::contains(map_output));
        });
    }
}
