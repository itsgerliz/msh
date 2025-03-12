/*
MIT License

Copyright (c) 2025 itsgerliz

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::io::Write;
use std::process::exit;
use std::ffi::CString;
use std::ptr::null;
use libc::{c_char, c_int, pid_t, WIFEXITED, WEXITSTATUS, WIFSIGNALED};

fn main() {
	let stdin_handler = std::io::stdin();
	let mut stdout_handler = std::io::stdout();

	let mut last_exit_code: c_int = -1;
	let mut user_input = String::new();
	let mut user_command_c: Vec<CString> = Vec::new();
	let mut user_command_c_raw_ptr: Vec<*const c_char> = Vec::new();

	loop {
		// Clear buffers in case some command was executed before
		user_input.clear();
		user_command_c.clear();
		user_command_c_raw_ptr.clear();

		// Print the shell prompt
		print!("({}) msh $ ", last_exit_code);
		stdout_handler.flush().unwrap();
		
		// Read next command from stdin
		stdin_handler.read_line(&mut user_input).unwrap();

		// Check if the user wants to exit the shell
		if user_input.as_str().trim() == "!exit" {
			println!("msh - Ending session...");
			exit(0);
		}

		// Check no input
		if user_input.as_str().trim().is_empty() {
			last_exit_code = -1;
			continue;
		}

		// Tokenize the user input and push them into the C-style strings vector
		for token in user_input.as_str().trim().split_whitespace() {
			user_command_c.push(CString::new(token).unwrap());
		}

		// Save a raw pointer for each C-style string and NULL terminate the vector
		for token_ptr in &user_command_c {
			user_command_c_raw_ptr.push(token_ptr.as_ptr());
		}
		user_command_c_raw_ptr.push(null());

		let fork_result: pid_t = unsafe { libc::fork() };
		
		// Fork failed
		if fork_result == -1 {
			eprintln!("msh - Fatal error trying to fork!");
			exit(1);
		// Fork OK - child side
		} else if fork_result == 0 {
			// Try to subsitute this routine to the new process
			let execvp_result = unsafe { libc::execvp(user_command_c_raw_ptr[0] as *const c_char, user_command_c_raw_ptr.as_ptr() as *const *const c_char) };
			// If nothing fails code beyond here will not execute
			if execvp_result == -1 {
				exit(250);
			}
		// Fork OK - parent side
		} else {
			// Wait for child end of execution
			let mut exit_status: c_int = 0;
			unsafe { libc::waitpid(fork_result, &mut exit_status, 0) };
			// Check how our child exited
			// Our child exited normally, 250 is reserved for: could not execute
			if WIFEXITED(exit_status) {
				let current_exit_code: c_int = WEXITSTATUS(exit_status);
				if current_exit_code != 250 {
					last_exit_code = current_exit_code;
				} else {
					eprintln!("msh - Command could not be executed!");
					last_exit_code = -1;
				}
			// A signal terminated our child
			} else if WIFSIGNALED(exit_status) {
				eprintln!("msh - A signal was sent to our child and got terminated!");
				last_exit_code = -1;
			}
		}
	}
}
