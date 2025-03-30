use std::{fs::OpenOptions, io::Write};

use super::Render;

#[derive(Debug)]
pub struct RustLang {
    path: String,
    unit_test_template: String,
}

impl RustLang {
    pub fn new(path: &str) -> Self {
        RustLang {
            path: path.to_string(),
            unit_test_template: String::from(
                r#"struct Solution {}

// Copy paste the impl code block to your leetcode
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let length = prices.len();
        let mut dp = vec![[0i32; 2]; length];
        dp[0][0] = 0;
        dp[0][1] = -prices[0];
        for (i, price) in prices.iter().enumerate() {
            if i == 0 {
                continue;
            }
            dp[i][0] = Self::max(&dp[i - 1][0], &(dp[i - 1][1] + *price));
            dp[i][1] = Self::max(&dp[i - 1][1], &(dp[i - 1][0] - *price));
        }
        return dp[length - 1][0];
    }

    fn max(a: &i32, b: &i32) -> i32 {
        if *a >= *b {
            return *a;
        } else {
            return *b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Define your test case here
        struct TestCase<T> {
            input: Vec<T>,
            expect: T,
        }
        impl<T: Clone> TestCase<T> {
            fn new(input: &Vec<T>, expect: T) -> TestCase<T> {
                let input_copy = input.clone();
                return TestCase {
                    input: input_copy,
                    expect: expect.clone(),
                };
            }
            fn get_input(&self) -> Vec<T> {
                return self.input.clone();
            }
            fn get_expect(&self) -> T {
                return self.expect.clone();
            }
        }

        // Define your test_case instances here
        let test_cases = vec![
            TestCase::new(&vec![7, 1, 5, 3, 6, 4], 7),
            TestCase::new(&vec![1, 2, 3, 4, 5], 4),
            TestCase::new(&vec![7, 6, 4, 3, 1], 0),
        ];

        // iterate all test_cases and assert them to be as expected
        for test_case in test_cases.iter() {
            let result = Solution::max_profit(test_case.get_input());

            assert_eq!(result, test_case.get_expect());
        }
    }
}
"#,
            ),
        }
    }
}

impl Render for RustLang {
    fn render_unit_test(&mut self) -> Result<usize, std::io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;

        file.write(self.unit_test_template.as_bytes())
    }
}
