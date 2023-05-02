pub mod db_airbases;
pub mod lua_utils;
pub mod oob_air;
pub mod serde_utils;
pub mod dce_utils;
pub mod projections;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
