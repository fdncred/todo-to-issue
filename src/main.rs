mod cli;
mod command;
mod issue;
mod parse;
mod request;

use request::Request;

fn main() {
    let args = match cli::init() {
        Some(args) => args,
        None => return,
    };

    let request = Request::new(args.get_token());
    let files = command::get_tracked_files();
    let issues = request.get_issues().expect("Failed to get issues");

    let pattern = args.get_pattern();
    let file_to_issues = parse::populate_map(&files, &issues, pattern);

    if file_to_issues.len() > 0 && !args.is_dry_run() {
        cli::output_and_send_issues(&request, &file_to_issues);
    }
}
