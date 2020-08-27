use crate::style::{ClassStyle, InlineStyle, StyleBuilder};
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
    segments: Vec<(ClassStyle, String)>,
    _builder: PhantomData<B>,
}
impl<B: StyleBuilder> Ansi<B> {
    fn update_segments(&mut self) {
        let s = &self.props.text;
        self.segments.clear();

        for (effect, content) in crate::get_sgr_segments(s) {
            self.segments
                .push((effect.to_class_style::<B>(), content.to_owned()))
        }
    }

    fn render_segment((class_style, content): &(ClassStyle, String)) -> Html {
        // TODO update to use optional attributes when they land
        let class = class_style.class.clone().unwrap_or_default();
        let style = class_style.style.clone().unwrap_or_default();
        html! {
            <span class=class style=style>
                { content }
            </span>
        }
    }
}
impl<B: StyleBuilder + 'static> Component for Ansi<B> {
    type Message = ();
    type Properties = AnsiProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut instance = Self {
            props,
            segments: Vec::new(),
            _builder: PhantomData::default(),
        };
        instance.update_segments();
        instance
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let update_segments = self.props.text != props.text;

        let should_render = if self.props != props {
            self.props = props;
            true
        } else {
            false
        };

        if update_segments {
            self.update_segments();
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
                { for self.segments.iter().map(Self::render_segment) }
            </pre>
        }
    }
}
