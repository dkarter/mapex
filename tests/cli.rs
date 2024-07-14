#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_mapex_simple() {
        let mut cmd = Command::cargo_bin("mapex").unwrap();
        let json_input = r#"{"key": "value"}"#;
        let map_output = r#"%{"key" => "value"}"#;

        cmd.write_stdin(json_input)
            .assert()
            .success()
            .stdout(predicate::str::contains(map_output));
    }

    #[test]
    fn test_mapex_full() {
        let mut cmd = Command::cargo_bin("mapex").unwrap();
        let json_input = r#"{
          "a_str": "hiii",
          "hello": ["world", "worlds", "mars", 1, 2,3],
          "a_num": 6,
          "a_bool": false,
          "a_null": null
        }"#;

        let map_output = r#"%{"a_bool" => false, "a_null" => nil, "a_num" => 6, "a_str" => "hiii", "hello" => ["world", "worlds", "mars", 1, 2, 3]}"#;

        cmd.write_stdin(json_input)
            .assert()
            .success()
            .stdout(predicate::str::contains(map_output));
    }
}
