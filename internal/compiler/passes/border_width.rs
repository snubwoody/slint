use crate::diagnostics::BuildDiagnostics;
use crate::expression_tree::{Expression, NamedReference};
use crate::object_tree::Component;
use smol_str::SmolStr;
use std::rc::Rc;

pub const BORDER_WIDTH_PROPERTIES: [&str; 4] =
    ["border-top-width", "border-right-width", "border-bottom-width", "border-left-width"];

// DOC: might need to remove the old border width
pub fn handle_border_width(root_component: &Rc<Component>, _diagnostics: &mut BuildDiagnostics) {
    crate::object_tree::recurse_elem_including_sub_components_no_borrow(
        root_component,
        &(),
        &mut |elem, _| {
            let bty = if let Some(bty) = elem.borrow().builtin_type() { bty } else { return };
            if bty.name == "Rectangle"
                && elem.borrow().is_binding_set("border-width", true)
                && BORDER_WIDTH_PROPERTIES
                    .iter()
                    .any(|property_name| elem.borrow().is_binding_set(property_name, true))
            {
                let border_width = NamedReference::new(elem, SmolStr::new_static("border-width"));
                for property_name in BORDER_WIDTH_PROPERTIES.iter() {
                    elem.borrow_mut().set_binding_if_not_set(SmolStr::new(property_name), || {
                        Expression::PropertyReference(border_width.clone())
                    });
                }
            }
        },
    )
}
