/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::HTMLTableColElementBinding;
use dom::bindings::utils::{DOMString, ErrorResult};
use dom::document::AbstractDocument;
use dom::element::HTMLTableColElementTypeId;
use dom::htmlelement::HTMLElement;
use dom::node::{AbstractNode, Node, ScriptView};

pub struct HTMLTableColElement {
    htmlelement: HTMLElement,
}

impl HTMLTableColElement {
    pub fn new_inherited(localName: ~str, document: AbstractDocument) -> HTMLTableColElement {
        HTMLTableColElement {
            htmlelement: HTMLElement::new_inherited(HTMLTableColElementTypeId, localName, document)
        }
    }

    pub fn new(localName: ~str, document: AbstractDocument) -> AbstractNode<ScriptView> {
        let element = HTMLTableColElement::new_inherited(localName, document);
        Node::reflect_node(@mut element, document, HTMLTableColElementBinding::Wrap)
    }
}

impl HTMLTableColElement {
    pub fn Span(&self) -> u32 {
        0
    }

    pub fn SetSpan(&mut self, _span: u32) -> ErrorResult {
        Ok(())
    }

    pub fn Align(&self) -> DOMString {
        ~""
    }

    pub fn SetAlign(&mut self, _align: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Ch(&self) -> DOMString {
        ~""
    }

    pub fn SetCh(&mut self, _ch: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn ChOff(&self) -> DOMString {
        ~""
    }

    pub fn SetChOff(&mut self, _ch_off: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn VAlign(&self) -> DOMString {
        ~""
    }

    pub fn SetVAlign(&mut self, _v_align: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Width(&self) -> DOMString {
        ~""
    }

    pub fn SetWidth(&mut self, _width: DOMString) -> ErrorResult {
        Ok(())
    }
}
