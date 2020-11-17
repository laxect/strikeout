use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("Can not parse as a episode")]
    EpisodeNotFound,
    #[error("Can not find a file extension")]
    ExtensionNotFound,
}

pub type Result<T> = std::result::Result<T, TransferError>;

pub fn trans(input: &str) -> Result<String> {
    let episode = find_episode_num(input)?;
    let extension = find_file_extension(input)?;
    let name = Name::new(episode, 1);
    Ok(format!("{}.{}", name, extension))
}

fn find_file_extension(input: &str) -> Result<&str> {
    if input.find('.').is_none() {
        return Err(TransferError::ExtensionNotFound);
    }
    let extension = input.rsplit('.').next().unwrap();
    if extension.is_empty() {
        return Err(TransferError::ExtensionNotFound);
    }
    Ok(extension)
}

fn find_episode_num(input: &str) -> Result<u16> {
    let blocks: Vec<&str> = input
        .split(|c| c == '[' || c == ']' || c == ' ')
        .filter(|s| !s.is_empty())
        .collect();
    for block in blocks {
        if block.chars().all(char::is_numeric) {
            return Ok(block.parse().unwrap());
        }
    }
    Err(TransferError::EpisodeNotFound)
}

struct Name {
    episode: u16,
    season: u16,
    episode_bit: usize,
}

impl Name {
    fn new(episode: u16, season: u16) -> Self {
        Self {
            episode,
            season,
            episode_bit: 2,
        }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut episode_formatted = self.episode.to_string();
        if episode_formatted.len() < self.episode_bit {
            let zeros = self.episode_bit - episode_formatted.len();
            let zeros: String = (0..zeros).into_iter().map(|_| "0").collect();
            episode_formatted.insert_str(0, &zeros);
        }
        let season_formatted = if self.season > 10 {
            self.season.to_string()
        } else {
            format!("0{}", self.season)
        };
        let name = ["S", &season_formatted, "E", &episode_formatted].concat();
        formatter.write_str(&name)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn st_test() {
        let input = "[Munou na Nana][06][BIG5][1080P].mp4";
        assert_eq!(trans(input).unwrap(), "S01E06.mp4".to_owned());
    }

    #[test]
    fn st_test_2() {
        let input = "[SweetSub&LoliHouse] Akudama Drive - 05 [WebRip 1080p HEVC-10bit AAC ASSx2].mkv";
        assert_eq!(trans(input).unwrap(), "S01E05.mkv".to_owned());
    }

    #[test]
    fn extension_test() {
        // no extension
        let in_1 = "aaa";
        assert!(find_file_extension(&in_1).is_err(), "There is no '.' in filename");
        let in_2 = "ccc.";
        assert!(
            find_file_extension(&in_2).is_err(),
            "The only '.' is last char of filename"
        );
        let in_3 = "cs.s.flv";
        assert_eq!(
            find_file_extension(&in_3).unwrap(),
            "flv",
            "should find flv as extension"
        );
    }
}
