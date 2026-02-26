/// Marker trait for a config field address.
///
/// Each impl lives on a unique zero-size type, giving compile-time uniqueness:
/// a type can have at most one `impl QuestionKey`, so two questions can never
/// share the same type — and therefore never accidentally collide at the same
/// config address without the programmer explicitly writing it that way.
pub trait QuestionKey {
    /// Dot-separated path used as the key in the flat answers map,
    /// e.g. `"personality.editor"`.
    const ADDRESS: &'static str;
}

// ── personality ───────────────────────────────────────────────────────────────

pub struct PersonalityEditor;
impl QuestionKey for PersonalityEditor {
    const ADDRESS: &'static str = "personality.editor";
}

pub struct PersonalityIndentation;
impl QuestionKey for PersonalityIndentation {
    const ADDRESS: &'static str = "personality.indentation";
}

pub struct PersonalityLanguage;
impl QuestionKey for PersonalityLanguage {
    const ADDRESS: &'static str = "personality.language";
}

pub struct PersonalitySchedule;
impl QuestionKey for PersonalitySchedule {
    const ADDRESS: &'static str = "personality.schedule";
}

pub struct PersonalityDebugStyle;
impl QuestionKey for PersonalityDebugStyle {
    const ADDRESS: &'static str = "personality.debug_style";
}
