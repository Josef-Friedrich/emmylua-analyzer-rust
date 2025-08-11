use emmylua_code_analysis::{DbIndex, LuaType, RenderLevel, humanize_type};

pub fn render_typ(db: &DbIndex, typ: &LuaType, level: RenderLevel) -> String {
    match typ {
        LuaType::IntegerConst(_) => "integer".to_string(),
        LuaType::FloatConst(_) => "number".to_string(),
        LuaType::StringConst(_) => "string".to_string(),
        LuaType::BooleanConst(_) => "boolean".to_string(),
        _ => humanize_type(db, typ, level),
    }
}

pub fn render_const(typ: &LuaType) -> Option<String> {
    match typ {
        LuaType::IntegerConst(i) | LuaType::DocIntegerConst(i) => Some(i.to_string()),
        LuaType::FloatConst(f) => Some(f.to_string()),
        LuaType::StringConst(s) | LuaType::DocStringConst(s) => {
            Some(format!("{:?}", s.to_string()))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internment::ArcIntern;


    #[test]
    fn test_render_const_none() {
        // Should return None for non-const types
        assert_eq!(render_const(&LuaType::Any), None);
    }

    #[test]
    fn test_render_const_integer_const() {
        assert_eq!(render_const(&LuaType::IntegerConst(42)), Some("42".to_string()));
    }

    #[test]
    fn test_render_const_doc_integer_const() {
        assert_eq!(render_const(&LuaType::DocIntegerConst(123)), Some("123".to_string()));
    }

    #[test]
    fn test_render_const_float_const() {
        assert_eq!(render_const(&LuaType::FloatConst(3.14)), Some("3.14".to_string()));
    }

    #[test]
    fn test_render_const_string_const() {
        let typ = LuaType::StringConst(ArcIntern::new(smol_str::SmolStr::new("hello")));
        assert_eq!(render_const(&typ), Some("\"hello\"".to_string()));
    }
    // #[test]
    // fn test_render_const_doc_string_const() {
    //     let typ = LuaType::DocStringConst("world".into());
    //     assert_eq!(render_const(&typ), Some("\"world\"".to_string()));
    // }
}
