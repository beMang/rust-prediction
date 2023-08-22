use std::fs::File;
use std::io::prelude::*;
use std::{fs::OpenOptions, io::Error, path::Path};

/**
 * Renvoie le contenu en entier d'un fichier
 */
pub fn read_file(file_name: &str) -> Result<String, Error> {
    let mut content = String::new();
    File::open(file_name)?.read_to_string(&mut content)?;

    Ok(content)
}

/**
 * Renvoie un vecteur contenant les lignes d'un fichier
 */
pub fn get_lines(file_name: &str) -> Vec<String> {
    let content = read_file(file_name).expect("Failed to read file");

    let lines: Vec<&str> = content.split("\n").collect();
    let mut result: Vec<String> = Vec::new();
    for l in lines {
        result.push(String::from(l));
    }
    result
}

/**
 * Renvoie un vecteur contenant les phrases d'un fichier
 */
pub fn get_sentences(file_name: &str) -> Vec<String> {
    let content = read_file(file_name).expect("Failed to read file");

    //We split the sentences
    let lines: Vec<&str> = content
        .split(|c| {
            c == '\n'
                || c == '.'
                || c == '?'
                || c == '!'
                || c == '\"'
                || c == '('
                || c == ')'
                || c == '{'
                || c == '}'
        })
        .collect();

    let mut result: Vec<String> = Vec::new();
    for l in lines {
        if !l.is_empty() {
            result.push(String::from(l));
        }
    }
    result
}

/**
 * Renvoie les fichiers et dossier contenu dans un dossier
 */
pub fn files_in_dir(dir_name: &str) -> Option<Vec<String>> {
    let path: &Path = Path::new(dir_name);
    if !path.is_dir() {
        return None;
    };

    let entries = path.read_dir().expect("Called to read_dir faile");
    let mut result = Vec::new();
    for i in entries {
        result.push(
            i.expect("Error while reading dir")
                .file_name()
                .to_str()
                .expect("Failed conversion")
                .to_string(),
        );
    }

    return Some(result);
}

pub fn append_file(file_name: &str, content: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(Path::new(file_name))
        .expect("Failed to open file with append");

    file.write_all(content.as_bytes())
        .expect("Failed to append file");
}

pub fn write_file_truncate(file_name: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(Path::new(file_name))
        .expect("Failed to open file with truncate");

    file.write_all(content.as_bytes()).expect("Failed to write file");
}
