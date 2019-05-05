use mig::app::converter::parser::lexical_analyzer;
use mig::app::converter::token::Token::*;

#[test]
fn success_1() {
    let s = ":create members{\n\tname{\n\t:date 1996-07-12 }\n}";
    let seq = lexical_analyzer(s.to_string()).unwrap();
    println!("{:?}", seq);
    let parsed = vec!(NameColon("create".to_string()), Name("members".to_string()), LMidParen, Name("name".to_string()), LMidParen, NameColon("date".to_string()), Ymd(1996, 7, 12), RMidParen, RMidParen);
    for i in 0..seq.get_tokens().len() {
        assert_eq!(seq.get_token(i), parsed[i]);
    }
    assert!(seq.check_syntax());
}

#[test]
fn failed_1() {
    let s = ":create members{\n}";
    let seq = lexical_analyzer(s.to_string()).unwrap();
    println!("{:?}", seq);
    let parsed = vec!(NameColon("create".to_string()), Name("members".to_string()), LMidParen, RMidParen);
    for i in 0..seq.get_tokens().len() {
        assert_eq!(seq.get_token(i), parsed[i]);
    }
    assert!(!seq.check_syntax());
}

#[test]
fn failed_2() {
    let s = ":create{\n}";
    let seq = lexical_analyzer(s.to_string()).unwrap();
    println!("{:?}", seq);
    let parsed = vec!(NameColon("create".to_string()), LMidParen, RMidParen);
    for i in 0..seq.get_tokens().len() {
        assert_eq!(seq.get_token(i), parsed[i]);
    }
    assert!(!seq.check_syntax());
}

#[test]
fn failed_3() {
    let s = "members{\n}";
    let seq = lexical_analyzer(s.to_string()).unwrap();
    println!("{:?}", seq);
    let parsed = vec!(Name("members".to_string()), LMidParen, RMidParen);
    for i in 0..seq.get_tokens().len() {
        assert_eq!(seq.get_token(i), parsed[i]);
    }
    assert!(!seq.check_syntax());
}

#[test]
fn failed_4() {
    let s = ":create members{\nname {}}";
    let seq = lexical_analyzer(s.to_string()).unwrap();
    println!("{:?}", seq);
    let parsed = vec!(NameColon("create".to_string()), Name("members".to_string()), LMidParen, Name("name".to_string()), LMidParen, RMidParen, RMidParen);
    for i in 0..seq.get_tokens().len() {
        assert_eq!(seq.get_token(i), parsed[i]);
    }
    assert!(!seq.check_syntax());
}

