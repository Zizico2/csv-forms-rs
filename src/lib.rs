#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tera::Tera;

mod array2d;
mod vec2d;
use array2d::Array2D;

#[derive(Serialize, Deserialize, Debug)]
struct Segment {
    pub question: String,
    pub answer: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Reply {
    pub segments: Vec<Segment>,
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        let form_template = include_str!("templates/form.html");
        match tera.add_raw_template("form.html", form_template) {
            Ok(_) => {}
            Err(_) => {
                println!("Errors n dat -- templates");
                ::std::process::exit(1);
            }
        };
        tera
    };
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;

    fn test_disp<T: Display, const H: usize, const W: usize>(a: &[T; W * H]) {
        let mut out = String::new();
        for x in 0..W {
            for y in 0..H {
                let index = x + y * W;
                let s = format!("| {} ", a[index]);
                out.push_str(&s);
            }
            out.push_str("|\n");
        }
        print!("{}", out);
    }
    #[test]
    fn it_works() {
        let mut a = Array2D::<_, 3, 2>::new([1_i32, 2, 3, 4, 5, 6]);
        println!("{}", a[(0, 0)]);
        println!("{}", a[(0, 1)]);
        println!("{}", a[(0, 2)]);
        println!("{}", a[(1, 0)]);
        println!("{}", a[(1, 1)]);
        println!("{}", a[(1, 2)]);

        a[(0, 0)] = -1;
        a[(0, 1)] = -2;
        a[(0, 2)] = -3;
        a[(1, 0)] = -4;
        a[(1, 1)] = -5;
        a[(1, 2)] = -6;

        println!("{}", a[(0, 0)]);
        println!("{}", a[(0, 1)]);
        println!("{}", a[(0, 2)]);
        println!("{}", a[(1, 0)]);
        println!("{}", a[(1, 1)]);
        println!("{}", a[(1, 2)]);

        let c = a.get_row_majored();

        // TODO the compiler should be able to infer the i32 in the future - compiler_bug
        test_disp::<i32, 3, 2>(c);

        /*
        use tera::Context;
        // Using the tera Context struct
        let mut replies = vec![];
        let segment = Segment {
            question: "Nome".into(),
            answer: "Bernardo".into(),
        };
        let segment2 = Segment {
            question: "Morada".into(),
            answer: "RVC".into(),
        };

        let segment3 = Segment {
            question: "Nome".into(),
            answer: "Pedro".into(),
        };
        let segment4 = Segment {
            question: "Morada".into(),
            answer: "RRF".into(),
        };

        replies.push(Reply {
            segments: vec![segment, segment2],
        });
        replies.push(Reply {
            segments: vec![segment3, segment4],
        });
        let mut context = Context::new();
        context.insert("replies", &replies);
        let res = match TEMPLATES.render("form.html", &context) {
            Ok(t) => t,
            Err(e) => {
                println!("Errors n dat: {}", e);
                ::std::process::exit(1);
            }
        };
        println!("{}", res);
        */
    }
}
