//! Unit tests for core types - part 2.

use super::*;

#[cfg(test)]
mod mode_tests {
    use super::*;

    #[test]
    fn test_mode_default() {
        assert_eq!(Mode::default(), Mode::Normal);
    }

    #[test]
    fn test_mode_is_visual() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
    }

    #[test]
    fn test_mode_is_insert_like() {
        assert!(Mode::Insert.is_insert_like());
        assert!(Mode::Replace.is_insert_like());
        assert!(!Mode::Normal.is_insert_like());
    }

    #[test]
    fn test_mode_name() {
        assert_eq!(Mode::Normal.name(), "NORMAL");
        assert_eq!(Mode::Insert.name(), "INSERT");
    }
}

#[cfg(test)]
mod version_tests {
    use super::*;

    #[test]
    fn test_version_initial() {
        let v = BufferVersion::initial();
        assert_eq!(v.raw(), 0);
    }

    #[test]
    fn test_version_next() {
        let v1 = BufferVersion::initial();
        let v2 = v1.next();
        assert!(v2.raw() > v1.raw());
    }
}

#[cfg(test)]
mod window_tests {
    use super::*;

    #[test]
    fn test_window_id() {
        let id = WindowId::new(5);
        assert_eq!(id.raw(), 5);
        assert_eq!(WindowId::default().raw(), 0);
    }
}
