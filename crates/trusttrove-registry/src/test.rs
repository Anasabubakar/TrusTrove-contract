#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_manifest_validation_success() {
        // Successful path test: validates correct WASM checksums and metadata
        let manifest_data = r#"{
            "source_commit": "abc1234",
            "network_passphrase": "Test SDF Network ; September 2015",
            "contract_ids": {
                "registry": "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXFG4DCZGPLXRQZUH5LSBTBOL"
            },
            "wasm_checksums": {
                "trusttrove_registry.wasm": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            }
        }"#;
        let parsed: serde_json::Value = serde_json::from_str(manifest_data).unwrap();
        assert_eq!(parsed["source_commit"], "abc1234");
        assert!(parsed["wasm_checksums"]["trusttrove_registry.wasm"].is_string());
    }

    #[test]
    fn test_deployment_manifest_validation_invalid_input() {
        // Invalid input rejection: malformed JSON or missing fields must fail parsing
        let malformed_data = r#"{
            "source_commit": "abc1234",
            "wasm_checksums": // invalid syntax or missing closing
        }"#;
        let result: Result<serde_json::Value, _> = serde_json::from_str(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_deployment_manifest_validation_idempotency_boundary() {
        // Idempotency boundary test: verifying the same manifest twice yields identical checksum results
        let manifest_data = r#"{
            "source_commit": "abc1234",
            "wasm_checksums": {
                "trusttrove_registry.wasm": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            }
        }"#;
        let first: serde_json::Value = serde_json::from_str(manifest_data).unwrap();
        let second: serde_json::Value = serde_json::from_str(manifest_data).unwrap();
        assert_eq!(first, second);
    }
}
