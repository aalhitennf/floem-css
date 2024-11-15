use std::time::Duration;

use floem::peniko::{Brush, Color};
use floem::prop;
use floem::style::{
    AlignContentProp, AlignItemsProp, AlignSelf, AspectRatio, Background, BorderBottom,
    BorderColor, BorderLeft, BorderRadius, BorderRight, BorderTop, BoxShadow, BoxShadowProp,
    ColGap, Cursor, CursorColor, CursorStyle, DisplayProp, FlexBasis, FlexDirectionProp, FlexGrow,
    FlexShrink, FlexWrapProp, FontFamily, FontSize, FontStyle, FontWeight, Height, InsetBottom,
    InsetLeft, InsetRight, InsetTop, JustifyContentProp, JustifySelf, LineHeight, MarginBottom,
    MarginLeft, MarginRight, MarginTop, MaxHeight, MaxWidth, MinHeight, MinWidth, Outline,
    OutlineColor, PaddingBottom, PaddingLeft, PaddingRight, PaddingTop, PositionProp, RowGap,
    Selectable, Style, StylePropValue, TextColor, TextOverflow, TextOverflowProp, Transition,
    Width, ZIndex,
};
use floem::taffy::{
    AlignContent, AlignItems, Display, FlexDirection, FlexWrap, JustifyContent, Position,
};
use floem::text::Weight;
use floem::unit::{Pct, Px, PxPct, PxPctAuto};
use floem::views::scroll::Border;
use floem_css_macros::StyleParser;
use smallvec::SmallVec;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BorderDef {
    width: Option<Px>,
    color: Option<Color>,
}

impl StylePropValue for BorderDef {}

prop!(pub Padding: PxPctAuto {} = PxPctAuto::Px(0.0));
prop!(pub Margin: PxPctAuto {} = PxPctAuto::Px(0.0));
prop!(pub TransitionProp: f64 {} = 0.0);
prop!(pub BorderProp: BorderDef {} = BorderDef::default());

#[derive(StyleParser)]
pub enum Declaration {
    #[property("display")]
    #[parser("parse_display")]
    #[style_class(DisplayProp)]
    Display(Display),

    #[property("position")]
    #[parser("parse_position")]
    #[style_class(PositionProp)]
    Position(Position),

    #[property("width")]
    #[parser("parse_pxpctauto")]
    #[style_class(Width)]
    Width(PxPctAuto),

    #[property("height")]
    #[parser("parse_pxpctauto")]
    #[style_class(Height)]
    Height(PxPctAuto),

    #[property("min-width")]
    #[parser("parse_pxpctauto")]
    #[style_class(MinWidth)]
    MinWidth(PxPctAuto),

    #[property("min-height")]
    #[parser("parse_pxpctauto")]
    #[style_class(MinHeight)]
    MinHeight(PxPctAuto),

    #[property("max-width")]
    #[parser("parse_pxpctauto")]
    #[style_class(MaxWidth)]
    MaxWidth(PxPctAuto),

    #[property("max-height")]
    #[parser("parse_pxpctauto")]
    #[style_class(MaxHeight)]
    MaxHeight(PxPctAuto),

    #[property("flex-direction")]
    #[parser("parse_flex_direction")]
    #[style_class(FlexDirectionProp)]
    FlexDirection(FlexDirection),

    #[property("flex-wrap")]
    #[parser("parse_flex_wrap")]
    #[style_class(FlexWrapProp)]
    FlexWrap(FlexWrap),

    #[property("flex-grow")]
    #[parser("parse_f32")]
    #[style_class(FlexGrow)]
    FlexGrow(f32),

    #[property("flex-shrink")]
    #[parser("parse_f32")]
    #[style_class(FlexShrink)]
    FlexShrink(f32),

    #[property("flex-basis")]
    #[parser("parse_pxpctauto")]
    #[style_class(FlexBasis)]
    FlexBasis(PxPctAuto),

    #[property("justify-content")]
    #[parser("parse_justify_content")]
    #[style_class(JustifyContentProp)]
    JustifyContent(JustifyContent),

    #[property("justify-self")]
    #[parser("parse_align_items")]
    #[style_class(JustifySelf)]
    JustifySelf(AlignItems),

    #[property("align-items")]
    #[parser("parse_align_items")]
    #[style_class(AlignItemsProp)]
    AlignItems(AlignItems),

    #[property("align-content")]
    #[parser("parse_align_content")]
    #[style_class(AlignContentProp)]
    AlignContent(AlignContent),

    #[property("align-self")]
    #[parser("parse_align_items")]
    #[style_class(AlignSelf)]
    AlignSelf(AlignItems),

    #[property("border")]
    #[parser("parse_border")]
    #[style_class(BorderProp)]
    Border(BorderDef),

    #[property("border-width")]
    #[parser("parse_px")]
    #[style_class(Border)]
    BorderWidth(Px),

    #[property("border-left")]
    #[parser("parse_px")]
    #[style_class(BorderLeft)]
    BorderLeft(Px),

    #[property("border-top")]
    #[parser("parse_px")]
    #[style_class(BorderTop)]
    BorderTop(Px),

    #[property("border-right")]
    #[parser("parse_px")]
    #[style_class(BorderRight)]
    BorderRight(Px),

    #[property("border-bottom")]
    #[parser("parse_px")]
    #[style_class(BorderBottom)]
    BorderBottom(Px),

    #[property("border-radius")]
    #[parser("parse_px_pct")]
    #[style_class(BorderRadius)]
    BorderRadius(PxPct),

    #[property("outline-color")]
    #[parser("parse_color")]
    #[style_class(OutlineColor)]
    OutlineColor(Color),

    #[property("outline")]
    #[parser("parse_px")]
    #[style_class(Outline)]
    Outline(Px),

    #[property("border-color")]
    #[parser("parse_color")]
    #[style_class(BorderColor)]
    BorderColor(Color),

    #[property("padding")]
    #[parser("parse_px_pct")]
    #[style_class(Padding)]
    Padding(PxPct),

    #[property("padding-left")]
    #[parser("parse_px_pct")]
    #[style_class(PaddingLeft)]
    PaddingLeft(PxPct),

    #[property("padding-top")]
    #[parser("parse_px_pct")]
    #[style_class(PaddingTop)]
    PaddingTop(PxPct),

    #[property("padding-right")]
    #[parser("parse_px_pct")]
    #[style_class(PaddingRight)]
    PaddingRight(PxPct),

    #[property("padding-bottom")]
    #[parser("parse_px_pct")]
    #[style_class(PaddingBottom)]
    PaddingBottom(PxPct),

    #[property("margin")]
    #[parser("parse_pxpctauto")]
    #[style_class(Margin)]
    Margin(PxPctAuto),

    #[property("margin-left")]
    #[parser("parse_pxpctauto")]
    #[style_class(MarginLeft)]
    MarginLeft(PxPctAuto),

    #[property("margin-top")]
    #[parser("parse_pxpctauto")]
    #[style_class(MarginTop)]
    MarginTop(PxPctAuto),

    #[property("margin-right")]
    #[parser("parse_pxpctauto")]
    #[style_class(MarginRight)]
    MarginRight(PxPctAuto),

    #[property("margin-bottom")]
    #[parser("parse_pxpctauto")]
    #[style_class(MarginBottom)]
    MarginBottom(PxPctAuto),

    #[property("left")]
    #[parser("parse_pxpctauto")]
    #[style_class(InsetLeft)]
    InsetLeft(PxPctAuto),

    #[property("top")]
    #[parser("parse_pxpctauto")]
    #[style_class(InsetTop)]
    InsetTop(PxPctAuto),

    #[property("right")]
    #[parser("parse_pxpctauto")]
    #[style_class(InsetRight)]
    InsetRight(PxPctAuto),

    #[property("bottom")]
    #[parser("parse_pxpctauto")]
    #[style_class(InsetBottom)]
    InsetBottom(PxPctAuto),

    #[property("z-index")]
    #[parser("parse_i32")]
    #[style_class(ZIndex)]
    ZIndex(i32),

    #[property("cursor")]
    #[parser("parse_cursor_style")]
    #[style_class(Cursor)]
    Cursor(CursorStyle),

    #[property("color")]
    #[parser("parse_color")]
    #[style_class(TextColor)]
    Color(Color),

    #[property("background-color")]
    #[parser("parse_color")]
    #[style_class(Background)]
    BackgroundColor(Color),

    #[property("box-shadow")]
    #[parser("parse_box_shadow")]
    #[style_class(BoxShadowProp)]
    BoxShadow(BoxShadow),

    #[property("font-size")]
    #[parser("parse_px")]
    #[style_class(FontSize)]
    FontSize(Px),

    #[property("font-family")]
    #[parser("to_owned")]
    #[style_class(FontFamily)]
    FontFamily(String),

    #[property("font-weight")]
    #[parser("parse_font_weight")]
    #[style_class(FontWeight)]
    FontWeight(Weight),

    #[property("font-style")]
    #[parser("parse_font_style")]
    #[style_class(FontStyle)]
    FontStyle(floem::text::Style),

    #[property("caret-color")]
    #[parser("parse_color")]
    #[style_class(CursorColor)]
    CursorColor(Color),

    #[property("text-wrap")]
    #[parser("parse_text_overflow")]
    #[style_class(TextOverflowProp)]
    TextOverflow(TextOverflow),

    #[property("line-height")]
    #[parser("parse_f32")]
    #[style_class(LineHeight)]
    LineHeight(f32),

    #[property("aspect-ratio")]
    #[parser("parse_f32")]
    #[style_class(AspectRatio)]
    AspectRatio(f32),

    #[property("column-gap")]
    #[parser("parse_px_pct")]
    #[style_class(ColGap)]
    ColGap(PxPct),

    #[property("row-gap")]
    #[parser("parse_px_pct")]
    #[style_class(RowGap)]
    RowGap(PxPct),

    #[property("gap")]
    #[parser("parse_gap")]
    #[style_class(RowGap)]
    Gap((PxPct, Option<PxPct>)),

    #[property("transition")]
    #[parser("parse_transition")]
    #[style_class(TransitionProp)]
    Transition((String, Transition)),

    #[property("user-select")]
    #[parser("parse_user_select")]
    #[style_class(Selectable)]
    UserSelect(bool),
}

impl Declaration {
    #[inline(never)]
    pub fn apply_style(self, s: Style) -> Style {
        match self {
            Self::Display(d) => s.display(d),
            Self::Position(p) => s.position(p),
            Self::Width(v) => s.width(v),
            Self::Height(v) => s.height(v),
            Self::MinWidth(v) => s.min_width(v),
            Self::MinHeight(v) => s.min_height(v),
            Self::MaxWidth(v) => s.max_width(v),
            Self::MaxHeight(v) => s.max_height(v),
            Self::FlexDirection(f) => s.flex_direction(f),
            Self::FlexWrap(f) => s.flex_wrap(f),
            Self::FlexGrow(f) => s.flex_grow(f),
            Self::FlexShrink(f) => s.flex_shrink(f),
            Self::FlexBasis(v) => s.flex_basis(v),
            Self::JustifyContent(j) => s.justify_content(j),
            Self::JustifySelf(a) => s.justify_self(a),
            Self::AlignItems(a) => s.align_items(a),
            Self::AlignContent(v) => s.align_content(v),
            Self::AlignSelf(v) => s.align_self(v),
            Self::Border(b) => s
                .apply_opt(b.width, |s, v| s.border(v.0))
                .apply_opt(b.color, Style::border_color),
            Self::BorderWidth(v) => s.border(v.0),
            Self::BorderLeft(v) => s.border_left(v.0),
            Self::BorderTop(v) => s.border_top(v.0),
            Self::BorderRight(v) => s.border_right(v.0),
            Self::BorderBottom(v) => s.border_bottom(v.0),
            Self::BorderRadius(v) => s.border_radius(v),
            Self::OutlineColor(v) => s.outline_color(v),
            Self::Outline(v) => s.outline(v.0),
            Self::BorderColor(v) => s.border_color(v),
            Self::Padding(v) => s.padding(v),
            Self::PaddingLeft(v) => s.padding_left(v),
            Self::PaddingTop(v) => s.padding_top(v),
            Self::PaddingRight(v) => s.padding_right(v),
            Self::PaddingBottom(v) => s.padding_bottom(v),
            Self::Margin(v) => s.margin(v),
            Self::MarginLeft(v) => s.margin_left(v),
            Self::MarginTop(v) => s.margin_top(v),
            Self::MarginRight(v) => s.margin_right(v),
            Self::MarginBottom(v) => s.margin_bottom(v),
            Self::InsetLeft(v) => s.inset_left(v),
            Self::InsetTop(v) => s.inset_top(v),
            Self::InsetRight(v) => s.inset_right(v),
            Self::InsetBottom(v) => s.inset_bottom(v),
            Self::ZIndex(v) => s.z_index(v),
            Self::Cursor(v) => s.cursor(v),
            Self::Color(v) => s.color(v),
            Self::BackgroundColor(v) => s.background(v),
            Self::BoxShadow(b) => s
                .box_shadow_blur(b.blur_radius)
                .box_shadow_color(b.color)
                .box_shadow_spread(b.spread)
                .box_shadow_h_offset(b.h_offset)
                .box_shadow_v_offset(b.v_offset),
            Self::FontSize(v) => s.font_size(v),
            Self::FontFamily(v) => s.font_family(v),
            Self::FontWeight(v) => s.font_weight(v),
            Self::FontStyle(v) => s.font_style(v),
            Self::CursorColor(v) => s.cursor_color(Brush::Solid(v)),
            Self::TextOverflow(v) => s.text_overflow(v),
            Self::LineHeight(v) => s.line_height(v),
            Self::AspectRatio(v) => s.aspect_ratio(v),
            Self::ColGap(v) => s.column_gap(v),
            Self::RowGap(v) => s.row_gap(v),
            Self::Gap(v) => s.row_gap(v.0).apply_opt(v.1, Style::column_gap),
            Self::Transition((key, t)) => Self::apply_transition(s, &key, t),
            Self::UserSelect(v) => s.selectable(v),
        }
    }
}

#[derive(Debug)]
pub struct ParseError<'a> {
    pub error: &'static str,
    pub value: &'a str,
}

impl<'a> ParseError<'a> {
    pub const fn new(error: &'static str, value: &'a str) -> Self {
        Self { error, value }
    }
}

impl std::fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.value)
    }
}

const fn parse_display(s: &str) -> Option<Display> {
    match s.as_bytes() {
        b"block" => Some(Display::Block),
        b"flex" => Some(Display::Flex),
        b"grid" => Some(Display::Grid),
        b"none" => Some(Display::None),
        _ => None,
    }
}

const fn parse_justify_content(s: &str) -> Option<JustifyContent> {
    match s.as_bytes() {
        b"start" => Some(JustifyContent::Start),
        b"end" => Some(JustifyContent::End),
        b"flex-start" => Some(JustifyContent::FlexStart),
        b"flex-end" => Some(JustifyContent::FlexEnd),
        b"center" => Some(JustifyContent::Center),
        b"stretch" => Some(JustifyContent::Stretch),
        b"space-between" => Some(JustifyContent::SpaceBetween),
        b"space-evenly" => Some(JustifyContent::SpaceEvenly),
        b"space-around" => Some(JustifyContent::SpaceAround),
        _ => None,
    }
}

const fn parse_align_items(s: &str) -> Option<AlignItems> {
    match s.as_bytes() {
        b"center" => Some(AlignItems::Center),
        b"start" => Some(AlignItems::Start),
        b"end" => Some(AlignItems::End),
        b"flex-start" => Some(AlignItems::FlexStart),
        b"flex-end" => Some(AlignItems::FlexEnd),
        b"baseline" => Some(AlignItems::Baseline),
        b"stretch" => Some(AlignItems::Stretch),
        _ => None,
    }
}

pub const fn parse_align_content(s: &str) -> Option<AlignContent> {
    match s.as_bytes() {
        b"center" => Some(AlignContent::Center),
        b"start" => Some(AlignContent::Start),
        b"end" => Some(AlignContent::End),
        b"flex-start" => Some(AlignContent::FlexStart),
        b"flex-end" => Some(AlignContent::FlexEnd),
        b"stretch" => Some(AlignContent::Stretch),
        b"space-between" => Some(AlignContent::SpaceBetween),
        b"space-evenly" => Some(AlignContent::SpaceEvenly),
        b"space-around" => Some(AlignContent::SpaceAround),
        _ => None,
    }
}

pub const fn parse_position(s: &str) -> Option<Position> {
    match s.as_bytes() {
        b"absolute" => Some(Position::Absolute),
        b"relative" => Some(Position::Relative),
        _ => None,
    }
}

pub const fn parse_flex_direction(s: &str) -> Option<FlexDirection> {
    match s.as_bytes() {
        b"row" => Some(FlexDirection::Row),
        b"column" => Some(FlexDirection::Column),
        b"row-reverse" => Some(FlexDirection::RowReverse),
        b"column-reverse" => Some(FlexDirection::ColumnReverse),
        _ => None,
    }
}

pub const fn parse_flex_wrap(s: &str) -> Option<FlexWrap> {
    match s.as_bytes() {
        b"wrap" => Some(FlexWrap::Wrap),
        b"no-wrap" => Some(FlexWrap::NoWrap),
        b"wrap-reverse" => Some(FlexWrap::WrapReverse),
        _ => None,
    }
}

fn parse_f32(s: &str) -> Option<f32> {
    s.parse::<f32>().ok()
}

fn parse_px(s: &str) -> Option<Px> {
    let pixels = s.strip_suffix("px")?;
    match pixels.trim_end().parse::<f64>() {
        Ok(value) => Some(Px(value)),
        Err(_) => None,
    }
}

fn parse_pct(s: &str) -> Option<Pct> {
    let percents = s.strip_suffix('%')?;
    match percents.trim_end().parse::<f64>() {
        Ok(value) => Some(Pct(value)),
        Err(_) => None,
    }
}

fn parse_px_pct(s: &str) -> Option<PxPct> {
    if let Some(px) = parse_px(s) {
        return Some(PxPct::Px(px.0));
    }
    if let Some(pct) = parse_pct(s) {
        return Some(PxPct::Pct(pct.0));
    }
    None
}

fn parse_pxpctauto(s: &str) -> Option<PxPctAuto> {
    if s == "auto" {
        return Some(PxPctAuto::Auto);
    }
    match parse_px_pct(s) {
        Some(PxPct::Px(px)) => Some(PxPctAuto::Px(px)),
        Some(PxPct::Pct(pct)) => Some(PxPctAuto::Pct(pct)),
        None => None,
    }
}

fn get_rgb_value(s: &str) -> Option<(usize, usize)> {
    let start = s.find('(').unwrap_or(0);
    let end = s[start..].find(')').unwrap_or(0);
    if end > start {
        Some((start + 1, end + 1))
    } else {
        None
    }
}

fn parse_color(s: &str) -> Option<Color> {
    if s.starts_with('#') {
        return Color::parse(s);
    }
    if s.starts_with("rgba") {
        let (start, end) = get_rgb_value(s)?;
        return parse_rgba(&s[start..end]);
    }
    if s.starts_with("rgb") {
        let (start, end) = get_rgb_value(s)?;
        return parse_rgb(&s[start..end]);
    }
    if s.starts_with("hsl") || s.starts_with("hwb") {
        // TODO Support these maybe
        return None;
    }
    Color::parse(s)
}

fn parse_i32(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

pub const fn parse_cursor_style(s: &str) -> Option<CursorStyle> {
    match s.as_bytes() {
        b"default" => Some(CursorStyle::Default),
        b"pointer" => Some(CursorStyle::Pointer),
        b"text" => Some(CursorStyle::Text),
        b"col-resize" => Some(CursorStyle::ColResize),
        b"row-resize" => Some(CursorStyle::RowResize),
        b"w-resize" => Some(CursorStyle::WResize),
        b"e-resize" => Some(CursorStyle::EResize),
        b"s-resize" => Some(CursorStyle::SResize),
        b"n-resize" => Some(CursorStyle::NResize),
        b"nw-resize" => Some(CursorStyle::NwResize),
        b"ne-resize" => Some(CursorStyle::NeResize),
        b"sw-resize" => Some(CursorStyle::SwResize),
        b"se-resize" => Some(CursorStyle::SeResize),
        b"nesw-resize" => Some(CursorStyle::NeswResize),
        b"nwse-resize" => Some(CursorStyle::NwseResize),
        _ => None,
    }
}

fn to_owned(s: &str) -> Option<String> {
    Some(s.to_string())
}

pub const fn parse_font_weight(s: &str) -> Option<Weight> {
    match s.as_bytes() {
        b"100" | b"thin" => Some(Weight(100)),
        b"200" => Some(Weight(200)),
        b"300" => Some(Weight(300)),
        b"400" | b"normal" => Some(Weight(400)),
        b"500" => Some(Weight(500)),
        b"600" => Some(Weight(600)),
        b"700" | b"bold" => Some(Weight(700)),
        b"800" => Some(Weight(800)),
        b"900" => Some(Weight(900)),
        _ => None,
    }
}

pub const fn parse_font_style(s: &str) -> Option<floem::text::Style> {
    match s.as_bytes() {
        b"normal" => Some(floem::text::Style::Normal),
        b"italic" => Some(floem::text::Style::Italic),
        b"oblique" => Some(floem::text::Style::Oblique),
        _ => None,
    }
}

pub const fn parse_text_overflow(s: &str) -> Option<TextOverflow> {
    match s.as_bytes() {
        b"clip" => Some(TextOverflow::Clip),
        b"ellipsis" => Some(TextOverflow::Ellipsis),
        b"wrap" => Some(TextOverflow::Wrap),
        _ => None,
    }
}

pub fn parse_gap(s: &str) -> Option<(PxPct, Option<PxPct>)> {
    let mut st = s.split_whitespace();
    let row_val = st.next()?;
    let row_px_pct = parse_px_pct(row_val)?;
    let col_val = st.next()?;
    let col_px_pct = parse_px_pct(col_val);
    Some((row_px_pct, col_px_pct))
}
#[allow(clippy::many_single_char_names)]
fn parse_box_shadow(s: &str) -> Option<BoxShadow> {
    let mut parts = SmallVec::<[&str; 5]>::new_const();
    let mut start = 0;
    let mut after_wp = false;
    for (i, c) in s.char_indices() {
        if c.is_whitespace() {
            parts.push(&s[start..i]);
            after_wp = true;
            start = i + 1;
        } else if after_wp && c.is_alphabetic() {
            break;
        } else {
            after_wp = false;
        }
    }
    parts.push(&s[start..]);
    match parts.as_slice() {
        ["none"] => Some(BoxShadow::default()),
        [a, b] => parse_box_shadow_2([a, b]),
        [a, b, c] => parse_box_shadow_3([a, b, c]),
        [a, b, c, d] => parse_box_shadow_4([a, b, c, d]),
        [a, b, c, d, e] => parse_box_shadow_5([a, b, c, d, e]),
        _ => None,
    }
}

fn parse_box_shadow_2([a, b]: [&str; 2]) -> Option<BoxShadow> {
    if let (Some(h_offset), Some(v_offset)) = (parse_px_pct(a), parse_px_pct(b)) {
        return Some(BoxShadow {
            h_offset,
            v_offset,
            ..BoxShadow::default()
        });
    };
    None
}

fn parse_box_shadow_3([a, b, c]: [&str; 3]) -> Option<BoxShadow> {
    // <h_offset> <v_offset> <color>
    if let (Some(h_offset), Some(v_offset), Some(color)) =
        (parse_px_pct(a), parse_px_pct(b), parse_color(c))
    {
        return Some(BoxShadow {
            color,
            h_offset,
            v_offset,
            ..BoxShadow::default()
        });
    }

    // <color> <h_offset> <v_offset>
    if let (Some(color), Some(h_offset), Some(v_offset)) =
        (parse_color(a), parse_px_pct(b), parse_px_pct(c))
    {
        return Some(BoxShadow {
            color,
            h_offset,
            v_offset,
            ..BoxShadow::default()
        });
    }
    // <h_offset> <v_offset> <blur>
    if let (Some(h_offset), Some(v_offset), Some(blur_radius)) =
        (parse_px_pct(a), parse_px_pct(b), parse_px_pct(c))
    {
        return Some(BoxShadow {
            blur_radius,
            h_offset,
            v_offset,
            ..BoxShadow::default()
        });
    }

    None
}
#[allow(clippy::many_single_char_names)]
fn parse_box_shadow_4([a, b, c, d]: [&str; 4]) -> Option<BoxShadow> {
    // <h_offset> <v_offset> <blur_radius> <color>
    if let (Some(h_offset), Some(v_offset), Some(blur_radius), Some(color)) = (
        parse_px_pct(a),
        parse_px_pct(b),
        parse_px_pct(c),
        parse_color(d),
    ) {
        return Some(BoxShadow {
            color,
            blur_radius,
            h_offset,
            v_offset,
            ..BoxShadow::default()
        });
    }
    // <color> <h_offset> <v_offset> <blur_radius>
    if let (Some(color), Some(h_offset), Some(v_offset), Some(blur_radius)) = (
        parse_color(a),
        parse_px_pct(b),
        parse_px_pct(c),
        parse_px_pct(d),
    ) {
        return Some(BoxShadow {
            color,
            blur_radius,
            h_offset,
            v_offset,
            ..BoxShadow::default()
        });
    }
    // <h_offset> <v_offset> <blur_radius> <blur_spread>
    if let (Some(h_offset), Some(v_offset), Some(blur_radius), Some(spread)) = (
        parse_px_pct(a),
        parse_px_pct(b),
        parse_px_pct(c),
        parse_px_pct(d),
    ) {
        return Some(BoxShadow {
            blur_radius,
            spread,
            h_offset,
            v_offset,
            ..BoxShadow::default()
        });
    }
    None
}
#[allow(clippy::many_single_char_names)]
fn parse_box_shadow_5([a, b, c, d, e]: [&str; 5]) -> Option<BoxShadow> {
    // <h_offset> <v_offset> <blur_radius> <blur_spread> <color>
    if let (Some(h_offset), Some(v_offset), Some(blur_radius), Some(spread), Some(color)) = (
        parse_px_pct(a),
        parse_px_pct(b),
        parse_px_pct(c),
        parse_px_pct(d),
        parse_color(e),
    ) {
        return Some(BoxShadow {
            h_offset,
            v_offset,
            blur_radius,
            spread,
            color,
        });
    }
    // <color> <h_offset> <v_offset> <blur_radius> <blur_spread>
    if let (Some(color), Some(h_offset), Some(v_offset), Some(blur_radius), Some(spread)) = (
        parse_color(a),
        parse_px_pct(b),
        parse_px_pct(c),
        parse_px_pct(d),
        parse_px_pct(e),
    ) {
        return Some(BoxShadow {
            h_offset,
            v_offset,
            blur_radius,
            spread,
            color,
        });
    }
    None
}

fn parse_rgba(s: &str) -> Option<Color> {
    let mut parts = SmallVec::<[&str; 4]>::new_const();
    parts.extend(s.split(',').map(str::trim));
    if let [r, g, b, a] = parts.as_slice() {
        if let (Some(r), Some(g), Some(b), Some(a)) = (
            parse_rgb_value(r),
            parse_rgb_value(g),
            parse_rgb_value(b),
            parse_rgb_alpha(a),
        ) {
            return Some(Color::rgba8(r, g, b, a));
        }
    }
    None
}

fn parse_rgb(s: &str) -> Option<Color> {
    let mut parts = SmallVec::<[&str; 3]>::new_const();
    parts.extend(s.split(',').map(str::trim));
    if let [r, g, b] = parts.as_slice() {
        if let (Some(r), Some(g), Some(b)) =
            (parse_rgb_value(r), parse_rgb_value(g), parse_rgb_value(b))
        {
            return Some(Color::rgb8(r, g, b));
        }
    }
    None
}

fn parse_rgb_value(s: &str) -> Option<u8> {
    s.parse::<u8>().ok()
}

fn parse_rgb_alpha(s: &str) -> Option<u8> {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    s.parse::<f64>()
        .map(|v| (v.clamp(0.0, 1.0) * 255.) as u8)
        .ok()
}

fn parse_transition(s: &str) -> Option<(String, Transition)> {
    let mut parts = s.split_whitespace();
    let key = parts.next()?;
    let duration_str = parts.next()?;
    let duration = parse_duration(duration_str)?;
    let transition = Transition::linear(duration);
    Some((key.to_string(), transition))
}

fn parse_duration(s: &str) -> Option<Duration> {
    if let Some(ms) = s.strip_suffix("ms") {
        if let Ok(d) = ms.parse::<u64>() {
            return Some(Duration::from_millis(d));
        }
    }
    if let Some(seconds) = s.strip_suffix('s') {
        if let Ok(f) = seconds.parse::<f64>() {
            if f > 0. {
                let ms = (f * 1000.) as u64;
                return Some(Duration::from_millis(ms));
            }
        }
    }
    None
}

const fn parse_user_select(s: &str) -> Option<bool> {
    match s.as_bytes() {
        b"none" => Some(false),
        b"auto" => Some(true),
        _ => None,
    }
}

fn parse_border(s: &str) -> Option<BorderDef> {
    let mut parts = s.split_whitespace();
    let first = parts.next();
    let second = parts.next();
    let mut retval = BorderDef {
        width: None,
        color: None,
    };
    let mut parse_val = |val: &str| {
        if let Some(px) = parse_px(val) {
            retval.width = Some(px);
            return Some(());
        } else if let Some(color) = parse_color(val) {
            retval.color = Some(color);
            return Some(());
        }
        None
    };
    match (first, second) {
        (Some(val), None) => {
            parse_val(val)?;
        }
        (Some(f), Some(s)) => {
            parse_val(f)?;
            parse_val(s)?;
        }
        _ => return None,
    }
    Some(retval)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use floem::{
        peniko::Color,
        unit::{Px, PxPct},
    };

    use crate::declaration::{
        get_rgb_value, parse_box_shadow_5, parse_rgb, parse_rgb_value, parse_rgba, BorderDef,
    };

    use super::{parse_border, parse_duration, parse_rgb_alpha};

    #[test]
    fn duration() {
        let sec = parse_duration("1s").unwrap();
        assert!(sec == Duration::from_secs(1));
        let tenth_sec = parse_duration("0.1s").unwrap();
        assert!(tenth_sec == Duration::from_millis(100));
        let ms = parse_duration("150ms").unwrap();
        assert!(ms == Duration::from_millis(150));
        // This should fail
        let value = parse_duration("1");
        assert!(value.is_none());
    }

    #[test]
    #[rustfmt::skip]
    fn border() {
        let v = parse_border("10px").unwrap();
        assert!(v == BorderDef { width: Some(Px(10.0)), color: None });
        let v = parse_border("10px red").unwrap();
        assert!(v == BorderDef { width: Some(Px(10.0)), color: Some(Color::RED) });
        let v = parse_border("red").unwrap();
        assert!(v == BorderDef { width: None, color: Some(Color::RED) });
    }

    #[test]
    fn rgb_alpha() {
        let v = parse_rgb_alpha("0.1").unwrap();
        assert!(v == 25);
        let v = parse_rgb_alpha("1.1").unwrap();
        assert!(v == 255);
        let v = parse_rgb_alpha("0").unwrap();
        assert!(v == 0);
    }

    #[test]
    fn rgb_value() {
        let v = parse_rgb_value("100").unwrap();
        assert!(v == 100);
        assert!(parse_rgb_value("300").is_none());
    }

    #[test]
    #[rustfmt::skip]
    fn rgb() {
        let v = parse_rgb("21, 22, 23").unwrap();
        assert!(v == Color {r: 21, g: 22, b: 23, a: 255 });
        assert!(parse_rgb("21, 22, 280").is_none());
    }

    #[test]
    #[rustfmt::skip]
    fn rgba() {
        let v = parse_rgba("21, 22, 23, 0.65").unwrap();
        assert!(v == Color {r: 21, g: 22, b: 23, a: 165 });
        assert!(parse_rgba("21, 22, 280, 0.1").is_none());
    }

    #[test]
    fn find_rgba_value() {
        let (start, end) = get_rgb_value("rgba(21, 22, 23, 0.65)").unwrap();
        assert!(start == 5);
        assert!(end == 18);
        let (start, end) = get_rgb_value("rgb(21, 22, 23)").unwrap();
        assert!(start == 4);
        assert!(end == 12);
        assert!(get_rgb_value("rgb(21, 22, 23").is_none());
    }

    #[test]
    fn box_shadow_5() {
        let v = parse_box_shadow_5(["4px", "8px", "10px", "15px", "black"]).unwrap();
        assert!(v.h_offset == PxPct::Px(4.0));
        assert!(v.v_offset == PxPct::Px(8.0));
        assert!(v.blur_radius == PxPct::Px(10.0));
        assert!(v.spread == PxPct::Px(15.0));
        assert!(v.color == Color::BLACK);
        let v = parse_box_shadow_5(["green", "4px", "8px", "10px", "15px"]).unwrap();
        assert!(v.h_offset == PxPct::Px(4.0));
        assert!(v.v_offset == PxPct::Px(8.0));
        assert!(v.blur_radius == PxPct::Px(10.0));
        assert!(v.spread == PxPct::Px(15.0));
        assert!(v.color == Color::GREEN);
    }
}
