use super::*;

#[test]
fn test_lower_program() -> anyhow::Result<()> {
    let source = crate::cirparser::source_file("let x::int = 1\nlet y :: bool = false")?;
    let mut lcx = LowerCtxt::default();
    let file = lcx.lower_source_file(&source);
    assert_eq!(file.items.len(), 2);
    Ok(())
}

#[test]
fn test_lower_value_def() -> anyhow::Result<()> {
    let value_def = crate::cirparser::value_def("let x::int = 1")?;
    let mut lcx = LowerCtxt::default();
    lcx.lower_value_def(&value_def);
    Ok(())
}
