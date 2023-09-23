use crate::chunk::Status;
use crate::{
    chunk::Chunk,
    elements::{Category, Elements},
    split::OPENING_DELIMITER,
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

    if let Some(release_group) = release_group_rare_case(c,d){
        e.add(Category::ReleaseGroup, &release_group);
        return;

    }
    
    if let Some(release_group) = release_group_normal_process(c, d){
        e.add(Category::ReleaseGroup, &release_group); 
    }
}

fn release_group_rare_case(c: &mut [Chunk], d: &[char]) -> Option<String>{
    let mut chunk_id = 0;
    while let Some(chunk ) = c.get(chunk_id) {
        if chunk.value().to_lowercase().trim_end_matches('s') == "fansub" {
            let start_id = chunk_id;
            let mut tmp_id = start_id;
            while c.get(tmp_id-1).is_some_and(|c| !c.is_status(Status::StrongDelimiter)) {
                if tmp_id <= 1 {
                    break;
                }
                tmp_id -= 1;
            }
        }
        if c.get(chunk_id+1).is_some_and(|c| c.value() == "by" && chunk.value() == "encoded"){
            //todo!("Implement encoded by");
            return Some(String::default());
        }
        chunk_id += 1;

    }
    None
}

fn release_group_normal_process(c: &mut [Chunk], d: &[char]) -> Option<String>{
    let mut chunk_id = 0;
    while c
        .get(chunk_id)
        .is_some_and(|c| !(c.is_status(Status::Unknown) && c.depth() > 0))
    {
        chunk_id += 1;
    }
    if chunk_id == c.len() {
        return None;
    }

    let mut release_group = String::default();
    while c
        .get(chunk_id)
        .is_some_and(|c| !(c.is_status(Status::Found) || c.is_status(Status::StrongDelimiter)))
    {
        release_group = format!("{}{}", release_group, c[chunk_id].value());
        chunk_id += 1;
    }

    release_group = release_group.clean(d);

    if !release_group.is_empty() {
        return Some(release_group);
    }

    None

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

            if chunk_index > 1 {
                let is_special_digit = c.get(chunk_index-1).is_some_and(|c| c.value().is_digit()) && c.get(chunk_index+1).is_some_and(|c| c.value().is_digit());
                if  is_special_digit {
                    episode_title = format!("{}{}", episode_title, c[chunk_index].value());
                    chunk_index += 1;
                    continue;
                }
            }
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

    e.add(
        Category::EpisodeTitle,
        episode_title.trim_matches(trim_delimiter.as_slice()),
    );
}

fn anime_title_start_id(chunks : &[Chunk]) -> Option<usize>{
    if chunks.is_empty() {
        return None
    }
    let mut start_id : usize = 0;
    if chunks.get(0).is_some_and(|c| c.is_status(Status::StrongDelimiter)){
        while chunks.get(start_id).is_some_and(|c| c.depth() > 0 || !c.is_status(Status::Unknown)){
            start_id += 1;
        }
        if start_id != chunks.len(){
            return Some(start_id)
        }

        start_id = 0;
        let mut strong_count = 0;
        
        while let Some(c) = chunks.get(start_id){
            if c.is_status(Status::Unknown) && strong_count > 2 {
                break;
            }
            if c.is_status(Status::StrongDelimiter){
                strong_count += 1; 
            }
            start_id += 1;
        }
        if start_id != chunks.len() {
            return Some(start_id)
        }
    }

    start_id = 0;
    while chunks.get(start_id).is_some_and(|c| !c.is_status(Status::Unknown)) {
        start_id +=1;
    }

    if start_id != chunks.len() {
        return Some(start_id)
    }

    None
}