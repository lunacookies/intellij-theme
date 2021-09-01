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
    builder.add_workspace_rule("editor.foreground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule("foreground", palette.base(BaseScale::LightFg));

    builder.add_workspace_rule(
        "editor.lineHighlightBackground",
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rule(
        "editorGutter.background",
        palette.base(BaseScale::LighterBg),
    );
    builder.add_workspace_rule(
        "editorLineNumber.foreground",
        palette.base(BaseScale::DarkerFg),
    );
    builder.add_workspace_rule(
        "editorLineNumber.activeForeground",
        palette.base(BaseScale::FadedFg),
    );

    builder.add_workspace_rules(
        &["sideBar.background", "activityBar.background"],
        palette.base(BaseScale::BrightBg),
    );

    builder.add_workspace_rules(
        &[
            "tab.inactiveBackground",
            "editorGroupHeader.tabsBackground",
            "editorGroupHeader.noTabsBackground",
            "tab.border",
        ],
        palette.base(BaseScale::BrightBg),
    );
    builder.add_workspace_rule("tab.activeBackground", palette.base(BaseScale::BrightestBg));
    builder.add_workspace_rule("tab.hoverBackground", palette.base(BaseScale::LightBg));
    builder.add_workspace_rules(
        &["tab.inactiveForeground", "tab.activeForeground"],
        palette.base(BaseScale::LightFg),
    );
    builder.add_workspace_rule("tab.activeBorderTop", palette.ui_blue());

    builder.add_workspace_rules(
        &["editorWidget.background", "editorWidget.border"],
        palette.base(BaseScale::BrighterBg),
    );

    builder.add_workspace_rules(
        &[
            "statusBar.background",
            "statusBar.debuggingBackground",
            "statusBar.noFolderBackground",
        ],
        palette.base(BaseScale::BrightBg),
    );
    builder.add_workspace_rule("statusBar.foreground", palette.base(BaseScale::LightFg));

    builder.add_workspace_rules(
        &["titleBar.inactiveBackground", "titleBar.activeBackground"],
        palette.base(BaseScale::BrightBg),
    );
    builder.add_workspace_rule(
        "titleBar.inactiveForeground",
        (palette.base(BaseScale::LightFg), 0x88),
    );
    builder.add_workspace_rule(
        "titleBar.activeForeground",
        palette.base(BaseScale::LightFg),
    );

    builder.add_workspace_rules(
        &["editorCursor.background", "terminalCursor.background"],
        palette.base(BaseScale::Bg),
    );
    builder.add_workspace_rules(
        &["editorCursor.foreground", "terminalCursor.foreground"],
        palette.base(BaseScale::LightFg),
    );

    builder.add_workspace_rule("terminal.foreground", palette.base(BaseScale::Fg));

    builder.add_workspace_rule("terminal.ansiBlack", palette.base(BaseScale::LightBg));
    builder.add_workspace_rule("terminal.ansiRed", palette.ansi_red());
    builder.add_workspace_rule("terminal.ansiGreen", palette.ansi_green());
    builder.add_workspace_rule("terminal.ansiYellow", palette.ansi_yellow());
    builder.add_workspace_rule("terminal.ansiBlue", palette.ansi_blue());
    builder.add_workspace_rule("terminal.ansiMagenta", palette.ansi_magenta());
    builder.add_workspace_rule("terminal.ansiCyan", palette.ansi_cyan());
    builder.add_workspace_rule("terminal.ansiWhite", palette.base(BaseScale::Fg));

    builder.add_workspace_rule("terminal.ansiBrightBlack", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule("terminal.ansiBrightRed", palette.ansi_bright_red());
    builder.add_workspace_rule("terminal.ansiBrightGreen", palette.ansi_bright_green());
    builder.add_workspace_rule("terminal.ansiBrightYellow", palette.ansi_bright_yellow());
    builder.add_workspace_rule("terminal.ansiBrightBlue", palette.ansi_bright_blue());
    builder.add_workspace_rule("terminal.ansiBrightMagenta", palette.ansi_bright_magenta());
    builder.add_workspace_rule("terminal.ansiBrightCyan", palette.ansi_bright_cyan());
    builder.add_workspace_rule("terminal.ansiBrightWhite", palette.base(BaseScale::Fg));

    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.background",
        palette.base(BaseScale::LighterBg),
    );
    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rules(
        &[
            "sideBar.border",
            "activityBar.border",
            "editorGroup.border",
            "editorGroupHeader.border",
            "panel.border",
            "statusBar.border",
            "titleBar.border",
        ],
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.border",
        palette.base(BaseScale::BrightBg),
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
