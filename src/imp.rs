use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

pub(crate) fn add_rules_intellij(builder: &mut ThemeBuilder, palette: &Palette) {
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

    builder.add_rule(Semantic("typeParameter"), palette.teal());

    builder.add_rule(Semantic("macro"), FontStyle::BoldItalic);
}

pub(crate) fn add_rules_goland(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(&[Semantic("function"), Semantic("method")], palette.tan());
    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
        ],
        palette.dark_orange(),
    );

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
    builder.add_rule(Semantic("builtinType"), palette.brown());
    builder.add_rule(Semantic("typeParameter"), palette.teal());

    builder.add_rule(Semantic("selfKeyword"), palette.deep_blue());

    builder.add_rule(Semantic("namespace"), palette.avocado());

    builder.add_rule(Semantic("macro"), palette.yellow());
}

pub(crate) fn add_rules_clion(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
        ],
        palette.orange(),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("union"),
            Semantic("interface"),
            Semantic("builtinType"),
            Semantic("namespace"),
        ],
        palette.lavender(),
    );
    builder.add_rules(
        &[Semantic("typeAlias"), Semantic("typeParameter")],
        palette.pale_lavender(),
    );

    builder.add_rule(Semantic("property"), palette.purple());

    builder.add_rule(Semantic("macro"), palette.yellow());
}

pub(crate) fn add_rules_appcode(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("colon"), palette.brown());

    builder.add_rules(
        &[
            Semantic("function"),
            Semantic("method"),
            Semantic("property"),
        ],
        palette.beige(),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct:rust"),
            Semantic("typeParameter"),
        ],
        palette.dark_cyan(),
    );
    builder.add_rules(
        &[
            Semantic("struct"),
            Semantic("enum"),
            Semantic("union"),
            Semantic("builtinType"),
        ],
        palette.lavender(),
    );
    builder.add_rule(Semantic("typeAlias"), palette.pale_lavender());
    builder.add_rule(Semantic("interface"), palette.sea_green());

    builder.add_rule(Semantic("macro"), palette.yellow());

    builder.add_rules(
        &[
            Semantic("macro.attribute"),
            Semantic("builtinAttribute.attribute"),
        ],
        palette.brown(),
    );
}

pub(crate) fn add_rules_rider(builder: &mut ThemeBuilder, palette: &Palette) {
    let red = crate::palette::oklch(0.67977995, 0.20635639, 28.741695);
    let brown = crate::palette::oklch(0.73571193, 0.08352805, 74.40954);
    let grass_green = crate::palette::oklch(0.7568192, 0.13580728, 137.36841);
    let green = crate::palette::oklch(0.7539199, 0.15139778, 160.86707);
    let cyan = crate::palette::oklch(0.7635109, 0.08930822, 203.79019);
    let blue = crate::palette::oklch(0.6777812, 0.13572988, 263.8467);
    let purple = crate::palette::oklch(0.745359, 0.16016285, 302.83);
    let violet = crate::palette::oklch(0.8549312, 0.09425437, 308.5686);
    let pink = crate::palette::oklch(0.77019036, 0.11934556, 349.18854);

    builder.add_rules(&[Semantic("keyword"), Semantic("builtinType")], blue);

    builder.add_rules(&[Semantic("function"), Semantic("method")], green);

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("union"),
            Semantic("typeParameter"),
            Semantic("typeAlias"),
            Semantic("namespace"),
        ],
        purple,
    );
    builder.add_rule(Semantic("interface"), violet);

    builder.add_rule(Semantic("enumMember"), violet);

    builder.add_rule(Semantic("property"), cyan);

    builder.add_rule(Semantic("string"), brown);
    builder.add_rule(Semantic("escapeSequence"), pink);
    builder.add_rule(Semantic("formatSpecifier"), purple);

    builder.add_rules(&[Semantic("number"), Semantic("character")], pink);

    builder.add_rule(Semantic("macro"), pink);

    builder.add_rules(
        &[Semantic("lifetime"), Semantic("label")],
        (palette.base(BaseScale::LightFg), FontStyle::Bold),
    );

    builder.add_rules(
        &[
            Semantic("macro.attribute"),
            Semantic("builtinAttribute.attribute"),
        ],
        blue,
    );

    builder.add_rule(Semantic("comment"), (grass_green, FontStyle::Italic));

    builder.add_rule(Semantic("unresolvedReference"), (red, FontStyle::Bold));

    builder.add_rule(Textmate("magit.header"), FontStyle::Bold);
    builder.add_rule(Textmate("magit.subheader"), blue);
    builder.add_rule(Textmate("magit.entity"), palette.base(BaseScale::DarkFg));
    builder.add_rule(Textmate("markup.inserted"), green);
    builder.add_rule(Textmate("markup.deleted"), red);
}

pub(crate) fn add_rules_last(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("*.mutable"), FontStyle::Underline);
    builder.add_rule(Semantic("*.unsafe"), (palette.red(), FontStyle::Bold));
}

pub(crate) fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
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
        &[
            "sideBar.background",
            "panel.background",
            "activityBar.background",
        ],
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
    builder.add_workspace_rule(
        "editorSuggestWidget.foreground",
        palette.base(BaseScale::LightFg),
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
        "editorInlayHint.background",
        palette.base(BaseScale::LighterBg),
    );
    builder.add_workspace_rule(
        "editorInlayHint.foreground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rule(
        "editorIndentGuide.background",
        palette.base(BaseScale::BrightBg),
    );
    builder.add_workspace_rule(
        "editorIndentGuide.activeBackground",
        palette.base(BaseScale::BrightestBg),
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

    builder.add_workspace_rule("symbolIcon.classForeground", (palette.type_icon(), 0x9a));
    builder.add_workspace_rule("symbolIcon.structForeground", (palette.type_icon(), 0x9a));
    builder.add_workspace_rule(
        "symbolIcon.typeParameterForeground",
        (palette.type_icon(), 0x9a),
    );
    builder.add_workspace_rule("symbolIcon.moduleForeground", (palette.type_icon(), 0x9a));
    builder.add_workspace_rule(
        "symbolIcon.functionForeground",
        (palette.function_icon(), 0x9a),
    );
    builder.add_workspace_rule(
        "symbolIcon.enumeratorForeground",
        (palette.type_icon(), 0x9a),
    );
    builder.add_workspace_rule(
        "symbolIcon.methodForeground",
        (palette.function_icon(), 0x9a),
    );
    builder.add_workspace_rule(
        "symbolIcon.interfaceForeground",
        (palette.interface_icon(), 0x9a),
    );
    builder.add_workspace_rule(
        "symbolIcon.propertyForeground",
        (palette.property_icon(), 0x9a),
    );
    builder.add_workspace_rule(
        "symbolIcon.fieldForeground",
        (palette.property_icon(), 0x9a),
    );
    builder.add_workspace_rule(
        "symbolIcon.variableForeground",
        (palette.variable_icon(), 0x9a),
    );
    builder.add_workspace_rule(
        "symbolIcon.constantForeground",
        (palette.variable_icon(), 0x9a),
    );
    builder.add_workspace_rule(
        "symbolIcon.enumeratorMemberForeground",
        (palette.property_icon(), 0x9a),
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
            Semantic("variable.constant"),
            Semantic("variable.static"),
            Semantic("enumMember"),
        ],
        (palette.purple(), FontStyle::Italic),
    );

    builder.add_rule(Semantic("string"), palette.green());
    builder.add_rules(
        &[Semantic("escapeSequence"), Semantic("formatSpecifier")],
        palette.brown(),
    );

    builder.add_rules(&[Semantic("number"), Semantic("character")], palette.blue());

    builder.add_rule(Semantic("comment"), palette.base(BaseScale::DarkFg));
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

    builder.add_rule(Semantic("unresolvedReference"), palette.red());

    builder.add_rule(Textmate("magit.header"), FontStyle::Bold);
    builder.add_rule(Textmate("magit.subheader"), palette.blue());
    builder.add_rule(Textmate("magit.entity"), palette.base(BaseScale::DarkFg));
    builder.add_rule(Textmate("markup.inserted"), palette.green());
    builder.add_rule(Textmate("markup.deleted"), palette.red());
}
