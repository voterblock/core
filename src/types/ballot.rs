use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Ballot {
  pub questions: Vec<BallotQuestion>
}

#[derive(Debug)]
pub struct BallotQuestion {
  pub question: String,
  pub choices: Vec<String>
}

lazy_static! {
  static ref BALLOT_QUESTION_REGEX: Regex = Regex::new("^\".+\"::(?:\\[\\w[\\w\\s]+])+$").unwrap();
}

impl FromStr for Ballot {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let ballot_questions = s.lines().map(|line| match line {
      s if BALLOT_QUESTION_REGEX.is_match(s) => {
        // ""This is a question"::[These][Are][Choices]" --> [""This is a question"", "[These][Are][Choices]"]
        let v = s.split("::").collect::<Vec<&str>>();
        let [quoted_question, braced_choices] = <[&str; 2]>::try_from(v).ok().unwrap();

        // ""This is a question"" --> "This is a question"
        let question: String = (&quoted_question[1..quoted_question.len() - 1]).to_string();

        // "[These][Are][Choices]" --> ["These", "Are", "Choices"]
        let choices: Vec<String> = braced_choices.split("][").map(String::from).map(|choice| choice.replace("[", "").replace("]", "")).collect::<Vec<String>>();

        // BallotQuestion
        //   question: "This is a question",
        //   choices: ["These", "Are", "Choices"]
        Ok(BallotQuestion { question, choices })
      },
      _ => Err(())
    }).collect::<Result<Vec<BallotQuestion>, ()>>()?;

    return Ok(Ballot { questions: ballot_questions });
  }
}
