use crate::*;

mod once {
    use super::*;

    static TEST: &str = "Hello, ${name}, nice to meet you.";

    #[test]
    fn ok() {
        let mut args = HashMap::new();
        args.insert("name", "Charles");

        let s = Template::new(TEST).unwrap().render(&args).unwrap();

        assert_eq!(s, "Hello, Charles, nice to meet you.");
    }

    #[test]
    fn err_missing() {
        let args:HashMap<&str, &str> = HashMap::new();

        let e = Template::new(TEST).unwrap().render(&args).unwrap_err();

        let expected = Errors {
            inner: vec![Error::Missing("name".to_owned())],
        };

        assert_eq!(e, expected);
    }

    #[test]
    fn err_wrong() {
        let mut args = HashMap::new();
        args.insert("eman", "selrahC");

        let e = Template::new(TEST).unwrap().render(&args).unwrap_err();

        let expected = Errors {
            inner: vec![
                Error::Extra("eman".to_owned()),
                Error::Missing("name".to_owned()),
            ],
        };

        assert_eq!(e, expected);
    }
}

mod beginning {
    use super::*;

    static TEST: &str = "${plural capitalized food} taste good.";

    #[test]
    fn ok() {
        let mut args = HashMap::new();
        args.insert("plural capitalized food", "Apples");

        let s = Template::new(TEST).unwrap().render(&args).unwrap();

        assert_eq!(s, "Apples taste good.");
    }
}

mod only {
    use super::*;

    static TEST: &str = "${why}";

    #[test]
    fn ok() {
        let mut args = HashMap::new();
        args.insert("why", "would you ever do this");

        let s = Template::new(TEST).unwrap().render(&args).unwrap();

        assert_eq!(s, "would you ever do this");
    }
}

mod end {
    use super::*;

    static TEST: &str = "I really love ${something}";

    #[test]
    fn ok() {
        let mut args = HashMap::new();
        args.insert("something", "programming");

        let s = Template::new(TEST).unwrap().render(&args).unwrap();

        assert_eq!(s, "I really love programming");
    }
}

// Dunno why you'd do this either
mod empty {
    use super::*;

    static TEST: &str = "";

    #[test]
    fn ok() {
        let args:HashMap<&str, &str> = HashMap::new();

        let s = Template::new(TEST).unwrap().render(&args).unwrap();

        assert_eq!(s, "");
    }
}

mod two {
    use super::*;

    static TEST: &str = "Hello, ${name}. You remind me of another ${name}.";

    #[test]
    fn ok() {
        let mut args = HashMap::new();
        args.insert("name", "Charles");

        let s = Template::new(TEST).unwrap().render(&args).unwrap();

        assert_eq!(s, "Hello, Charles. You remind me of another Charles.");
    }
}

mod twice {
    use super::*;

    static TEST: &str = "${name}, why are you writing code at ${time} again?";

    #[test]
    fn ok() {
        let mut args = HashMap::new();
        args.insert("name", "Charles");
        args.insert("time", "2 AM");

        let s = Template::new(TEST).unwrap().render(&args).unwrap();

        assert_eq!(s, "Charles, why are you writing code at 2 AM again?");
    }

    #[test]
    fn err_missing_name() {
        let mut args = HashMap::new();
        args.insert("time", "2 AM");

        let e = Template::new(TEST).unwrap().render(&args).unwrap_err();

        let expected = Errors {
            inner: vec![Error::Missing("name".into())],
        };

        assert_eq!(e, expected);
    }

    #[test]
    fn err_missing_time() {
        let mut args = HashMap::new();
        args.insert("name", "Charles");

        let e = Template::new(TEST).unwrap().render(&args).unwrap_err();

        let expected = Errors {
            inner: vec![Error::Missing("time".into())],
        };

        assert_eq!(e, expected);
    }

    #[test]
    fn err_missing_both() {
        let args:HashMap<&str, &str> = HashMap::new();

        let e = Template::new(TEST).unwrap().render(&args).unwrap_err();

        let mut errors =
            vec![Error::Missing("name".into()), Error::Missing("time".into())];

        let expected_1 = Errors {
            inner: errors.clone(),
        };

        // Same thing but the other order, since hashmap iterators are random
        errors.swap(0, 1);
        let expected_2 = Errors {
            inner: errors,
        };

        assert!(e == expected_1 || e == expected_2);

        // These will be in the order they show up in the template
        let expected_msg = r#"missing keys: "name" and "time""#;

        assert_eq!(format!("{}", e), expected_msg);
    }
}

mod missing_keys {
    use super::*;

    static TEST: &str = "${wow}${lots}${of}${keys}";

    #[test]
    fn ok() {
        let args:HashMap<&str, &str> = HashMap::new();

        let e = Template::new(TEST).unwrap().render(&args).unwrap_err();

        assert_eq!(
            format!("{}", e),
            r#"missing keys: "wow", "lots", "of", and "keys""#,
        );
    }
}

// This could really either be interpreted as nothing or an attempt to create a
// replace point. I think the latter is more likely to be the expected behavior.
mod unclosed {
    use super::*;

    static TEST: &str = "Hello, ${name";

    #[test]
    fn ok() {
        let mut args = HashMap::new();
        args.insert("name", "Charles");

        match Template::new(TEST) {
            Err(e) => {
                assert_eq!(e, Error::Unclosed(7));
            },
            Ok(_) => {
                panic!("should have been an error");
            }
        }
    }
}

// This is sort of a weird way to error about mismatched delimiters, but I guess
// it's workable. I'm open to better ideas though, given a working
// implementation to back up those ideas.
mod mismatched {
    use super::*;

    static TEST: &str = "Hello, ${name. You smell like ${smell}.";

    #[test]
    fn ok() {
        let mut args = HashMap::new();
        args.insert("name", "Charles");
        args.insert("smell", "human");

        let actual = Template::new(TEST).unwrap().render(&args).unwrap_err();

        let mut errors = vec![
            Error::Extra("smell".into()),
            Error::Extra("name".into()),
            Error::Missing("name. You smell like ${smell".into()),
        ];

        let expected_1 = Errors {
            inner: errors.clone(),
        };

        // `Extra`s are swapped because hashmap iterators are random
        errors.swap(0, 1);
        let expected_2 = Errors {
            inner: errors,
        };

        assert!(actual == expected_1 || actual == expected_2);
    }
}

// By now, you might be wondering whether there's a way to escape a replace
// point. There isn't, for two reasons:
//
// 1. I was too lazy to figure out a reasonable way to do it
// 2. Just do something like `args.insert("replace", "${replace}")` if you
//    really need it
//
// That second part is mostly a joke, but it'll work. If you need this feature
// and are willing to write the code for it, I'll happily merge it. However, I
// personally don't need this, and so I'm probably not going to be bothered to
// add this functionality myself.
