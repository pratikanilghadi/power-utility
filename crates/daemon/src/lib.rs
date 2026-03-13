pub mod fetch_hotkeys {
    use global_hotkey::hotkey::{Code, HotKey, Modifiers};

    pub struct UtilityHotkeys {
        key_count: i8,
        modifier_keys: Vec<Modifiers>,
        code_key: Code,
    }

    pub fn set_keys<'a>() -> Vec<UtilityHotkeys> {
        let launcher_key = UtilityHotkeys {
            key_count: 2,
            modifier_keys: vec![Modifiers::ALT],
            code_key: Code::Space,
        };

        vec![launcher_key]
    }
    pub fn load_keys<'a>() -> Vec<HotKey> {
        let all_keys = set_keys();
        let mut all_hotkeys: Vec<HotKey> = vec![];

        for ht_ele in all_keys {
            if ht_ele.key_count == 2 {
                all_hotkeys.push(HotKey::new(Some(ht_ele.modifier_keys[0]), ht_ele.code_key));
            } else {
                all_hotkeys.push(HotKey::new(
                    Some(ht_ele.modifier_keys[0] | ht_ele.modifier_keys[1]),
                    ht_ele.code_key,
                ));
            }
        }

        all_hotkeys
    }
}
