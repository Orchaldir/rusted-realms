pub struct HealthComponent {
    pub penalty: u32,
    pub is_alive: bool,
}

impl HealthComponent {
    pub fn new() -> HealthComponent {
        HealthComponent {
            penalty: 0,
            is_alive: true,
        }
    }
}
