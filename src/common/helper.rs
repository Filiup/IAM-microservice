pub fn get_env_variable(variable_name: &str) -> String {
    std::env::var(variable_name)
        .unwrap_or_else(|_| panic!("{} env variable is not set.", variable_name))
}
