#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
//#![feature(const_evaluatable_unchecked)]
//#![feature(const_mut_refs)]

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

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
        let form_template = include_str!("templates/form.tera");
        match tera.add_raw_template("form.tera", form_template) {
            Ok(_) => {}
            Err(_) => {
                println!("Errors n dat -- templates");
                ::std::process::exit(1);
            }
        };
        tera
    };
}

fn insert_data_into_context<T, const H: usize, const W: usize>(arr: [T; H * W]) -> Context
where
    T: Copy + std::fmt::Debug,
    [[T; H]; W]: Default + Serialize,
{
    let data = Array2D::<T, H, W>::from(arr);
    let data = data.into_array_of_arrays();

    let mut context = Context::new();

    let columns = 0..W;
    let columns: Vec<usize> = columns.collect();
    let rows = 0..H;
    let rows: Vec<usize> = rows.collect();

    context.insert("table", &data);
    println!("{:?}", data);
    context.insert("columns", &W);
    context.insert("rows", &H);

    context
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        const W: usize = 3;
        const H: usize = 3;
        let a = Array2D::<i32, W, H>::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        println!("{}", a[(0, 0)]);
        println!("{}", a[(0, 1)]);
        println!("{}", a[(0, 2)]);
        println!("{}", a[(1, 0)]);
        println!("{}", a[(1, 1)]);
        println!("{}", a[(1, 2)]);
        println!("{}", a);

        /*
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
        println!("{}", a);
        */

        let context = insert_data_into_context::<i32, W, H>(a.into_array());

        let res = match TEMPLATES.render("form.tera", &context) {
            Ok(t) => t,
            Err(e) => {
                println!("Errors n dat: {}", e);
                ::std::process::exit(1);
            }
        };
        println!("{}", res);

        /*for i in 0..b.len() {
            println!("{:#?}", b[i]);
        }*/
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
