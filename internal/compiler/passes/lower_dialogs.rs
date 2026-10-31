// Copyright © Wakunguma Kalimukwa <contact@wakunguma.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

use crate::diagnostics::BuildDiagnostics;
use crate::langtype::ElementType;
use crate::object_tree::*;
use crate::typeregister::TypeRegister;
use std::cell::RefCell;
use std::rc::Rc;

pub fn lower_dialogs(
    component: &Rc<Component>,
    type_register: &TypeRegister,
    diag: &mut BuildDiagnostics,
) {
    recurse_elem_including_sub_components_no_borrow(
        component,
        &None,
        &mut |elem, parent_element: &Option<ElementRc>| {
            if is_dialog_window(elem) {
                lower_dialog_window(elem, parent_element.as_ref(), diag);
            }
            Some(elem.clone())
        },
    )
}

fn is_dialog_window(element: &ElementRc) -> bool {
    match &element.borrow().base_type {
        ElementType::Builtin(base_type) => base_type.name == "DialogWindow",
        // TODO: add support for inheriting DialogWindow
        // ElementType::Component(base_type) => base_type.inherits_popup_window.get(),
        _ => false,
    }
}

fn lower_dialog_window(
    dialog_window_element: &ElementRc,
    parent_element: Option<&ElementRc>,
    diag: &mut BuildDiagnostics,
) {
    let parent_component = dialog_window_element.borrow().enclosing_component.upgrade().unwrap();
    if Rc::ptr_eq(&parent_component.root_element, dialog_window_element) {
        diag.push_error(
            "DialogWindow cannot be directly repeated or conditional".into(),
            &*dialog_window_element.borrow(),
        );
        return;
    }

    let parent_element = match parent_element {
        Some(parent_element) => parent_element,
        None => return,
    };

    // Remove the popup_window_element from its parent
    let mut parent_element_borrowed = parent_element.borrow_mut();
    let index = parent_element_borrowed
        .children
        .iter()
        .position(|child| Rc::ptr_eq(child, dialog_window_element))
        .expect("DialogWindow must be a child of its parent");
    parent_element_borrowed.children.remove(index);
    drop(parent_element_borrowed);
    for parent_cip in parent_component.child_insertion_points.borrow_mut().values_mut() {
        if Rc::ptr_eq(&parent_cip.parent, parent_element) && parent_cip.insertion_index > index {
            parent_cip.insertion_index -= 1;
        }
    }

    let dialog_component = Rc::new(Component {
        root_element: dialog_window_element.clone(),
        parent_element: RefCell::new(Rc::downgrade(parent_element)),
        ..Default::default()
    });

    let weak = Rc::downgrade(&dialog_component);
    recurse_elem(&dialog_component.root_element, &(), &mut |e, _| {
        e.borrow_mut().enclosing_component = weak.clone()
    });

    super::focus_handling::call_focus_on_init(&dialog_component);

    parent_component
        .dialog_windows
        .borrow_mut()
        .push(DialogWindow { component: dialog_component, parent_element: parent_element.clone() });
}
