use std::hash::Hash;

use failure::Error;
use std::hash::Hasher;
use std::num::FpCategory;

/// An `f64` that compares/hashes as if it was formatted to a string.
#[derive(Debug, Copy, Clone)]
pub struct TextualFloat {
    pub val: f64,
}

impl Hash for TextualFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(format!("{}", self.val).as_bytes());
    }
}

impl PartialEq for TextualFloat {
    fn eq(&self, other: &TextualFloat) -> bool {
        if self.val.is_sign_negative() != other.val.is_sign_negative() {
            return false;
        }

        let our_classification = self.val.classify();
        if our_classification != other.val.classify() {
            return false;
        }

        match our_classification {
            FpCategory::Normal | FpCategory::Subnormal => self.val == other.val,
            FpCategory::Zero | FpCategory::Infinite | FpCategory::Nan => true,
        }
    }
}

impl Eq for TextualFloat {}
