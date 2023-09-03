use regex::Regex;

use crate::chunk::Status;
use crate::{
    chunk::Chunk,
    elements::{Category, Elements},
    keyword::Manager,
    split::{CLOSING_DELIMITER, OPENING_DELIMITER},
};

pub fn parse_anime_title(chunks: &mut [Chunk], e: &mut Elements, d: &[char]) {
    let mut chunk_index = 0;
    let mut anime_title = String::default();
    if chunks
        .get(chunk_index)
        .is_some_and(|c| c.is_status(Status::StrongDelimiter))
    {
        while chunks
            .get(chunk_index)
            .is_some_and(|c| c.depth() > 0 || c.is_status(Status::StrongDelimiter))
        {
            chunk_index += 1;
        }
    }
    if chunk_index == chunks.len() {
        return;
    }

    let mut before_is_delimiter = false;
    while chunks
        .get(chunk_index)
        .is_some_and(|c| !c.is_status(Status::Found))
    {
        if chunks[chunk_index].is_status(Status::WeakDelimiter) && !before_is_delimiter {
            before_is_delimiter = true;
            anime_title = format!("{} ", anime_title);
            chunk_index += 1;
            continue;
        }
        if chunks[chunk_index].is_status(Status::Unknown) {
            chunks[chunk_index].found();
        }
        before_is_delimiter = false;
        anime_title = format!("{}{}", anime_title, chunks[chunk_index].value());
        chunk_index += 1;
    }

    let mut trim_delimiter = d.to_vec();
    OPENING_DELIMITER.map(|d| trim_delimiter.push(d));

    e.add(
        Category::AnimeTitle,
        anime_title.trim_matches(trim_delimiter.as_slice()),
    );

}

pub fn parse_release_group(c: &mut [Chunk], e: &mut Elements, d: &[char]) {
    if !e.is_category_empty(Category::ReleaseGroup) {
        return;
    }

    let mut chunk_id = 0;
    while c
        .get(chunk_id)
        .is_some_and(|c| !(c.is_status(Status::Unknown) && c.depth() > 0))
    {
        chunk_id += 1;
    }
    if chunk_id == c.len() {
        return;
    }

    let mut release_group = String::default();
    while c
        .get(chunk_id)
        .is_some_and(|c| !(c.is_status(Status::Found) || c.is_status(Status::StrongDelimiter)))
    {
        release_group = format!("{}{}", release_group, c[chunk_id].value());
        chunk_id += 1;
    }

    if !release_group.is_empty() {
        e.add(Category::ReleaseGroup, release_group.trim_matches(d));
    }
}

pub fn parse_episode_title(c: &mut [Chunk], e: &mut Elements, d: &[char]) {
    println!("##########################################################");
    println!("Parse Episode Title");
    let mut chunk_index = 0;
    if c.get(chunk_index)
        .is_some_and(|c| c.is_status(Status::StrongDelimiter))
    {
        while c
            .get(chunk_index)
            .is_some_and(|c| c.depth() > 0 || c.is_status(Status::StrongDelimiter))
        {
            chunk_index += 1;
        }
    }
    if chunk_index == c.len() {
        return;
    }
    while c
        .get(chunk_index)
        .is_some_and(|c| !(c.is_status(Status::Unknown) && c.depth() == 0))
    {
        chunk_index += 1;
    }

    if chunk_index == c.len() {
        return;
    }

    let mut episode_title = String::default();
    let mut before_is_delimiter = false;
    while c
        .get(chunk_index)
        .is_some_and(|chunk| !chunk.is_status(Status::Found) && !chunk.is_status(Status::Found))
    {
        if c[chunk_index].is_status(Status::WeakDelimiter) && !before_is_delimiter {
            before_is_delimiter = true;
            episode_title = format!("{} ", episode_title);
            chunk_index += 1;
            continue;
        }
        if c[chunk_index].is_status(Status::Unknown) {
            c[chunk_index].found();
        }
        before_is_delimiter = false;
        episode_title = format!("{}{}", episode_title, c[chunk_index].value());
        chunk_index += 1;
    }
    let mut trim_delimiter = d.to_vec();
    OPENING_DELIMITER.map(|d| trim_delimiter.push(d));
    println!("##########################################################");

    e.add(Category::EpisodeTitle, episode_title.trim_matches(trim_delimiter.as_slice()));
}
