fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn longest_str<'a, 'b, 'out>(s1: &'a str, s2: &'b str) -> &'out str
where
    'a: 'out,
    'b: 'out,
{
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn no_need(s: &'static str, s1: &str) -> &'static str {
    s
}
fn main() {
    println!("no need {}", no_need("hh", ""));

    let s1 = "hello world";
    let s2 = "hello";
    println!("longest {}", longest(s1, s2));
    let result: &str;
    {
        let r2 = "world";
        result = longest_str(r2, s1);
        println!("Longest string: {}", result);
    }
}
