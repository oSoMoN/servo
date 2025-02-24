/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cell::Cell;

use dom_struct::dom_struct;
use servo_arc::Arc;
use style::shared_lock::SharedRwLock;
use style::stylesheets::Stylesheet as StyleStyleSheet;

use crate::dom::bindings::codegen::Bindings::CSSStyleSheetBinding::CSSStyleSheetMethods;
use crate::dom::bindings::error::{Error, ErrorResult, Fallible};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject};
use crate::dom::bindings::root::{DomRoot, MutNullableDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::cssrulelist::{CSSRuleList, RulesSource};
use crate::dom::element::Element;
use crate::dom::medialist::MediaList;
use crate::dom::node::{stylesheets_owner_from_node, Node};
use crate::dom::stylesheet::StyleSheet;
use crate::dom::window::Window;

#[dom_struct]
pub struct CSSStyleSheet {
    stylesheet: StyleSheet,
    owner: MutNullableDom<Element>,
    rulelist: MutNullableDom<CSSRuleList>,
    #[ignore_malloc_size_of = "Arc"]
    #[no_trace]
    style_stylesheet: Arc<StyleStyleSheet>,
    origin_clean: Cell<bool>,
}

impl CSSStyleSheet {
    fn new_inherited(
        owner: &Element,
        type_: DOMString,
        href: Option<DOMString>,
        title: Option<DOMString>,
        stylesheet: Arc<StyleStyleSheet>,
    ) -> CSSStyleSheet {
        CSSStyleSheet {
            stylesheet: StyleSheet::new_inherited(type_, href, title),
            owner: MutNullableDom::new(Some(owner)),
            rulelist: MutNullableDom::new(None),
            style_stylesheet: stylesheet,
            origin_clean: Cell::new(true),
        }
    }

    #[allow(crown::unrooted_must_root)]
    pub fn new(
        window: &Window,
        owner: &Element,
        type_: DOMString,
        href: Option<DOMString>,
        title: Option<DOMString>,
        stylesheet: Arc<StyleStyleSheet>,
    ) -> DomRoot<CSSStyleSheet> {
        reflect_dom_object(
            Box::new(CSSStyleSheet::new_inherited(
                owner, type_, href, title, stylesheet,
            )),
            window,
        )
    }

    fn rulelist(&self) -> DomRoot<CSSRuleList> {
        self.rulelist.or_init(|| {
            let rules = self.style_stylesheet.contents.rules.clone();
            CSSRuleList::new(self.global().as_window(), self, RulesSource::Rules(rules))
        })
    }

    pub fn disabled(&self) -> bool {
        self.style_stylesheet.disabled()
    }

    pub fn get_owner(&self) -> Option<DomRoot<Element>> {
        self.owner.get()
    }

    pub fn set_disabled(&self, disabled: bool) {
        if self.style_stylesheet.set_disabled(disabled) && self.get_owner().is_some() {
            stylesheets_owner_from_node(self.get_owner().unwrap().upcast::<Node>())
                .invalidate_stylesheets();
        }
    }

    pub fn set_owner(&self, value: Option<&Element>) {
        self.owner.set(value);
    }

    pub fn shared_lock(&self) -> &SharedRwLock {
        &self.style_stylesheet.shared_lock
    }

    pub fn style_stylesheet(&self) -> &StyleStyleSheet {
        &self.style_stylesheet
    }

    pub fn set_origin_clean(&self, origin_clean: bool) {
        self.origin_clean.set(origin_clean);
    }

    pub fn medialist(&self) -> DomRoot<MediaList> {
        MediaList::new(
            self.global().as_window(),
            self,
            self.style_stylesheet().media.clone(),
        )
    }
}

impl CSSStyleSheetMethods for CSSStyleSheet {
    // https://drafts.csswg.org/cssom/#dom-cssstylesheet-cssrules
    fn GetCssRules(&self) -> Fallible<DomRoot<CSSRuleList>> {
        if !self.origin_clean.get() {
            return Err(Error::Security);
        }
        Ok(self.rulelist())
    }

    // https://drafts.csswg.org/cssom/#dom-cssstylesheet-insertrule
    fn InsertRule(&self, rule: DOMString, index: u32) -> Fallible<u32> {
        if !self.origin_clean.get() {
            return Err(Error::Security);
        }
        self.rulelist()
            .insert_rule(&rule, index, /* nested */ false)
    }

    // https://drafts.csswg.org/cssom/#dom-cssstylesheet-deleterule
    fn DeleteRule(&self, index: u32) -> ErrorResult {
        if !self.origin_clean.get() {
            return Err(Error::Security);
        }
        self.rulelist().remove_rule(index)
    }
}
