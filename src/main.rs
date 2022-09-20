use core::panic;
use pcre2::bytes::Regex;
extern crate colored;
use colored::*;
use std::fs::{read_to_string, write};
fn main() {
    //regexes
    let reg_is_1digit = Regex::new(r"\d\.").unwrap();
    let reg_answer_a = Regex::new(r" a(\)|\}).+?(?=b(\)|\}))").unwrap();
    let reg_answer_b = Regex::new(r" b(\)|\}).+?(?=c(\)|\}))").unwrap();
    let reg_answer_c = Regex::new(r" c(\)|\}).+").unwrap();
    let reg_correct_answer = Regex::new(r"[abcABC]").unwrap();
    let reg_question = Regex::new(r"(\d|\d\d)\. .+?(?=(\w\)|\w\}))").unwrap();
    //start
    let text_raw = read_to_string("QUESTIONS.txt").unwrap();
    let answers_raw = read_to_string("ANSWERS.txt").unwrap();
    let mut questions: Vec<String> = Vec::new();
    let mut answers_A: Vec<String> = Vec::new();
    let mut answers_B: Vec<String> = Vec::new();
    let mut answers_C: Vec<String> = Vec::new();
    let mut all_correct_anwsers: Vec<CorrectAnswer> = Vec::new();
    let mut relevant_correct_anwsers: Vec<CorrectAnswer> = Vec::new();
    let mut current_question_index = 0;
    let mut amnt_of_duplicates = 0;

    //parse correct answers
    for (i, line) in answers_raw.lines().enumerate() {
        if !line.contains("Test") || !line.contains("test") {
            if let Ok(Some(ans)) = reg_correct_answer.find(line.as_bytes()) {
                let start = ans.start();
                let end = ans.end();

                match &line[start..end] {
                    "a" | "A" => all_correct_anwsers.push(CorrectAnswer::A),
                    "b" | "B" => all_correct_anwsers.push(CorrectAnswer::B),
                    "c" | "C" => all_correct_anwsers.push(CorrectAnswer::C),
                    _ => {
                        println!("{}", format!("Line {} isn't somehow a|b|c", i).red());
                        panic!();
                    }
                }
            }
        }
    }
    //parse questions
    for (i, line) in text_raw.lines().enumerate() {
        let mut curr_question = String::new();

        //Get Question
        if let Ok(val) = reg_question.find(line.as_bytes()) {
            if let Some(question_pos) = val {
                let start = question_pos.start();
                let end = question_pos.end();
                if let Ok(_) = reg_is_1digit.is_match(&line.as_bytes()) {
                    curr_question = line[start + 3..end].trim().to_string();
                } else {
                    curr_question = line[start + 4..end].trim().to_string();
                }
            }
        }
        //If question is original
        if !questions.contains(&curr_question) {
            //Get All Answers
            questions.push(curr_question);
            if let Ok(Some(a)) = reg_answer_a.find(line.as_bytes()) {
                let start = a.start();
                let end = a.end();
                answers_A.push(line[start..end].trim().to_string());
            }
            if let Ok(Some(b)) = reg_answer_b.find(line.as_bytes()) {
                let start = b.start();
                let end = b.end();
                answers_B.push(line[start..end].trim().to_string());
            }
            if let Ok(Some(c)) = reg_answer_c.find(line.as_bytes()) {
                let start = c.start();
                let end = c.end();
                answers_C.push(line[start..end].trim().to_string());
            }
            //Get correct answer for current question
            relevant_correct_anwsers.push(
                all_correct_anwsers
                    .iter()
                    .nth(current_question_index)
                    .unwrap()
                    .clone(),
            );
        } else {
            amnt_of_duplicates = amnt_of_duplicates + 1;
        }
        current_question_index = current_question_index + 1;
    }

    println!(
        "{}",
        format!("\nAmount Of Duplicates: {}\nQuestions:", amnt_of_duplicates).red()
    );

    let mut output: String = String::new();

    for (i, question) in questions.iter().enumerate() {
        output = output + question + "\t";
        output = output + answers_A.get(i).unwrap() + "\t";
        if relevant_correct_anwsers.get(i).unwrap() == &CorrectAnswer::A {
            output = output + "TRUE" + "\t";
        } else {
            output = output + "FALSE" + "\t";
        }
        output = output + answers_B.get(i).unwrap() + "\t";
        if relevant_correct_anwsers.get(i).unwrap() == &CorrectAnswer::B {
            output = output + "TRUE" + "\t";
        } else {
            output = output + "FALSE" + "\t";
        }
        output = output + answers_C.get(i).unwrap() + "\t";
        if relevant_correct_anwsers.get(i).unwrap() == &CorrectAnswer::C {
            output = output + "TRUE" + "\n";
        } else {
            output = output + "FALSE" + "\n";
        }
    }
    write("PROCESSED_QUESTIONS.txt", output).unwrap();
}

#[derive(PartialEq, Clone)]
pub enum CorrectAnswer {
    A,
    B,
    C,
}

fn __is_sequence_correct(text_raw: String) {
    let mut q_index = 1;
    let mut curr_q = 0;

    for (i, line) in text_raw.lines().enumerate() {
        curr_q = match line[0..2].parse() {
            Ok(val) => val,
            Err(_err) => match line[0..1].parse() {
                Ok(val) => val,
                Err(_err) => {
                    print!("{} Not a number at the start of the line?", line.red());
                    panic!();
                }
            },
        };

        //actual comparisons
        if curr_q != q_index {
            println!(
                "{}",
                format!("\nLine {i}: curr_q:{curr_q} =! q_index:{q_index}\nText: {line}\n").red()
            );
            panic!();
        }
        if q_index < 15 {
            q_index = q_index + 1;
        } else {
            q_index = 1;
        }
    }
}
