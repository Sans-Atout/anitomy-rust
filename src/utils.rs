use unicode_normalization::UnicodeNormalization;

use crate::{chunk::Chunk, traits::ExtendedString};

impl ExtendedString for str {
    fn remove_ignored(&self, ignored: &[String]) -> String {
        let mut return_value = self.to_string();
        for string in ignored {
            return_value = return_value.replace(string, "");
        }
        return_value
    }

    fn normalize(&self) -> String {
        self.nfkd()
            .filter(|c| c.is_ascii())
            .collect::<String>()
            .to_uppercase()
    }
}

#[cfg(test)]
mod test {
    use crate::traits::ExtendedString;

    #[test]
    fn remove_one_string() {
        let tested_string = "Hello World!";
        let r1 = tested_string.remove_ignored(&["World".to_string()]);
        assert_eq!(r1, "Hello !");
    }

    #[test]
    fn remove_multiple() {
        let tested_string = "Hello World!";
        let r1 = tested_string.remove_ignored(&["World".to_string(), "Hello".to_string()]);
        assert_eq!(r1, " !");
    }

    #[test]
    fn normalize() {
        assert_eq!("HELLO WORLD !", "Hello World !".normalize());
        assert_eq!("EPISODE1", "épisode1".normalize());
    }

    #[test]
    fn nothing_to_remove() {
        let tested_string = "EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv";
        let r2 = tested_string.remove_ignored(&["['EvoBot.']".to_string()]);
        assert_eq!(r2, tested_string);
    }

    #[test]
    fn real_test_data() {
        let tested_string = "EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv";
        let r2 = tested_string.remove_ignored(&["EvoBot.".to_string()]);
        assert_eq!(
            r2,
            "[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv"
        );
    }
}

