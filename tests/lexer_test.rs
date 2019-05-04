extern crate mig;

use chrono::Date;

use mig::app;
use mig::app::converter::parser::{lexical_analyzer, ParserError};
use mig::app::converter::token::Token;

#[test]
fn name_colon_parser() {
    let s_1 = ":hoge";
    let s_2 = ":create";
    let s_3 = "create";

    let p_1 = lexical_analyzer(s_1.to_string());
    assert_eq!(p_1.unwrap().get_token(0), Token::NameColon("hoge".to_string()));

    let p_2 = lexical_analyzer(s_2.to_string());
    assert_eq!(p_2.unwrap().get_token(0), Token::NameColon("create".to_string()));

    let p_3 = lexical_analyzer(s_3.to_string());
    assert_ne!(p_3.unwrap().get_token(0), Token::NameColon("create".to_string()));
}

#[test]
fn user_string_parser() {
    let s_1 = "member";
    let s_2 = "\"member\"";

    let p_1 = lexical_analyzer(s_1.to_string());
    assert_eq!(p_1.unwrap().get_token(0), Token::Name("member".to_string()));

    let p_2 = lexical_analyzer(s_2.to_string());
    assert_ne!(p_2.unwrap().get_token(0), Token::Name("member".to_string()));
}

#[test]
fn white_spaces() {
    let s_1 = "hoge hoge";
    let s_2 = " hoge hoge";
    let s_3 = "hoge hoge ";
    let s_4 = "hoge\nhoge";
    let s_5 = "\nhoge hoge";
    let s_6 = "hoge\nhoge\n";
    let s_7 = "hoge\nhoge ";
    let s_8 = "hoge  hoge";
    let s_9 = "hoge\n\nhoge";

    let set = [s_1, s_2, s_3, s_4, s_5, s_6, s_7, s_8, s_9];
    for s in set.iter() {
        let p = lexical_analyzer((*s).to_string()).unwrap();
        let tokens = p.get_tokens();
        let l = p.get_tokens().len();
        assert_eq!(l, 2);
        assert_eq!(tokens[0], Token::Name("hoge".to_string()));
        assert_eq!(tokens[1], Token::Name("hoge".to_string()));
    }
}

#[test]
fn table_option_parser() {
    let s_1 = ":timestamps";
    let s_2 = ":unique-index {\n:target group member} ";

    let p_1 = lexical_analyzer(s_1.to_string());
    assert_eq!(p_1.unwrap().get_token(0), Token::NameColon("timestamps".to_string()));

    let p_2 = lexical_analyzer(s_2.to_string()).unwrap();
    assert_eq!(p_2.get_tokens().len(), 6);
    assert_eq!(p_2.get_token(0), Token::NameColon("unique-index".to_string()));
    assert_eq!(p_2.get_token(1), Token::LMidParen);
    assert_eq!(p_2.get_token(2), Token::NameColon("target".to_string()));
    assert_eq!(p_2.get_token(3), Token::Name("group".to_string()));
    assert_eq!(p_2.get_token(4), Token::Name("member".to_string()));
    assert_eq!(p_2.get_token(5), Token::RMidParen);
}

#[test]
fn column_option_parser() {
    let s_1 = "timestamps{}";
    let s_2 = "uniqueindex {\n:target group member \n :hoge \"fuga\"} ";
    let s_3 = "unique-index {\n:target group member} ";

    let p_1 = lexical_analyzer(s_1.to_string()).unwrap();
    assert_eq!(p_1.get_tokens().len(), 3);
    assert_eq!(p_1.get_token(0), Token::Name("timestamps".to_string()));
    assert_eq!(p_1.get_token(1), Token::LMidParen);
    assert_eq!(p_1.get_token(2), Token::RMidParen);

    let p_2 = lexical_analyzer(s_2.to_string()).unwrap();
    assert_eq!(p_2.get_tokens().len(), 8);
    assert_eq!(p_2.get_token(0), Token::Name("uniqueindex".to_string()));
    assert_eq!(p_2.get_token(1), Token::LMidParen);
    assert_eq!(p_2.get_token(2), Token::NameColon("target".to_string()));
    assert_eq!(p_2.get_token(3), Token::Name("group".to_string()));
    assert_eq!(p_2.get_token(4), Token::Name("member".to_string()));
    assert_eq!(p_2.get_token(5), Token::NameColon("hoge".to_string()));
    assert_eq!(p_2.get_token(6), Token::String("fuga".to_string()));
    assert_eq!(p_2.get_token(7), Token::RMidParen);

    let p_3 = lexical_analyzer(s_3.to_string());
    match p_3 {
        Ok(_) => assert_eq!(1, 2), // not pass this test
        Err(e) => assert_eq!(e, ParserError::UnknownToken(1, 7))
    }
}

#[test]
fn integer_parser() {
    let s_1 = "4323";
    let s_2 = "-4323";
    let s_3 = "001";
    let s_4 = "12a";
    let s_5 = "12 a";

    let p_1 = lexical_analyzer(s_1.to_string()).unwrap();
    assert_eq!(p_1.get_token(0), Token::Integer(4323));

    let p_2 = lexical_analyzer(s_2.to_string()).unwrap();
    assert_eq!(p_2.get_token(0), Token::Integer(-4323));

    let p_3 = lexical_analyzer(s_3.to_string()).unwrap();
    assert_eq!(p_3.get_token(0), Token::Integer(1));

    let p_4 = lexical_analyzer(s_4.to_string());
    match p_4 {
        Ok(_) => assert_eq!(1, 2), // not pass this test
        Err(e) => assert_eq!(e, ParserError::NotANumber(1, 2)),
    }

    let p_5 = lexical_analyzer(s_5.to_string()).unwrap();
    let v_5 = p_5.get_tokens();
    assert_eq!(v_5.len(), 2);
    assert_eq!(p_5.get_token(0), Token::Integer(12));
}

#[test]
fn double_parser() {
    let s_1 = "0.01";
    let s_2 = "432.3";
    let s_3 = "-432.3";
    let s_4 = "001.0";
    let s_5 = "12.9a";
    let s_6 = "12.9 a";
    let s_7 = "12.";

    let p_1 = lexical_analyzer(s_1.to_string()).unwrap();
    assert_eq!(p_1.get_token(0), Token::Double(0.01));

    let p_2 = lexical_analyzer(s_2.to_string()).unwrap();
    assert_eq!(p_2.get_token(0), Token::Double(432.3));

    let p_3 = lexical_analyzer(s_3.to_string()).unwrap();
    assert_eq!(p_3.get_token(0), Token::Double(-432.3));

    let p_4 = lexical_analyzer(s_4.to_string()).unwrap();
    assert_eq!(p_4.get_token(0), Token::Double(1.0));

    let p_5 = lexical_analyzer(s_5.to_string());
    match p_5 {
        Ok(_) => assert_ne!(1, 2), // not pass this test
        Err(e) => assert_eq!(e, ParserError::NotANumber(1, 4)),
    }

    let p_6 = lexical_analyzer(s_6.to_string()).unwrap();
    let v_6 = p_6.get_tokens();
    assert_eq!(v_6.len(), 2);
    assert_eq!(p_6.get_token(0), Token::Double(12.9));

    let p_7 = lexical_analyzer(s_7.to_string());
    match p_7 {
        Ok(_) => assert_ne!(1, 2), // not pass this test
        Err(e) => assert_eq!(e, ParserError::NotANumber(1, 3)),
    }
}

#[test]
fn ymd_parser() {
    let s_1 = "1996-07-12";
    let s_2 = "0001-12-14";
    let s_3 = "12-12-12";
    let s_4 = "1212--12-12";
    let s_5 = "1996-07-12_";
    let s_6 = "1996-13-12";
    let s_7 = "1996-07-33";

    let p_1 = lexical_analyzer(s_1.to_string()).unwrap();
    assert_eq!(p_1.get_token(0), Token::Ymd(1996, 7, 12));

    let p_2 = lexical_analyzer(s_2.to_string()).unwrap();
    assert_eq!(p_2.get_token(0), Token::Ymd(1, 12, 14));

    let p_3 = lexical_analyzer(s_3.to_string());
    match p_3 {
        Ok(_) => assert_eq!(1, 2), // not pass test
        Err(e) => assert_eq!(e, ParserError::NotANumber(1, 2))
    }

    let p_4 = lexical_analyzer(s_4.to_string());
    match p_4 {
        Ok(_) => assert_eq!(1, 2), // not pass test
        Err(e) => assert_eq!(e, ParserError::UnknownToken(1, 11))
    }

    let p_5 = lexical_analyzer(s_5.to_string());
    match p_5 {
        Ok(_) => assert_eq!(1, 2), // not pass test
        Err(e) => assert_eq!(e, ParserError::UnknownToken(1, 11))
    }

    let p_6 = lexical_analyzer(s_6.to_string());
    match p_6 {
        Ok(_) => assert_eq!(1, 2), // not pass test
        Err(e) => assert_eq!(e, ParserError::NumberRangeError(1, 10))
    }

    let p_7 = lexical_analyzer(s_7.to_string());
    match p_7 {
        Ok(_) => assert_eq!(1, 2), // not pass test
        Err(e) => assert_eq!(e, ParserError::NumberRangeError(1, 10))
    }
}

#[test]
fn time_parser() {
    let s_1 = "12:12:12";
    let s_2 = "22:22:22";
    let s_3 = "00:00:00";
    let s_4 = "24:00:00";
    let s_5 = "00:60:00";
    let s_6 = "00:00:60";
    let s_7 = "26:27:98";
    let s_8 = "1111:11:11";
    let s_9 = "00:0s:00";

    let p_1 = lexical_analyzer(s_1.to_string()).unwrap();
    assert_eq!(p_1.get_token(0), Token::Time(12, 12, 12));

    let p_2 = lexical_analyzer(s_2.to_string()).unwrap();
    assert_eq!(p_2.get_token(0), Token::Time(22, 22, 22));

    let p_3 = lexical_analyzer(s_3.to_string()).unwrap();
    assert_eq!(p_3.get_token(0), Token::Time(0, 0, 0));

    let p_4 = lexical_analyzer(s_4.to_string());
    match p_4 {
        Ok(_) => assert_eq!(1, 2), // not pass test
        Err(e) => assert_eq!(e, ParserError::NumberRangeError(1, 8))
    }

    let p_5 = lexical_analyzer(s_5.to_string());
    match p_5 {
        Ok(_) => assert_eq!(1, 2), // not pass test
        Err(e) => assert_eq!(e, ParserError::NumberRangeError(1, 8))
    }

    let p_6 = lexical_analyzer(s_6.to_string());
    match p_6 {
        Ok(_) => assert_eq!(1, 2), // not pass test
        Err(e) => assert_eq!(e, ParserError::NumberRangeError(1, 8))
    }

    let p_7 = lexical_analyzer(s_7.to_string());
    match p_7 {
        Ok(_) => assert_eq!(1, 2), // not pass test
        Err(e) => assert_eq!(e, ParserError::NumberRangeError(1, 8))
    }

    let p_8 = lexical_analyzer(s_8.to_string());
    match p_8 {
        Ok(_) => assert_eq!(1, 2), // not pass test
        Err(e) => assert_eq!(e, ParserError::NotANumber(1, 4))
    }

    let p_8 = lexical_analyzer(s_8.to_string());
    match p_8 {
        Ok(_) => assert_eq!(1, 2), // not pass test
        Err(e) => assert_eq!(e, ParserError::NotANumber(1, 4))
    }
}

#[test]
fn datetime_parser() {
    let s_1 = "1996-07-12_23:59:59";
    let s_2 = "1996--07-12_23:59:59";
    let s_3 = "1996-07-12_23:59:599";
    let s_4 = "1996-07-12_";
    let s_5 = "1996-07-12_1";

    let p_1 = lexical_analyzer(s_1.to_string()).unwrap();
    assert_eq!(p_1.get_token(0), Token::DateTime(1996, 7, 12, 23, 59, 59));

    let p_2 = lexical_analyzer(s_2.to_string());
    match p_2 {
        Ok(_) => assert_eq!(1, 2),
        Err(e) => assert_eq!(e, ParserError::UnknownToken(1, 11))
    }

    let p_3 = lexical_analyzer(s_3.to_string());
    match p_3 {
        Ok(_) => assert_eq!(1, 2),
        Err(e) => assert_eq!(e, ParserError::UnknownToken(1, 20))
    }

    let p_4 = lexical_analyzer(s_4.to_string());
    match p_4 {
        Ok(_) => assert_eq!(1, 2),
        Err(e) => assert_eq!(e, ParserError::UnknownToken(1, 11))
    }

    let p_5 = lexical_analyzer(s_5.to_string());
    match p_5 {
        Ok(_) => assert_eq!(1, 2),
        Err(e) => assert_eq!(e, ParserError::UnknownToken(1, 12))
    }
}
