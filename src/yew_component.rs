use crate::{
    graphic_rendition::SgrEffect,
    sequences::{self, Csi, Escape, Marker},
    style::{ClassStyle, InlineStyle, StyleBuilder},
};
use std::marker::PhantomData;
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

const CSS_ANSI_CONTAINER: &str = "font-family:monospace;";

/// Props that can be passed to the [`Ansi`] component.
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AnsiProps {
    /// Classes to add to the root element. (Optional)
    #[prop_or_default]
    pub class: Classes,
    /// Content to render. (Required)
    pub text: String,
    /// Whether to disable the inline style applied to the root component. (Optional)
    #[prop_or_default]
    pub no_default_style: bool,
}

/// Component for rendering text containing ANSI escape codes.
///
/// By default the component uses [`InlineStyle`] to build the style for each part.
/// You can pass your own [`StyleBuilder`] like `Ansi<MyBuilder>`.
///
/// See [`AnsiProps`] for the props that can be passed to this component.
#[derive(Debug)]
pub struct Ansi<B: StyleBuilder = InlineStyle> {
    props: AnsiProps,
    parts: Vec<Part>,
    _builder: PhantomData<B>,
}
impl<B: StyleBuilder> Ansi<B> {
    fn update_parts(&mut self) {
        let s = &self.props.text;
        self.parts.clear();
        let mut effect = SgrEffect::default();
        for marker in sequences::get_markers(s) {
            match marker {
                Marker::Text(content) => {
                    let part = Part {
                        content: content.to_owned(),
                        style: effect.to_class_style::<B>(),
                    };
                    self.parts.push(part);
                }
                Marker::Sequence(Escape::Csi(Csi::Sgr(sgrs))) => {
                    effect.apply_sgrs(sgrs);
                }
            }
        }
    }
}
impl<B: StyleBuilder + 'static> Component for Ansi<B> {
    type Message = ();
    type Properties = AnsiProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut instance = Self {
            props,
            parts: Vec::new(),
            _builder: PhantomData::default(),
        };
        instance.update_parts();
        instance
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let update_parts = self.props.text != props.text;

        let should_render = if self.props != props {
            self.props = props;
            true
        } else {
            false
        };

        if update_parts {
            self.update_parts();
        }

        should_render
    }

    fn view(&self) -> Html {
        let props = &self.props;
        let style = if props.no_default_style {
            ""
        } else {
            CSS_ANSI_CONTAINER
        };
        html! {
            <pre class=props.class.clone() style=style>
                { for self.parts.iter().map(Part::render) }
            </pre>
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct Part {
    content: String,
    style: ClassStyle,
}
impl Part {
    fn render(&self) -> Html {
        let Self {
            content,
            style: ClassStyle { class, style },
        } = self;

        // TODO update to use optional attributes when they land
        let class = class.clone().unwrap_or_default();
        let style = style.clone().unwrap_or_default();
        html! {
            <span class=class style=style>
                { content }
            </span>
        }
    }
}
