pub struct BannerState {
    pub phase: u8,
    pub tick: u32,
    pub done: bool,
}

impl BannerState {
    pub fn new() -> Self {
        Self {
            phase: 0,
            tick: 0,
            done: false,
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        match self.tick {
            0..=50 => self.phase = 0,
            51..=95 => self.phase = 1,
            96..=140 => self.phase = 2,
            141..=175 => self.phase = 3,
            _ => self.done = true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_transitions_at_boundaries() {
        let mut state = BannerState::new();

        for _ in 0..50 {
            state.tick();
        }
        assert_eq!(state.phase, 0);
        assert!(!state.done);

        state.tick();
        assert_eq!(state.phase, 1);
        assert_eq!(state.tick, 51);

        for _ in 52..=95 {
            state.tick();
        }
        assert_eq!(state.phase, 1);

        state.tick();
        assert_eq!(state.phase, 2);
        assert_eq!(state.tick, 96);

        for _ in 97..=140 {
            state.tick();
        }
        assert_eq!(state.phase, 2);

        state.tick();
        assert_eq!(state.phase, 3);
        assert_eq!(state.tick, 141);

        for _ in 142..=175 {
            state.tick();
        }
        assert_eq!(state.phase, 3);
        assert!(!state.done);

        state.tick();
        assert!(state.done);
        assert_eq!(state.tick, 176);
    }
}
