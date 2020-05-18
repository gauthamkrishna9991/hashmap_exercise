/*
 *   Copyright (c) 2020 Goutham Krishna K V <gauthamkrishna9991@live.com>
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut inputstr = String::new();
    let mut emp_dept : HashMap<String, String> = HashMap::new();
    let mut departments : Vec<String> = Vec::new();
    loop {
        print!("$ ");
        flush_output("main->loop");
        io::stdin().read_line(&mut inputstr).expect("Error in registering input.");
        if inputstr.trim().starts_with("Add") || inputstr.trim().starts_with("add"){
            let mut name = String::new();
            let mut department = String::new();
            for (i, j) in inputstr.split_whitespace().enumerate() {
                if i == 1 {
                    name.push_str(j);
                } else if i == 3 {
                    department.push_str(j);
                }
            }
            println!("NAME: {}", name);
            println!("DEPT: {}", department);
            let mut is_found = false;
            for ele in &departments {
                if ele.eq(&department) {
                    is_found = true;
                }
            }
            if is_found {
               println!("Department exists.");
            } else {
                println!("Department doesn't exist. Adding to list.");
                departments.push(String::from(department.trim()));
            }
            match emp_dept.insert( String::from(name.trim()), String::from(department.trim())) {
                Some(x) => {
                    println!("X given is {:?}", x);
                    println!("The list is {:?}", emp_dept);
                }
                None => {
                    println!("Null is returened. The list is {:?}", emp_dept);
                }
            }
        } else {
            match inputstr.trim().to_lowercase().as_str() {
                "exit" | "quit" | "x" | "q" => {
                    println!("Exiting.");
                    return;
                }
                "show all" => {
                    println!("EMPLOYEES: {:#?}", emp_dept.keys());

                }
                "show grouped" | "show categorized" => {
                    let mut names : Vec<String> = Vec::new();
                    for department in &departments {
                        println!("For DEPARTMENT {}", department);
                        for name in emp_dept.keys() {
                            match emp_dept.get(name) {
                                Some(x) => {
                                    if x.eq(department) {
                                        names.push(String::from(name));
                                    }
                                }
                                None => { }
                            }
                        }
                        names.sort();
                        println!("NAMES ARE: {:#?}", names);
                        names.clear();
                    }
                }
                _ => { }
            }
        }
        inputstr.clear();
    }
}

fn flush_output(error_msg: &str) {
    io::stdout().flush().expect(&format!("Error in flushing stdout: {}.", error_msg));
}