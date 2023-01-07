extern crate git2;
use git2::Repository;

use std::fs::File;
use std::io::{Write, Read};
use std::process;
use std::env;
use regex::Regex;
use lazy_static::lazy_static;

const TICKET_PATTERN: &str = r"(CLUSTER|APM|BIZ)-[0-9]+";
const COMMIT_MESSAGE: &str = "message";

lazy_static! {
  static ref RE: Regex = Regex::new(TICKET_PATTERN).unwrap();
}

fn main() {
    let commit_filename = env::args().nth(1);

    //More on the contents of commit_source variable can be found here: https://git-scm.com/docs/githooks#_prepare_commit_msg
    let commit_source = env::args().nth(2);
    let branch_name = get_current_branch();
    let ticket_string = get_ticket_from_branch(branch_name);

    match (commit_filename, commit_source, ticket_string) {
        (Some(filename), Some(commit_source), Some(ticket_string)) if commit_source == COMMIT_MESSAGE => {
          let write_result = prepend_string(ticket_string, filename);
            match write_result {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Failed to prepend message. {}", e);
                    process::exit(2);
                }
            };
        }
        (Some(filename), None, Some(ticket_string)) => {
            let write_result = prepend_string(ticket_string, filename);
            match write_result {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Failed to prepend message. {}", e);
                    process::exit(2);
                }
            };
        },
        (_, Some(_), _) => {
            // Do nothing silently. Comes up with template, merge or commit (-c) commits.
        },
        (None, _, _) => {
            eprintln!("Commit file was not provided");
            process::exit(2);
        },
        (_, _, None) => {
            eprintln!("Error encountered extractin ticket from branch");
            process::exit(3);
        }
    }
}

fn get_ticket_from_branch(branch_name: Result<String, git2::Error>) -> Option<String>{

    match branch_name {
      Ok(branch) => {
        let potential_match = RE.find_iter(&branch).nth(0);

        match potential_match {
          Some(ticket_string) => Some(ticket_string.as_str().to_string()),
          None => Some("NO-ISSUE".to_string()),
        }
      }
      Err(_) => None,
    }
}

fn get_current_branch() -> Result<String, git2::Error> {
    let git_repo = Repository::discover("./")?;
    let head = git_repo.head()?;
    let head_name =  head.shorthand();
    match head_name {
        Some(name) => Ok(name.to_string()),
        None => Err(git2::Error::from_str("No branch name found"))
    }
}

fn prepend_string(prefix: String, commit_filename: String) -> Result<(), std::io::Error> {
    // It turns out that prepending a string to a file is not an
    // obvious action. You can only write to the end of a file :(
    //
    // The solution is to read the existing contents, then write a new
    // file starting with the branch name, and then writing the rest
    // of the file.
    // Will not preped if prefix is already contained in message.

    let mut read_commit_file = File::open(commit_filename.clone())?;
    let mut current_message = String::new();
    read_commit_file.read_to_string(&mut current_message)?;

    if !current_message.contains(&prefix) {
      let mut commit_file = File::create(commit_filename)?;
      writeln!(commit_file, "{} ", prefix)?;
      write!(commit_file, "{}", current_message)
    } else {
      Ok(())
    }
}