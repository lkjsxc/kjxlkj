//! Tests for the register set.

#[cfg(test)]
mod tests {
    use crate::register::{RegisterSet, RegisterType};

    #[test]
    fn yank_updates_unnamed_and_zero() {
        let mut regs = RegisterSet::new();
        regs.store_yank(None, "hello".into(), RegisterType::Charwise);
        assert_eq!(regs.read('"'), "hello");
        assert_eq!(regs.read('0'), "hello");
    }

    #[test]
    fn named_register_write_read() {
        let mut regs = RegisterSet::new();
        regs.store_yank(Some('a'), "test".into(), RegisterType::Charwise);
        assert_eq!(regs.read('a'), "test");
    }

    #[test]
    fn uppercase_appends() {
        let mut regs = RegisterSet::new();
        regs.store_yank(Some('a'), "hello".into(), RegisterType::Charwise);
        regs.store_yank(Some('A'), " world".into(), RegisterType::Charwise);
        assert_eq!(regs.read('a'), "hello world");
    }

    #[test]
    fn delete_rotates_numbered() {
        let mut regs = RegisterSet::new();
        regs.store_delete(None, "first\n".into(), RegisterType::Linewise, true);
        regs.store_delete(None, "second\n".into(), RegisterType::Linewise, true);
        assert_eq!(regs.read('1'), "second\n");
        assert_eq!(regs.read('2'), "first\n");
    }

    #[test]
    fn small_delete_non_linewise() {
        let mut regs = RegisterSet::new();
        regs.store_delete(None, "wo".into(), RegisterType::Charwise, false);
        assert_eq!(regs.read('-'), "wo");
    }

    #[test]
    fn black_hole_discards() {
        let mut regs = RegisterSet::new();
        regs.store_delete(Some('_'), "gone".into(), RegisterType::Charwise, false);
        assert_eq!(regs.read('_'), "");
    }
}
