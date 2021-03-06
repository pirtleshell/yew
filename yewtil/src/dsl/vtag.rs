use crate::dsl::BoxedVNodeProducer;
use yew::virtual_dom::vcomp::ScopeHolder;
use yew::virtual_dom::{Listener, VTag};
use yew::{Classes, Component};

pub struct Effect<T, COMP: Component>(Box<dyn FnOnce(T, &ScopeHolder<COMP>) -> T>);
impl<T, COMP: Component> Effect<T, COMP> {
    fn new(f: impl FnOnce(T, &ScopeHolder<COMP>) -> T + 'static) -> Self {
        Effect(Box::new(f))
    }
}

pub struct VTagProducer<COMP: Component> {
    tag_type: &'static str,
    effects: Vec<Effect<VTag<COMP>, COMP>>,
}

impl<COMP: Component> VTagProducer<COMP> {
    pub fn new(tag_type: &'static str) -> Self {
        VTagProducer {
            tag_type,
            effects: vec![],
        }
    }

    // TODO, consider making this T: Into<VNode> - The whole dsl doesn't need to be lazy. - although being generic over an additional argument that is either () OR Scope is problematic.
    pub fn child<T: Into<BoxedVNodeProducer<COMP>> + 'static>(mut self, child: T) -> Self {
        let effect = Effect::new(move |mut vtag: VTag<COMP>, scope: &ScopeHolder<COMP>| {
            let child = child.into().execute(scope);
            vtag.add_child(child);
            vtag
        });
        self.effects.push(effect);
        self
    }

    pub fn attribute(mut self, name: String, value: String) -> Self {
        let effect = Effect::new(move |mut vtag: VTag<COMP>, _scope: &ScopeHolder<COMP>| {
            vtag.add_attribute(&name, &value);
            vtag
        });
        self.effects.push(effect);
        self
    }

    pub fn listener(mut self, listener: Box<dyn Listener<COMP>>) -> Self {
        let effect = Effect::new(move |mut vtag: VTag<COMP>, _scope: &ScopeHolder<COMP>| {
            vtag.add_listener(listener);
            vtag
        });
        self.effects.push(effect);
        self
    }

    pub fn classes(mut self, classes: Classes) -> Self {
        let effect = Effect::new(move |mut vtag: VTag<COMP>, _scope: &ScopeHolder<COMP>| {
            vtag.set_classes(classes);
            vtag
        });
        self.effects.push(effect);
        self
    }
}

impl<COMP: Component> From<VTagProducer<COMP>> for BoxedVNodeProducer<COMP> {
    fn from(vtag_prod: VTagProducer<COMP>) -> Self {
        BoxedVNodeProducer::wrap(move |scope| {
            let mut vtag = VTag::new(vtag_prod.tag_type);
            for effect in vtag_prod.effects.into_iter() {
                vtag = (effect.0)(vtag, &scope)
            }
            vtag.into()
        })
    }
}
