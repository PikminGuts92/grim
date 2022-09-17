use std::str::FromStr;


pub struct FormattedAnimEvent<'a> {
    text: &'a str,
    property: &'a str,
    values: Box<[&'a str]>
}

impl<'a> FormattedAnimEvent<'a> {
    pub fn try_from_str(text: &'a str) -> Option<FormattedAnimEvent<'a>> {
        if !is_anim_event(text) {
            return None;
        }

        let (property, values) = get_property_values(text).unwrap();

        Some(FormattedAnimEvent {
            text,
            property,
            values: values.into_boxed_slice()
        })
    }

    pub fn get_text(&'a self) -> &'a str {
        self.text
    }

    pub fn get_property(&'a self) -> &'a str {
        self.property
    }

    pub fn get_values(&'a self) -> &'a [&'a str] {
        &self.values
    }

    pub fn try_parse_values<const N: usize, T: FromStr + Copy>(&self) -> [Option<T>; N] {
        let text_values = self.get_values();
        let mut parsed_values = [None; N];

        for (i, p) in parsed_values.iter_mut().enumerate() {
            *p = text_values.get(i).and_then(|t| t.parse::<T>().ok());
        }

        parsed_values
    }
}

fn is_anim_event(text: &str) -> bool {
    if let Some((property, values)) = get_property_values(text) {
        are_chars_valid(property)
            && values
                .iter()
                .all(|v| are_chars_valid(v))
    } else {
        false
    }
}

fn are_chars_valid(text: &str) -> bool {
    const VALID_CHARS: [char; 7] = [
        '.', '_', '-', '[', ']', '(', ')'
    ];

    // Should be [a-zA-Z0-9._[]()]+
    !text.is_empty() && !text
        .chars()
        .any(|c| !(
            c.is_ascii_alphanumeric()
            || VALID_CHARS.iter().any(|vc| c.eq(vc))
        ))
}

fn get_property_values<'a>(text: &'a str) -> Option<(&'a str, Vec<&'a str>)> {
    // Extract values from "property (value0 value1 ...)" text
    if text.len() < 3 || !text.starts_with('[') || !text.ends_with(']') {
        return None;
    }

    // Remove []
    let text_no_brackets = &text[1..(text.len() - 1)];

    // Find whitespace start
    let whitespace_start = text_no_brackets.find(|c: char| c.is_whitespace());
    if whitespace_start.is_none() {
        return Some((text_no_brackets, Vec::new()));
    }

    // Split values separated by whitespace
    let (property, mut remaining) = text_no_brackets.split_at(whitespace_start.unwrap());
    remaining = remaining.trim_start();

    if remaining.is_empty() || !remaining.starts_with('(') || !remaining.ends_with(')')  {
        // Exit early. Invalid syntax. Don't parse values.
        return None;
    }

    // Remove ()
    let remaining_no_brackets = &remaining[1..(remaining.len() - 1)];

    // Split values in "array" by whitespace
    let values = remaining_no_brackets
        .split_whitespace()
        .collect();

    Some((
        property,
        values
    ))
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[rstest]
    #[case("", false)]
    #[case("[]", false)]
    #[case("[shot CrowdCvStage04.shot]", false)]
    #[case("[shot (CrowdCvStage04.shot)]", true)]
    #[case("[CrowdCvStage04.shot]", true)]
    #[case("[lid_weight (2.0)]", true)]
    #[case("[position (2.0 7.0 -4.77778)]", true)]
    #[case("[lighting_preset (Back_Spot_Patriot_Strobe_(2fpb).pst)]", true)]
    fn test_is_anim_event(#[case] input_text: &str, #[case] expected_result: bool) {
        let actual_result = is_anim_event(input_text);
        assert_eq!(expected_result, actual_result);
    }

    #[rstest]
    #[case("",                     false)] // Empty
    #[case(" ",                    false)] // Whitespace
    #[case("420",                   true)] // Number
    #[case("shot",                  true)] // Lower
    #[case("face_clip_balance",     true)] // Underscore
    #[case("kWaypointConfigLegacy", true)] // Upper
    #[case("CrowdCvStage04.shot",   true)] // Period
    #[case("some-random-thing",     true)] // Hyphen (Needed for negative numbers)
    #[case("Strobe_(2fpb).pst",     true)] // Circle brackets
    #[case("Strobe_[2fpb].pst",     true)] // Square brackets
    fn test_are_char_valid(#[case] input_text: &str, #[case] expected_result: bool) {
        let actual_result = are_chars_valid(input_text);
        assert_eq!(expected_result, actual_result);
    }

    #[rstest]
    #[case("some_random_event", None)]
    #[case("", None)]  // Empty
    #[case("[", None)] // Malformed
    #[case("[))(", None)]
    #[case("[shot (CrowdCvStage04.shot)]", Some(("shot", vec!["CrowdCvStage04.shot"])))]
    #[case("[shot (CrowdCvStage04.shot)]", Some(("shot", vec!["CrowdCvStage04.shot"])))]
    #[case("[add_face_weight (0.2)]", Some(("add_face_weight", vec!["0.2"])))]
    #[case("[attention (FOCUS_Crowd_Interest01.intr)]", Some(("attention", vec!["FOCUS_Crowd_Interest01.intr"])))]
    #[case("[position (0.0 5.0 -7.5)]", Some(("position", vec!["0.0", "5.0", "-7.5"])))]
    #[case("[lighting_preset (Back_Spot_Patriot_Strobe_(2fpb).pst)]", Some(("lighting_preset", vec!["Back_Spot_Patriot_Strobe_(2fpb).pst"])))]
    fn test_get_property_values(#[case] input_text: &str, #[case] expected_result: Option<(&str, Vec<&str>)>) {
        let actual_result = get_property_values(input_text);
        assert_eq!(expected_result, actual_result);
    }
}