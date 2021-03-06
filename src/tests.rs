use super::IrcMessage;
use std::borrow::Cow::Borrowed;

#[test]
fn command_only() {
    let topic = IrcMessage::parse_slice("FOO").unwrap();

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Borrowed("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command() {
    let topic = IrcMessage::parse_slice(":test FOO").unwrap();

    println!("topic: {:?}", topic.prefix);

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Borrowed("test")));
    assert_eq!(topic.command, Some(Borrowed("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command_trailing_space() {
    let topic = IrcMessage::parse_slice(":test FOO  ").unwrap();

    println!("topic: {:?}", topic.prefix);

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Borrowed("test")));
    assert_eq!(topic.command, Some(Borrowed("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command_middle_trailing_param() {
    let topic = IrcMessage::parse_slice(":test!me@test.ing PRIVMSG #Test :This is a test");
    let topic = topic.unwrap();

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Borrowed("test!me@test.ing")));
    assert_eq!(topic.command, Some(Borrowed("PRIVMSG")));
    assert_eq!(topic.params, vec![Borrowed("#Test"), Borrowed("This is a test")]);
}

#[test]
fn command_middle_trailing_spaces() {
    let topic = IrcMessage::parse_slice("PRIVMSG #foo :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Borrowed("PRIVMSG")));
    assert_eq!(topic.params, vec![Borrowed("#foo"), Borrowed("This is a test")]);
}

#[test]
fn prefix_command_middle_trailing_spaces() {
    let topic = IrcMessage::parse_slice(":test PRIVMSG foo :A string  with spaces   ").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Borrowed("test")));
    assert_eq!(topic.command, Some(Borrowed("PRIVMSG")));
    assert_eq!(topic.params, vec![Borrowed("foo"), Borrowed("A string  with spaces   ")]);
}

#[test]
fn extraneous_spaces() {
    let topic = IrcMessage::parse_slice(":test    PRIVMSG  foo   :bar").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Borrowed("test")));
    assert_eq!(topic.command, Some(Borrowed("PRIVMSG")));
    assert_eq!(topic.params, vec![Borrowed("foo"), Borrowed("bar")]);
}

#[test]
fn multiple_params_prefix() {
    let topic = IrcMessage::parse_slice(":test FOO bar baz quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Borrowed("test")));
    assert_eq!(topic.command, Some(Borrowed("FOO")));
    assert_eq!(topic.params, vec![Borrowed("bar"), Borrowed("baz"), Borrowed("quux")]);
}

#[test]
fn multiple_middle_no_prefix() {
    let topic = IrcMessage::parse_slice("FOO bar baz quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Borrowed("FOO")));
    assert_eq!(topic.params, vec![Borrowed("bar"), Borrowed("baz"), Borrowed("quux")]);
}

#[test]
fn multiple_middle_extra_spaces() {
    let topic = IrcMessage::parse_slice("FOO   bar   baz  quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Borrowed("FOO")));
    assert_eq!(topic.params, vec![Borrowed("bar"), Borrowed("baz"), Borrowed("quux")]);
}

#[test]
fn multiple_middle_trailing_params() {
    let topic = IrcMessage::parse_slice("FOO   bar   baz  quux :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Borrowed("FOO")));
    assert_eq!(topic.params, vec![Borrowed("bar"), Borrowed("baz"), Borrowed("quux"), Borrowed("This is a test")]);
}

#[test]
fn multiple_middle_containing_colons() {
    let topic = IrcMessage::parse_slice(":test PRIVMSG #fo:oo :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Borrowed("test")));
    assert_eq!(topic.command, Some(Borrowed("PRIVMSG")));
    assert_eq!(topic.params,
               vec![Borrowed("#fo:oo"),
                    Borrowed("This is a test")]);
}

#[test]
fn tags_prefix_command_middle_params_trailiing_params() {
    let topic = IrcMessage::parse_slice(
        "@best=super;single :test!me@test.ing FOO bar baz quux :This is a test");
    let topic = topic.unwrap();

    assert_eq!(topic.tags[Borrowed("best")], Borrowed("super"));
    assert_eq!(topic.tags[Borrowed("single")], Borrowed("true"));
    assert_eq!(topic.prefix, Some(Borrowed("test!me@test.ing")));
    assert_eq!(topic.command, Some(Borrowed("FOO")));
    assert_eq!(topic.params,
               vec![Borrowed("bar"),
                    Borrowed("baz"),
                    Borrowed("quux"),
                    Borrowed("This is a test")]);
}

#[cfg(test)]
fn parse_file(filepath: &str) {
    use std::io::fs::File;
    use std::io::BufferedReader;
    let file = File::open(&Path::new(filepath)).unwrap();
    let mut file = BufferedReader::new(file);
    for line in file.lines() {
        let line = line.unwrap();
        assert!(IrcMessage::parse_slice(line.as_slice()).is_ok());
    }
}

#[test]
fn read_intro_logs_1() {
    parse_file("./examples/intro.txt");
}

#[test]
fn read_intro_logs_2() {
    parse_file("./examples/intro2.txt");
}

#[test]
fn read_long_logs_1() {
    parse_file("./examples/long.txt");
}

#[test]
fn read_long_logs_2() {
    parse_file("./examples/long2.txt");
}
