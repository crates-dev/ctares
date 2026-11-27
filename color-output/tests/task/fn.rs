use super::*;

#[test]
fn test_task() {
    let mut task: Task<'_> = Task::default();
    let mut text: Text<'_> = Text::default();
    text.set_text("1");
    task.add(Text::default()).add(text);
    task.run_all();
    println!("{task:?}");
}
