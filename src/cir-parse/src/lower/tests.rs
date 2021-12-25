use super::*;

#[test]
fn test_lower_program() -> anyhow::Result<()> {
    let source = crate::cirparser::source_file("let x: Int = 1\nlet y: Bool = false")?;
    let mut lcx = LowerCtxt::default();
    let file = lcx.lower_source_file(&source);
    assert_eq!(file.items.len(), 2);
    Ok(())
}

#[test]
fn test_lower_value_def() -> anyhow::Result<()> {
    macro_rules! lower {
        ($s:expr) => {{
            let value_def = crate::cirparser::value_def($s)?;
            let mut lcx = LowerCtxt::default();
            lcx.lower_value_def(&value_def);
        }};
    }

    lower!("let x: Int = 1");
    // lower!("let f :: (a -> b) -> a -> b = \\f -> \\x -> f x");
    Ok(())
}
