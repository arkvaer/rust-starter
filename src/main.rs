mod flow_control;
mod generic;
mod method;
mod pattern_match_match;
mod traits;

fn main() {
    // flow_control::q1();
    // flow_control::q2();
    // flow_control::q3();
    // flow_control::q4();
    // flow_control::q5();
    // flow_control::q8();
    // flow_control::q9();
    // flow_control::q10();
    // flow_control::q11();
    // pattern_match_match;
    // generic::test_sum();
    // generic::test_impl_generic();
    // generic::test_different_generic();
    // generic::q3();
    // generic::q6();
    // generic::q7();
    println!("========================================================");
    traits::test_summary();
    traits::test_prelude();
}

fn test() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
    let f = format!("{}{}", "Hello ", &s2);
    dbg!(f);
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("{}", byte_escape);
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!"#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
