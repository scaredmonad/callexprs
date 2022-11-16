#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(box_syntax)]
use regex::Regex;

// `patch` allows replacing of matched symbols.
pub fn match_patch(src: &str, fn_name: &str, patch: bool, callback: Box<dyn Fn(Vec<&str>) -> String>) -> String {
    let patt = format!(r"[^\s?]?({})\((.+)\)", fn_name);
    let re = Regex::new(&patt).unwrap();
    let captured = re.captures(src);
    if let Some(captured) = captured {
        let args = captured.get(2);
        if let Some(args) = args {
            let args: Vec<&str> = args
                .as_str()
                .split(",")
                .into_iter()
                .map(|arg| arg.trim())
                .collect();
            let out = callback(args);
            if patch {
                return String::from(re.replace(src, out));
            } else {
                return String::from(src);
            }
        } else {
            return String::from(src);
        }
    } else {
        return String::from(src);
    }
}

#[cfg(test)]
mod tests {
    use crate::match_patch;
    #[test]
    fn can_match_and_patch() {
        let src = "value: add(10, 20)";
        let out = match_patch(
            src,
            "add",
            true,
            box (|args| {
                let a = args[0].parse::<i32>().unwrap();
                let b = args[1].parse::<i32>().unwrap();
                let out = (a + b).to_string();
                out
            }),
        );
        let out = out.as_str();

        assert_eq!(out, "value: 30");
    }
}
