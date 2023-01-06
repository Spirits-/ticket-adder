extern crate git2;
use git2::Repository;

use std::fs::File;
use std::io::{Write, Read};
use std::process;
use std::env;
use regex::Regex;

fn main() {
    let commit_filename = env::args().nth(1);

    // the commit source will will be filled with labels like 'merge'
    // to say how you got to this point.
    let commit_source = env::args().nth(2);
    let branch_name = get_current_branch();
    let ticket_string = get_ticket_from_branch(branch_name);

    match (commit_filename, commit_source, ticket_string) {
        (Some(filename), _, Some(ticket_string)) => {
          println!("{filename}");
          println!("{ticket_string}");

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
            // do nothing silently. This comes up on merge commits,
            // amendment commits, if a message was specified on the
            // cli.
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
    let re = Regex::new(r"(CLUSTER|APM|BIZ)-[0-9]+");

    match(branch_name, re) {
      (Ok(branch), Ok(re)) => {
        let potential_match = re.find_iter(&branch).nth(0);

        match potential_match {
          Some(ticket_string) => Some(ticket_string.as_str().to_string()),
          None => Some("NO-ISSUE".to_string()),
        }
      }
      (Err(_), _) => None,
      (Ok(_), Err(_)) => None,
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