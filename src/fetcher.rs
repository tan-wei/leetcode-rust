extern crate reqwest;
extern crate serde_json;

use serde_json::Value;
use std::fmt::{Display, Error, Formatter};

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";
const GRAPHQL_URL: &str = "https://leetcode.com/graphql";
const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
        metaData
    }
}"#;
const QUESTION_QUERY_OPERATION: &str = "questionData";

pub fn get_problem(frontend_question_id: u32) -> Option<Problem> {
    let problems = get_problems().unwrap();
    for problem in problems.stat_status_pairs.iter() {
        if problem.stat.frontend_question_id == frontend_question_id {
            if problem.paid_only {
                return None;
            }

            let client = reqwest::blocking::Client::new();
            let resp: RawProblem = client
                .post(GRAPHQL_URL)
                .json(&Query::question_query(
                    problem.stat.question_title_slug.as_ref().unwrap(),
                ))
                .send()
                .unwrap()
                .json()
                .unwrap();
            return Some(Problem {
                title: problem.stat.question_title.clone().unwrap(),
                title_slug: problem.stat.question_title_slug.clone().unwrap(),
                code_definition: serde_json::from_str(&resp.data.question.code_definition).unwrap(),
                content: resp.data.question.content,
                sample_test_case: resp.data.question.sample_test_case,
                difficulty: problem.difficulty.to_string(),
                question_id: problem.stat.frontend_question_id,
                return_type: {
                    let v: Value = serde_json::from_str(&resp.data.question.meta_data).unwrap();
                    v["return"]["type"].to_string().replace("\"", "")
                },
            });
        }
    }
    None
}

pub async fn get_problem_async(problem_stat: StatWithStatus) -> Option<Problem> {
    if problem_stat.paid_only {
        println!(
            "Problem {} is paid-only",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }
    let resp = surf::post(GRAPHQL_URL).body_json(&Query::question_query(
        problem_stat.stat.question_title_slug.as_ref().unwrap(),
    ));
    if resp.is_err() {
        println!(
            "Problem {} not initialized due to some error",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }
    let resp = resp.unwrap().recv_json().await;
    if resp.is_err() {
        println!(
            "Problem {} not initialized due to some error",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }
    let resp: RawProblem = resp.unwrap();
    return Some(Problem {
        title: problem_stat.stat.question_title.clone().unwrap(),
        title_slug: problem_stat.stat.question_title_slug.clone().unwrap(),
        code_definition: serde_json::from_str(&resp.data.question.code_definition).unwrap(),
        content: resp.data.question.content,
        sample_test_case: resp.data.question.sample_test_case,
        difficulty: problem_stat.difficulty.to_string(),
        question_id: problem_stat.stat.frontend_question_id,
        return_type: {
            let v: Value = serde_json::from_str(&resp.data.question.meta_data).unwrap();
            v["return"]["type"].to_string().replace("\"", "")
        },
    });
}

pub fn get_problems() -> Option<Problems> {
    let headers = {
        let mut h = reqwest::header::HeaderMap::new();
        h.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64; rv:123.0) Gecko/20100101 Firefox/123.0",
            ),
        );
        h.insert(
            "Referer",
            reqwest::header::HeaderValue::from_static("www.leetcode.com"),
        );
        h.insert(
            "Origin",
            reqwest::header::HeaderValue::from_static("www.leetcode.com"),
        );
        h.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        h.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        h.insert(
            "Host",
            reqwest::header::HeaderValue::from_static("www.leetcode.com"),
        );
        h.insert(
            "X-Requested-With",
            reqwest::header::HeaderValue::from_static("XMLHttpRequest"),
        );

        h.insert(
            "Cookie",
            reqwest::header::HeaderValue::from_static("csrftoken=CmUTtU5KRa6cnCggj1H5EvhJvJ3tPytP4G82GuH5Q6t7CFFCZkD3FBbQLyE7KCe5; _ga_CDRWKZTDEX=GS1.1.1710580144.71.1.1710580189.15.0.0; _ga=GA1.2.1781999148.1688193072; gr_user_id=88d74b3a-9d56-40ac-aa14-499bc1aa41ae; __stripe_mid=3c5f9c92-a334-4c8f-af94-f0bfdef4406d10a818; cf_clearance=dnt.AMZ.ey0VqTrqSI1PcyxNTEEgyuBxMnEgNXwdKg8-1710580164-1.0.1.1-ESw5ZmKLb5G1GAquNkjZ9uQZX0VXDk3wzqYSRG6X4IB4pwKKPs8uUME9DOB8O_pQKftpAVv8MNr1UIcVIbsK3g; __gads=ID=3361b3060affd9e4-2233a6c8d3e70007:T=1699180244:RT=1710580185:S=ALNI_MYf4cBTAmgXxlZcF6XRWPq5c_6Y9w; __gpi=UID=00000a4002b8045b:T=1699180244:RT=1710580185:S=ALNI_Ma77MyW1F0CFb2ShFHhCMjWdbg-FA; __eoi=ID=820ea8f483341efc:T=1708869842:RT=1710580185:S=AA-AfjYICQ6p1w7uMqbWbzsohL9i; INGRESSCOOKIE=891032c5aa2f9fbd7d738f9d9e0fb367|8e0876c7c1464cc0ac96bc2edceabd27; messages=.eJyLjlaKj88qzs-Lz00tLk5MT1XSMdAxMtVRCi5NTgaKpJXm5FQqFGem56WmKGTmKSQWK4Rn5pWkFhWlZhan6inF6lBqQCwAycQt5Q:1rlQ2o:XBVIUKIIB5Qf8zuE7xQnOzI4sSATQtIXbRavklBO7Uo; __cf_bm=WyH_WYeQsNxke9oPYFf3fV0Dd.dsDZjY_0kpU.mCYmQ-1710580060-1.0.1.1-wy5AlTIGVY153Zjay9yh0dbD2vdxbVPF_eZ9QNhjZYf.7NBY5.SVHQgwluaVzUaKEu12bBnujttv8l3h7iTlmQ; _gid=GA1.2.1877820459.1710580151; _dd_s=rum=0&expire=1710581747104; 87b5a3c3f1a55520_gr_session_id=5cdbc5e1-7d0b-4728-9b9b-2a8da15a29da; 87b5a3c3f1a55520_gr_session_id_sent_vst=5cdbc5e1-7d0b-4728-9b9b-2a8da15a29da; LEETCODE_SESSION=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJfYXV0aF91c2VyX2lkIjoiMTE1NDc4NSIsIl9hdXRoX3VzZXJfYmFja2VuZCI6ImRqYW5nby5jb250cmliLmF1dGguYmFja2VuZHMuTW9kZWxCYWNrZW5kIiwiX2F1dGhfdXNlcl9oYXNoIjoiNjQwZTZlOGYxY2MxYzIwNThmYTQzZmRiYzg5YTE4YjQzYmMxOTQ2NDg4MjZmNTVhOGE1Mjg0MDlmN2E3N2U3MiIsImlkIjoxMTU0Nzg1LCJlbWFpbCI6InRhbndlaTA0MDRAMTYzLmNvbSIsInVzZXJuYW1lIjoiV2ludGVycmVpc2UiLCJ1c2VyX3NsdWciOiJXaW50ZXJyZWlzZSIsImF2YXRhciI6Imh0dHBzOi8vYXNzZXRzLmxlZXRjb2RlLmNvbS91c2Vycy9kZWZhdWx0X2F2YXRhci5qcGciLCJyZWZyZXNoZWRfYXQiOjE3MTA1ODAxNzQsImlwIjoiNTAuMTE0LjU5LjciLCJpZGVudGl0eSI6Ijc0YjFmYjhkNjAzNjFiNDZjMDJhODVkNWJiMzYyM2YzIiwic2Vzc2lvbl9pZCI6NTc3MDI3NDUsIl9zZXNzaW9uX2V4cGlyeSI6MTIwOTYwMH0.BpXtHQRLNVglW2kKCNmfn0wjRJnZXSlZBifgSN5MYrU; 87b5a3c3f1a55520_gr_last_sent_sid_with_cs1=5cdbc5e1-7d0b-4728-9b9b-2a8da15a29da; 87b5a3c3f1a55520_gr_last_sent_cs1=Winterreise; 87b5a3c3f1a55520_gr_cs1=Winterreise; FCNEC=%5B%5B%22AKsRol9Jk56X9bgfDJ14RByWNi3b99Ls0jClosKGrQEGlQVsUDZLEPKJJcwNlXmrE4VTsWBCMZGQZaR4d7YjZTpbcjgsdZTovy60ngj4hyRybBNQAq52Vx_NF6oL76ux6L5GU8jHkX9GKu-Q5XUc8TC9AfIAHKVLEg%3D%3D%22%5D%5D"),
        );
        h.insert(
            "x-csrftoken",
            reqwest::header::HeaderValue::from_static(
                "CmUTtU5KRa6cnCggj1H5EvhJvJ3tPytP4G82GuH5Q6t7CFFCZkD3FBbQLyE7KCe5",
            ),
        );

        h
    };
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let reponse = client.get(PROBLEMS_URL).send().unwrap();
    println!("Response: {:?}", reponse);
    let result = reponse.json();
    result.unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct Problem {
    pub title: String,
    pub title_slug: String,
    pub content: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: Vec<CodeDefinition>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    pub difficulty: String,
    pub question_id: u32,
    pub return_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String,
}

impl Query {
    fn question_query(title_slug: &str) -> Query {
        Query {
            operation_name: QUESTION_QUERY_OPERATION.to_owned(),
            variables: json!({ "titleSlug": title_slug }),
            query: QUESTION_QUERY_STRING.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RawProblem {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    content: String,
    stats: String,
    #[serde(rename = "codeDefinition")]
    code_definition: String,
    #[serde(rename = "sampleTestCase")]
    sample_test_case: String,
    #[serde(rename = "metaData")]
    meta_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problems {
    user_name: String,
    num_solved: u32,
    num_total: u32,
    ac_easy: u32,
    ac_medium: u32,
    ac_hard: u32,
    pub stat_status_pairs: Vec<StatWithStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatWithStatus {
    pub stat: Stat,
    difficulty: Difficulty,
    paid_only: bool,
    is_favor: bool,
    frequency: u32,
    progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    question_id: u32,
    #[serde(rename = "question__article__slug")]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    question_hide: bool,
    total_acs: u32,
    total_submitted: u32,
    pub frontend_question_id: u32,
    is_new_question: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Difficulty {
    level: u32,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.level {
            1 => f.write_str("Easy"),
            2 => f.write_str("Medium"),
            3 => f.write_str("Hard"),
            _ => f.write_str("Unknown"),
        }
    }
}
