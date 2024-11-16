use floem::{
    style::{BorderRadius, SelectionCornerRadius, Style, StyleValue, TextColor},
    unit::PxPct,
    views::{editor::SelectionColor, PlaceholderTextClass},
};
use floem_css_parser::{css_to_rules, declaration::Declaration, PseudoClass, Rule};

use crate::style::StyleMap;

fn rule_to_style(value: &Rule) -> Style {
    let mut style = Style::new();
    for kv in value.iter_props() {
        if let Some(d) = Declaration::from_cow(kv) {
            style = d.apply_style(style);
        }
    }
    style
}

#[must_use]
pub fn parse_rules(rules: &[Rule]) -> StyleMap {
    let mut map = StyleMap::new_const();
    for rule in rules {
        let style = rule_to_style(rule);
        for selector in &rule.selectors {
            let selector_style = style.clone();
            let to_modify = map.remove(selector.selector).unwrap_or_default();
            let modified = modify_selector(&selector.pseudo_class, to_modify, selector_style);
            map.insert(selector.selector, modified);
        }
    }
    map
}

fn modify_selector(
    pseudo_class: &Option<PseudoClass>,
    to_modify: Style,
    selector_style: Style,
) -> Style {
    if pseudo_class.is_none() {
        return to_modify.apply(selector_style);
    }
    let pseudo_class = pseudo_class.unwrap();
    apply_pseudo(pseudo_class, to_modify, selector_style)
}

#[cold]
fn apply_pseudo(pseudo_class: PseudoClass, to_modify: Style, selector_style: Style) -> Style {
    match pseudo_class {
        PseudoClass::Hover => to_modify.hover(|_| selector_style),
        PseudoClass::Focus => to_modify.focus(|_| selector_style),
        PseudoClass::FocusHover => to_modify.focus(|s| s.hover(|_| selector_style)),
        PseudoClass::Active => to_modify.active(|_| selector_style),
        PseudoClass::ActiveHover => to_modify.active(|s| s.hover(|_| selector_style)),
        PseudoClass::Disabled => to_modify.disabled(|_| selector_style),
        PseudoClass::DisabledHover => to_modify.disabled(|s| s.hover(|_| selector_style)),
        PseudoClass::Placeholder => {
            to_modify.class(PlaceholderTextClass, |s| s.apply(selector_style))
        }
        PseudoClass::Selection => {
            // TODO Maybe ugly maybe not
            if let StyleValue::Val(PxPct::Px(radius)) = selector_style.get_style_value(BorderRadius)
            {
                return to_modify.set_style_value(SelectionCornerRadius, radius.into());
            }
            if let StyleValue::Val(Some(color)) = selector_style.get_style_value(TextColor) {
                return to_modify.set_style_value(SelectionColor, color.into());
            }
            to_modify
        }
    }
}

#[must_use]
pub fn parse_css(input: &str) -> StyleMap {
    let now = std::time::SystemTime::now();

    let rules = css_to_rules(input);
    let mut map = StyleMap::new_const();
    for rule in &rules {
        let style = rule_to_style(rule);
        for selector in &rule.selectors {
            let selector_style = style.clone();
            let mut to_modify = map.remove(selector.selector).unwrap_or_default();
            let to_insert = match selector.pseudo_class {
                Some(PseudoClass::Hover) => to_modify.hover(|_| selector_style),
                Some(PseudoClass::Focus) => to_modify.focus(|_| selector_style),
                Some(PseudoClass::FocusHover) => to_modify.focus(|s| s.hover(|_| selector_style)),
                Some(PseudoClass::Active) => to_modify.active(|_| selector_style),
                Some(PseudoClass::ActiveHover) => to_modify.active(|s| s.hover(|_| selector_style)),
                Some(PseudoClass::Disabled) => to_modify.disabled(|_| selector_style),
                Some(PseudoClass::DisabledHover) => {
                    to_modify.disabled(|s| s.hover(|_| selector_style))
                }
                Some(PseudoClass::Placeholder) => {
                    to_modify.class(PlaceholderTextClass, |s| s.apply(selector_style))
                }
                Some(PseudoClass::Selection) => {
                    // TODO Maybe ugly maybe not
                    if let StyleValue::Val(PxPct::Px(radius)) =
                        selector_style.get_style_value(BorderRadius)
                    {
                        to_modify = to_modify.set_style_value(SelectionCornerRadius, radius.into());
                    }
                    if let StyleValue::Val(Some(color)) = selector_style.get_style_value(TextColor)
                    {
                        to_modify = to_modify.set_style_value(SelectionColor, color.into());
                    }
                    to_modify
                }
                None => to_modify.apply(selector_style),
            };
            map.insert(selector.selector, to_insert);
        }
    }
    {
        let elaps = std::time::SystemTime::now()
            .duration_since(now)
            .expect("Time is going backwards");
        if elaps.as_millis() == 0 {
            log::debug!("Styles parsed in {}μs", elaps.as_micros());
        } else {
            log::debug!("Styles parsed in {}ms", elaps.as_millis());
        }
    }
    map
}
