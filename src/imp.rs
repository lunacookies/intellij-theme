use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["editor.foreground", "foreground"],
        palette.base(BaseScale::Fg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("keyword"), palette.orange());

    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
        ],
        palette.yellow(),
    );
    builder.add_rules(
        &[Semantic("function.trait"), Semantic("method.trait")],
        (palette.yellow(), FontStyle::Italic),
    );
    builder.add_rule(Semantic("method.static"), FontStyle::Italic);

    builder.add_rule(Semantic("property"), palette.purple());

    builder.add_rule(Semantic("typeParameter"), palette.teal());

    builder.add_rule(Semantic("string"), palette.green());
    builder.add_rule(Semantic("number"), palette.blue());
}
