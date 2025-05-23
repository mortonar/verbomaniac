use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub word: String,
    pub meanings: Vec<Meaning>,
}

impl Word {
    pub fn first_definition(&self) -> Option<&str> {
        self.meanings
            .first()
            .and_then(|m| m.definitions.first())
            .map(|m| m.definition.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meaning {
    pub definitions: Vec<Definition>,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Definition {
    pub definition: String,
}

pub fn define<T: AsRef<str>>(word: T) -> Result<Option<Word>, reqwest::Error> {
    let url = format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        word.as_ref()
    );

    let mut words: Vec<Word> = reqwest::blocking::get(url)?.json()?;

    Ok(words.pop())
}
