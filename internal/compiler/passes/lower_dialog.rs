// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

//! This pass transforms the DialogWindow element into a component

use std::{cell::RefCell, rc::Rc};

use crate::{
    diagnostics::BuildDiagnostics, langtype::ElementType, object_tree::*,
    typeregister::TypeRegister,
};

pub fn lower_dialogs(
    component: &Rc<Component>,
    type_register: &TypeRegister,
    diag: &mut BuildDiagnostics,
) {
    recurse_elem_including_sub_components_no_borrow(
        component,
        &None,
        &mut |elem, parent_element: &Option<ElementRc>| {
            lower_dialog_window(elem, parent_element.as_ref(), diag);
            Some(elem.clone())
        },
    )
}

fn lower_dialog_window(
    dialog_window_element: &ElementRc,
    parent_element: Option<&ElementRc>,
    diag: &mut BuildDiagnostics,
) {
    let parent_component = dialog_window_element.borrow().enclosing_component.upgrade().unwrap();
    let parent_element = match parent_element {
        None => {
            // DOC: FIXME
            // if matches!(dialog_window_element.borrow().base_type, ElementType::Builtin(_)) {
            //     dialog_window_element.borrow_mut().base_type = window_type.clone();
            // }
            parent_component.inherits_popup_window.set(true);
            return;
        }
        Some(parent_element) => parent_element,
    };

    if Rc::ptr_eq(&parent_component.root_element, dialog_window_element) {
        diag.push_error(
            "PopupWindow cannot be directly repeated or conditional".into(),
            &*dialog_window_element.borrow(),
        );
        return;
    }

    // DOC: might need this
    // Remove the popup_window_element from its parent
    // let mut parent_element_borrowed = parent_element.borrow_mut();
    // let index = parent_element_borrowed
    //     .children
    //     .iter()
    //     .position(|child| Rc::ptr_eq(child, popup_window_element))
    //     .expect("PopupWindow must be a child of its parent");
    // parent_element_borrowed.children.remove(index);
    // parent_element_borrowed.has_popup_child = true;
    // drop(parent_element_borrowed);

    // for parent_cip in parent_component.child_insertion_points.borrow_mut().values_mut() {
    //     if Rc::ptr_eq(&parent_cip.parent, parent_element) && parent_cip.insertion_index > index {
    //         parent_cip.insertion_index -= 1;
    //     }
    // }

    let dialog_component = Rc::new(Component {
        root_element: dialog_window_element.clone(),
        parent_element: RefCell::new(Rc::downgrade(parent_element)),
        ..Component::default()
    });

    super::focus_handling::call_focus_on_init(&dialog_component);

    parent_component
        .dialog_windows
        .borrow_mut()
        .push(DialogWindow { component: dialog_component, parent_element: parent_element.clone() });
}
