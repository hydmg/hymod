// Error Message Quality Tests (PRD §13)

#[test]
fn test_error_missing_gradle_wrapper() {
    panic!("Error must include: './gradlew not found', suggestion to verify project was created with 'hymod new'.");
}
