#![no_std]
#![no_main]

extern crate alloc;
extern crate user_lib;

use alloc::string::String;
use alloc::vec::Vec;
use core::str;
use user_lib::console::get_line;
use user_lib::*;

#[no_mangle]
fn main() -> i32 {
    let mut cwd = String::new();
    let mut ret_code = 0;
    let mut backgroud_pids = Vec::new();
    getcwd(&mut cwd);
    loop {
        for i in (0..backgroud_pids.len()).rev() {
            let ret = waitpid(backgroud_pids[i], &mut ret_code, true);
            if ret > 0 {
                println!("[{}] Done", backgroud_pids.len());
                backgroud_pids.remove(i);
            }
        }
        print!("root@rusted_os:{}# ", cwd);
        let input = get_line();
        let mut args = input.split_whitespace().collect::<Vec<_>>();
        if !args.is_empty() {
            match args[0] {
                "cd" => cd(&mut cwd, &args),
                "write" => write_test(&args),
                "exit" => break,
                _ => {
                    let pid = fork();
                    if let Some(input_pos) = args.iter().position(|arg| *arg == "<") {
                        if pid == 0 {
                            if input_pos + 1 >= args.len() {
                                println!("syntax error");
                                continue;
                            }
                            let fd = open(args[input_pos + 1], RDONLY);
                            if fd == -1 {
                                println!("'{}': No such file or directory", args[input_pos + 1]);
                                continue;
                            }
                            dup2(fd as usize, 0);
                        }
                        args.drain(input_pos..=input_pos + 1);
                    }
                    if let Some(output_pos) = args.iter().position(|arg| *arg == ">") {
                        if pid == 0 {
                            if output_pos + 1 >= args.len() {
                                println!("syntax error");
                                continue;
                            }
                            let fd = open(args[output_pos + 1], WRONLY | CREATE);
                            if fd == -1 {
                                println!("'{}': No such file or directory", args[output_pos + 1]);
                                continue;
                            }
                            dup2(fd as usize, 1);
                        }
                        args.drain(output_pos..=output_pos + 1);
                    }
                    match args.last().unwrap() {
                        &"&" => {
                            if pid == 0 {
                                exec(&(String::from("/bin/") + args[0]), &args[..args.len() - 1]);
                                println!("{}: command not found", args[0]);
                            } else {
                                backgroud_pids.push(pid as usize);
                                println!("[{}] {}", backgroud_pids.len(), pid);
                            }
                        }
                        _ => {
                            if pid == 0 {
                                exec(&(String::from("/bin/") + args[0]), &args);
                                println!("{}: command not found", args[0]);
                            } else {
                                waitpid(pid as usize, &mut ret_code, false);
                            }
                        }
                    }
                }
            }
        }
    }
    0
}

fn cd(cwd: &mut String, args: &Vec<&str>) {
    let path = match args.len() {
        1 => String::from("/"),
        2 => String::from(args[1]),
        _ => {
            println!("too many arguments");
            return;
        }
    };

    match chdir(&path) {
        0 => {}
        -1 => println!("{}: No such file or directory", args[1]),
        -2 => println!("{}: Not a directory", args[1]),
        _ => panic!(),
    }
    getcwd(cwd);
}

fn write_test(args: &Vec<&str>) {
    if args.len() <= 2 {
        println!("missing operand");
        return;
    }
    let target = args[1];
    let buf_str = args[2].as_bytes();
    let fd = open(target, WRONLY | CREATE);
    if fd == -1 {
        println!("{}: No such file or directory", target);
        return;
    }
    let len = write(fd as usize, buf_str);
    match len {
        -1 => println!("{}: Bad file descriptor", fd),
        _ => println!("ok"),
    }
    close(fd as usize);
}
