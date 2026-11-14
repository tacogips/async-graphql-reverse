use heck::SnakeCase;

pub trait SnakeCaseWithUnderscores: ToOwned {
    /// Convert this type to snake case without trimming leading / trailing underscores
    /// that might already be present on the string.
    fn to_snake_case_with_underscores(&self) -> Self::Owned;
}

impl SnakeCaseWithUnderscores for str {
    fn to_snake_case_with_underscores(&self) -> String {
        let leading_underscores: String = self.chars().take_while(|&c| c == '_').collect();
        let trailing_underscores: String = self.chars().rev().take_while(|&c| c == '_').collect();

        let trimmed = &self[leading_underscores.len()..self.len() - trailing_underscores.len()];

        format!(
            "{}{}{}",
            leading_underscores,
            trimmed.to_snake_case(),
            trailing_underscores
        )
    }
}
