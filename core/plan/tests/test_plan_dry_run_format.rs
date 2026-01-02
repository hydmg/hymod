use core_plan::Step;

#[test]
fn test_plan_dry_run_format() {
    let step = Step::Mkdir {
        path: "/tmp/foo".to_string(),
    };
    assert_eq!(format!("{}", step), "MKDIR     /tmp/foo");

    let step = Step::UploadRsync {
        local: "src".to_string(),
        remote: "dest".to_string(),
        opts: "-avz".to_string(),
    };
    assert_eq!(format!("{}", step), "UPLOAD    src -> dest");
}
