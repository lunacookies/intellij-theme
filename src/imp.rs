use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

pub(crate) fn add_rules_goland(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[
            Semantic("function"),
            Semantic("method"),
            Semantic("function.trait"),
            Semantic("method.trait"),
            Semantic("method.static"),
        ],
        palette.tan(),
    );
    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
            Semantic("function.trait.declaration"),
            Semantic("method.trait.declaration"),
            Semantic("method.static.declaration"),
        ],
        palette.dark_orange(),
    );

    builder.add_rule(Semantic("builtinType"), palette.brown());
    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("union"),
            Semantic("interface"),
            Semantic("typeAlias"),
        ],
        palette.cyan(),
    );
    builder.add_rules(
        &[
            Semantic("type.declaration"),
            Semantic("class.declaration"),
            Semantic("struct.declaration"),
            Semantic("enum.declaration"),
            Semantic("union.declaration"),
            Semantic("interface.declaration"),
            Semantic("typeAlias.declaration"),
        ],
        palette.base(BaseScale::Fg),
    );

    builder.add_rule(Semantic("property"), palette.base(BaseScale::Fg));

    builder.add_rule(Semantic("selfKeyword"), palette.deep_blue());

    builder.add_rule(Semantic("namespace"), palette.avocado());
}

pub(crate) fn add_rules_last(builder: &mut ThemeBuilder) {
    builder.add_rule(Semantic("*.mutable"), FontStyle::Underline);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["editor.foreground", "foreground"],
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rule(
        "editor.lineHighlightBackground",
        palette.base(BaseScale::LightBg),
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
        (palette.orange(), FontStyle::Inherit),
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

    builder.add_rules(
        &[Semantic("string"), Semantic("character")],
        palette.green(),
    );
    builder.add_rules(
        &[Semantic("escapeSequence"), Semantic("formatSpecifier")],
        palette.brown(),
    );

    builder.add_rule(Semantic("number"), palette.blue());

    builder.add_rule(Semantic("comment"), palette.base(BaseScale::DarkFg));
    builder.add_rule(
        Semantic("comment.documentation"),
        (palette.bright_green(), FontStyle::Italic),
    );

    builder.add_rule(Semantic("macro"), FontStyle::BoldItalic);

    builder.add_rule(Semantic("lifetime"), (palette.teal(), FontStyle::Italic));
    builder.add_rule(Semantic("label"), palette.bright_blue());

    builder.add_rules(
        &[
            Semantic("macro.attribute"),
            Semantic("builtinAttribute.attribute"),
        ],
        palette.yellow(),
    );

    builder.add_rule(Semantic("unresolvedReference"), palette.red());
}
