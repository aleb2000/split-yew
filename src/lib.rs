#![doc = include_str!("../README.md")]
use std::fmt::Display;

use wasm_bindgen::{__rt::IntoJsResult, prelude::*};
use web_sys::{Element, HtmlElement};
use yew::prelude::*;

mod js;

#[derive(Clone, Properties, PartialEq)]
pub struct SplitProps {
    /// Classes to apply to the split container element
    #[prop_or_default]
    pub class: Classes,

    /// Initial sizes of each element
    pub sizes: Option<Vec<f64>>,

    /// Minimum size of all elements (if min_sizes is specified, this will be ignored)
    pub min_size: Option<f64>,

    /// Minimum size of each element
    pub min_sizes: Option<Vec<f64>>,

    /// Maximum size of all elements (if max_sizes is specified, this will be ignored)
    pub max_size: Option<f64>,

    /// Maximum size of each element
    pub max_sizes: Option<Vec<f64>>,

    /// Grow initial sizes to min_size (default: false)
    pub expand_to_min: Option<bool>,

    /// Gutter size in pixels (default: 10)
    pub gutter_size: Option<f64>,

    /// Gutter alignment between elements (default: GutterAlign::Center)
    pub gutter_align: Option<GutterAlign>,

    /// Snap to minimum size offset in pixels (default: 30)
    pub snap_offset: Option<f64>,

    /// Number of pixels to drag (default: 1)
    pub drag_interval: Option<f64>,

    /// Direction to split: horizontal or vertical (default: Direction::Horizontal)
    pub direction: Option<Direction>,

    /// Cursor to display while dragging (default: Cursor::ColResize)
    pub cursor: Option<Cursor>,

    /// Called to create each gutter element
    #[prop_or(
        Closure::<dyn Fn(js_sys::BigInt, String, Element) -> Element>::new(
            |_index, direction, _pair_element| {
                let gutter_element = web_sys::window()
                    .expect_throw("No window")
                    .document()
                    .expect_throw("No document")
                    .create_element("div")
                    .expect_throw("Failed to create gutter div");

                gutter_element.set_class_name(&format!("gutter gutter-{}", direction));
                js_sys::Reflect::set(
                    &gutter_element,
                    &"__isSplitGutter".into(),
                    &true.into(),
                )
                .expect_throw("Unable to set __isSplitGutter property");
                gutter_element
            },
        )
        .into_js_value()
        .into()
    )]
    pub gutter: js_sys::Function,

    /// Called to set the style of each element
    pub element_style: Option<js_sys::Function>,

    /// Called to set the style of the gutter
    pub gutter_style: Option<js_sys::Function>,

    /// Called on drag
    pub on_drag: Option<js_sys::Function>,

    /// Called on drag start
    pub on_drag_start: Option<js_sys::Function>,

    /// Called on drag end
    pub on_drag_end: Option<js_sys::Function>,

    pub collapsed: Option<usize>,

    pub children: Children,
}

pub struct Split {
    parent_ref: NodeRef,
    split: Option<js::Split>,
}

impl Component for Split {
    type Message = ();
    type Properties = SplitProps;

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let children = self
                .parent_ref
                .cast::<HtmlElement>()
                .unwrap_throw()
                .children();

            let children = js_sys::Array::from(&children);
            let options = ctx.props().make_options_object();
            self.split = Some(js::Split::new(children, options));

            if let Some(collapsed) = ctx.props().collapsed {
                self.split.as_ref().unwrap_throw().collapse(collapsed);
            }
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            parent_ref: NodeRef::default(),
            split: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let SplitProps {
            class, children, ..
        } = ctx.props();

        html! {
            <div class={(*class).clone()} ref={self.parent_ref.clone()}>
                { for children.iter() }
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let SplitProps {
            sizes,
            min_size,
            min_sizes,
            collapsed,
            ..
        } = ctx.props();

        let SplitProps {
            min_size: old_min_size,
            min_sizes: old_min_sizes,
            sizes: old_sizes,
            collapsed: old_collapsed,
            ..
        } = old_props;

        let mut needs_recreate = ctx.props().other_props_changed(old_props);

        if min_sizes.is_some() && old_min_sizes.is_some() {
            let mut min_size_changed = false;

            min_sizes
                .as_ref()
                .unwrap_throw()
                .iter()
                .enumerate()
                .for_each(|(i, min_size_i)| {
                    min_size_changed |= min_size_i
                        != old_min_sizes.as_ref().unwrap_throw().get(i).expect_throw(
                            "Cannot index min_sizes during update. Did the length change?",
                        );
                });

            needs_recreate |= min_size_changed;
        } else if min_sizes.is_some() || old_min_sizes.is_some() {
            needs_recreate = true;
        } else {
            needs_recreate |= min_size != old_min_size;
        }

        if needs_recreate {
            web_sys::console::log_1(&"Recreating split".into());
            let options = ctx.props().make_options_object();
            web_sys::console::log_1(&options);

            // This is done in the React version, not sure why exactly but I'm doing it as well
            let cur_sizes = js_sys::Reflect::get(&options, &"sizes".into()).unwrap_throw();
            if cur_sizes.is_falsy() {
                js_sys::Reflect::set(
                    &options,
                    &"sizes".into(),
                    &js::Split::get_sizes(self.split.as_ref().unwrap_throw()),
                )
                .unwrap_throw();
            }

            js::Split::destroy(self.split.as_ref().unwrap_throw(), true.into(), true.into());

            // The old gutter creates new div elements, here we just want to get the divs already
            // in the DOM and prepare them for the new split
            let new_gutter: js_sys::Function =
                Closure::<dyn Fn(js_sys::BigInt, String, Element) -> Element>::new(
                    |_index, direction, pair_element: Element| {
                        let gutter_el: Element = pair_element
                            .previous_sibling()
                            .map(|node| node.into_js_result().unwrap_throw())
                            .unwrap_or(JsValue::UNDEFINED)
                            .into();

                        if direction == "horizontal" {
                            gutter_el.set_class_name("gutter gutter-horizontal");
                        } else {
                            gutter_el.set_class_name("gutter gutter-vertical");
                        }

                        // We need to reset the styles otherwise the element will keep the
                        // width/height that was assigned by the previous split. No need to manually
                        // set the width/height ourselves as split.js will do that for us
                        gutter_el
                            .set_attribute("style", "")
                            .expect_throw("Cannot reset gutter style on recreate");

                        gutter_el
                    },
                )
                .into_js_value()
                .into();
            js_sys::Reflect::set(&options, &"gutter".into(), &new_gutter).unwrap_throw();

            let non_gutter_children = js_sys::Array::from(
                &self
                    .parent_ref
                    .cast::<HtmlElement>()
                    .expect_throw("No parent during update")
                    .children(),
            )
            .filter(&mut |element, _, _| {
                js_sys::Reflect::get(&element, &"__isSplitGutter".into())
                    .unwrap_throw()
                    .is_falsy()
            });

            web_sys::console::log_1(&"Non gutter children".into());
            web_sys::console::log_1(&non_gutter_children);

            self.split = Some(js::Split::new(non_gutter_children, options));
        } else if sizes.is_some() {
            let mut size_changed = false;

            sizes
                .as_ref()
                .unwrap_throw()
                .iter()
                .enumerate()
                .for_each(|(i, size_i)| {
                    size_changed |= size_i
                        != old_sizes.as_ref().unwrap_throw().get(i).expect_throw(
                            "Cannot index sizes during update. Did the length change?",
                        );
                });

            if size_changed {
                let new_sizes = js_sys::Array::new();
                for size in sizes.as_ref().unwrap_throw().iter() {
                    new_sizes.push(&JsValue::from(*size));
                }
                js::Split::set_sizes(self.split.as_ref().unwrap_throw(), new_sizes);
            }
        }

        if collapsed.is_some()
            && (old_collapsed.is_some() && collapsed.unwrap_throw() != old_collapsed.unwrap_throw()
                || needs_recreate)
        {
            js::Split::collapse(self.split.as_ref().unwrap_throw(), collapsed.unwrap_throw());
        }

        true
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        js::Split::destroy(
            self.split.as_ref().unwrap_throw(),
            false.into(),
            false.into(),
        );
        self.split = None;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GutterAlign {
    Start,
    End,
    Center,
}

impl Display for GutterAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GutterAlign::Start => write!(f, "start"),
            GutterAlign::End => write!(f, "end"),
            GutterAlign::Center => write!(f, "center"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Vertical,
    Horizontal,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Vertical => write!(f, "vertical"),
            Direction::Horizontal => write!(f, "horizontal"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cursor {
    ColResize,
    RowResize,
}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cursor::ColResize => write!(f, "col-resize"),
            Cursor::RowResize => write!(f, "row-resize"),
        }
    }
}

impl SplitProps {
    fn other_props_changed(&self, old_props: &SplitProps) -> bool {
        let SplitProps {
            max_size,
            expand_to_min,
            gutter_size,
            gutter_align,
            snap_offset,
            drag_interval,
            direction,
            cursor,
            ..
        } = self;

        let SplitProps {
            max_size: old_max_size,
            expand_to_min: old_expand_to_min,
            gutter_size: old_gutter_size,
            gutter_align: old_gutter_align,
            snap_offset: old_snap_offset,
            drag_interval: old_drag_interval,
            direction: old_direction,
            cursor: old_cursor,
            ..
        } = old_props;

        // TODO: remove
        if direction != old_direction {
            web_sys::console::log_1(&"Direction changed".into());
            web_sys::console::log_1(&direction.as_ref().unwrap().to_string().into());
        }

        max_size != old_max_size
            || expand_to_min != old_expand_to_min
            || gutter_size != old_gutter_size
            || gutter_align != old_gutter_align
            || snap_offset != old_snap_offset
            || drag_interval != old_drag_interval
            || direction != old_direction
            || cursor != old_cursor
    }

    fn set_option<T, F: Fn(&T) -> JsValue>(
        options: &js_sys::Object,
        key: &str,
        value: &Option<T>,
        f: F,
    ) {
        if value.is_some() {
            js_sys::Reflect::set(
                options,
                &key.into(),
                &value.as_ref().map_or(JsValue::UNDEFINED, f),
            )
            .unwrap_throw();
        }
    }

    fn make_options_object(&self) -> js_sys::Object {
        let SplitProps {
            sizes,
            min_size,
            min_sizes,
            max_size,
            max_sizes,
            expand_to_min,
            gutter_size,
            gutter_align,
            snap_offset,
            drag_interval,
            direction,
            cursor,
            gutter,
            element_style,
            gutter_style,
            on_drag,
            on_drag_start,
            on_drag_end,
            ..
        } = self;

        let options = js_sys::Object::new();
        let f64_vec_to_arr = |v: &Vec<f64>| {
            let arr = js_sys::Array::new();
            for val in v.iter() {
                arr.push(&JsValue::from(*val));
            }
            arr.into()
        };

        fn val_to_js<T: Into<JsValue> + Clone>(v: &T) -> JsValue {
            (*v).clone().into()
        }

        fn to_js_string<T: Display>(v: &T) -> JsValue {
            v.to_string().into()
        }

        Self::set_option(&options, "sizes", sizes, f64_vec_to_arr);

        if min_sizes.is_some() {
            Self::set_option(&options, "minSize", min_sizes, f64_vec_to_arr);
        } else if min_size.is_some() {
            Self::set_option(&options, "minSize", min_size, val_to_js);
        }

        if max_sizes.is_some() {
            Self::set_option(&options, "maxSize", max_sizes, f64_vec_to_arr);
        } else if max_size.is_some() {
            Self::set_option(&options, "maxSize", max_size, val_to_js);
        }

        Self::set_option(&options, "expandToMin", expand_to_min, val_to_js);
        Self::set_option(&options, "gutterSize", gutter_size, val_to_js);
        Self::set_option(&options, "gutterAlign", gutter_align, to_js_string);
        Self::set_option(&options, "snapOffset", snap_offset, val_to_js);
        Self::set_option(&options, "dragInterval", drag_interval, val_to_js);
        Self::set_option(&options, "direction", direction, to_js_string);
        Self::set_option(&options, "cursor", cursor, to_js_string);
        Self::set_option(&options, "gutter", &Some(gutter), val_to_js);
        Self::set_option(&options, "elementStyle", element_style, val_to_js);
        Self::set_option(&options, "gutterStyle", gutter_style, val_to_js);
        Self::set_option(&options, "onDrag", on_drag, val_to_js);
        Self::set_option(&options, "onDragStart", on_drag_start, val_to_js);
        Self::set_option(&options, "onDragEnd", on_drag_end, val_to_js);

        options
    }
}
