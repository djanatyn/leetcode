//! Create directory containing Leetcode problem description.

use duct::cmd;
use serde::Deserialize;
use std::env;

// ```
// ❯ curl https://leetcode.com/problems/contains-duplicate -vL -o/dev/null 2>&1 \
//   | rg 'set-cookie: csrftoken=([^ ]+);' -or '$1'
// AdasXUlUAP5wuVhq9qcTs6LYBZsk4IxKzdThaqsHcQIUR8JnReF1f4jTSJ7k8MAL
//
// ❯ curl 'https://leetcode.com/graphql' -X POST \
//     -H 'content-type: application/json' \
//     -H 'Cookie: csrftoken=AdasXUlUAP5wuVhq9qcTs6LYBZsk4IxKzdThaqsHcQIUR8JnReF1f4jTSJ7k8MAL' \
//     -H 'x-csrftoken: AdasXUlUAP5wuVhq9qcTs6LYBZsk4IxKzdThaqsHcQIUR8JnReF1f4jTSJ7k8MAL' \
//     -H 'Referer: https://leetcode.com/problems/contains-duplicate/' \
//     --data-raw '{"operationName":"questionData","variables":{"titleSlug":"contains-duplicate"},"query":"query questionData($titleSlug: String!) { question(titleSlug: $titleSlug) { content } }"}' 2>/dev/null \
//      | jq '.data.question.content' -r \
//      | pandoc -f html -t gfm
// ```
// > Given an integer array `nums`, return `true` if any value appears **at
// > ...

const API_URL: &str = "https://leetcode.com/graphql";
const PROBLEMS_URL: &str = "https://leetcode.com/problems";

#[derive(Deserialize, Debug)]
struct ProblemData {
    data: ProblemQuestion,
}

#[derive(Deserialize, Debug)]
struct ProblemQuestion {
    question: ProblemContent,
}

#[derive(Deserialize, Debug)]
struct ProblemContent {
    content: String,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let graphql = include_str!("query.graphql");

    for slug in dbg!(args) {
        let problem_url = format!("{PROBLEMS_URL}/{slug}");

        // get csrf token
        let agent = ureq::agent();
        agent
            .get(&problem_url)
            .call()
            .expect("failed initial request");

        // request problem data
        let api_request = agent
            .post(API_URL)
            .send_json(ureq::json!({
                "operationName": "questionData",
                "variables": {"titleSlug": slug},
                "query": graphql
            }))
            .expect("failed api request");

        let problem =
            dbg!(api_request.into_json::<ProblemData>()).expect("failed to parse response");

        // create problem directory
        std::fs::create_dir(&slug).expect("could not create directory!");

        // write README.md
        let readme_path = format!("./{slug}/README.md");
        cmd!("pandoc", "-f", "html", "-t", "gfm")
            .stdin_bytes(problem.data.question.content)
            .stdout_path(readme_path)
            .read()
            .expect("failed to convert to markdown");
    }
}
