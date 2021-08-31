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
    builder.add_rules(
        &[
            Semantic("keyword"),
            Semantic("boolean"),
            Semantic("comma"),
            Semantic("semicolon"),
        ],
        palette.brown(),
    );

    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
        ],
        palette.orange(),
    );
    builder.add_rules(
        &[Semantic("function.trait"), Semantic("method.trait")],
        (palette.orange(), FontStyle::Italic),
    );
    builder.add_rules(
        &[Semantic("function"), Semantic("method.static")],
        FontStyle::Italic,
    );

    builder.add_rule(Semantic("property"), palette.purple());
    builder.add_rules(
        &[
            Semantic("variable.constant"),
            Semantic("variable.static"),
            Semantic("enumMember"),
        ],
        (palette.purple(), FontStyle::Italic),
    );

    builder.add_rule(Semantic("typeParameter"), palette.teal());

    builder.add_rule(Semantic("string"), palette.green());
    builder.add_rules(
        &[Semantic("escapeSequence"), Semantic("formatSpecifier")],
        palette.brown(),
    );

    builder.add_rule(Semantic("number"), palette.blue());

    builder.add_rule(
        Semantic("comment.documentation"),
        (palette.bright_green(), FontStyle::Italic),
    );

    builder.add_rule(Semantic("lifetime"), (palette.teal(), FontStyle::Italic));
    builder.add_rule(Semantic("label"), palette.bright_blue());

    builder.add_rules(
        &[
            Semantic("macro.attribute"),
            Semantic("builtinAttribute.attribute"),
        ],
        palette.yellow(),
    );
}
