const DIGIT_DICT: [&'static str; 9] = [
    "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];
const TWENTY_DICT: [&'static str; 9] = [
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];
const TWO_DIGIT_DICT: [&'static str; 10] = [
    "Zero", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];
const THREE_DIGIT_DICT: [(i32, &str); 5] = [
    (9, " Billion"),
    (6, " Million"),
    (3, " Thousand"),
    (2, " Hundred"),
    (0, ""),
];

pub fn number_to_words(num: i32) -> String {
    if num < 100 {
        let num = num as usize;
        return if num % 10 == 0 {
            TWO_DIGIT_DICT[num / 10].to_string()
        } else if num < 10 {
            DIGIT_DICT[num - 1].to_string()
        } else if num < 20 {
            TWENTY_DICT[num - 11].to_string()
        } else {
            format!("{} {}", TWO_DIGIT_DICT[num / 10], DIGIT_DICT[num % 10 - 1])
        }
    }

    let mut output = Vec::new();
    let mut retain = num as u32;
    for (p, t) in THREE_DIGIT_DICT {
        if retain < 10u32.pow(p as u32) {
            continue;
        }

        let amount = retain / 10u32.pow(p as u32);
        retain -= 10u32.pow(p as u32) * amount;
        output.push(format!("{}{}", number_to_words(amount as i32), t));
    }

    output.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = number_to_words(123);
        assert_eq!(result, "One Hundred Twenty Three");
    }

    #[test]
    fn example_2() {
        let result = number_to_words(123);
        assert_eq!(result, "Twelve Thousand Three Hundred Forty Five");
    }

    #[test]
    fn example_3() {
        let result = number_to_words(123);
        assert_eq!(
            result,
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );
    }
}
