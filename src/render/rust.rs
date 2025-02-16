use std::{
    fs::{File, OpenOptions},
    io::Write,
    string,
};

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
                r#"struct Solution;

// Copy paste the impl code block to your leetcode
impl Solution {
    // Edit your function below
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::binary_search(&nums, &target) as i32
    }

    fn binary_search(nums: &Vec<i32>, target: &i32) -> usize {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] < *target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Write your test case here
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(2, Solution::search_insert(nums, target));
        println!("finished")
    }
}"#,
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
