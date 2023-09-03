use crate::elements::Elements;

pub trait ChunksManipulation{
    fn get_isolated_number(&self) -> Vec<usize>;
    fn isolated_id(&self) -> Option<usize>;

}

pub trait ExtendedString {
    fn remove_ignored(&self, ignored: &[String]) -> String;
    fn normalize(&self) -> String;
}

pub trait ParsingNumber {
    fn is_crc32(&self) -> bool;
    fn is_resolution(&self) -> bool;
    fn is_hexa(&self) -> bool;
    fn is_digit(&self) -> bool;
    fn is_anime_year(&self) -> bool;
    fn ordinals_to_nb(&self) -> &str;
    fn contains_digit(&self) -> bool;
}

pub trait EpisodeMatching {
    fn is_fractal_ep(&self, e : &mut Elements) -> bool;
    fn is_japanese_ep(&self, e : &mut Elements ) -> bool;
    fn is_partial_ep(&self, e : &mut Elements ) -> bool;
    fn is_season_ep(&self, e : &mut Elements ) -> bool;
    fn is_multiple_ep(&self, e : &mut Elements ) -> bool;
    fn is_single_ep(&self, e : &mut Elements ) -> bool;
    fn match_episode_type_pattern(&self, e : &mut Elements, d : &[char]) -> bool;
    fn is_number_sign_pattern(&self, e : &mut Elements ) -> bool;
}