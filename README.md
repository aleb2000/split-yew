# split-yew

[![crates-badge]][crates-url]
[![docsrs-badge]][docsrs-url]
[![yew-badge]][yew-url]
[![license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/split-yew
[crates-url]: https://crates.io/crates/split-yew
[docsrs-badge]: https://img.shields.io/docsrs/split-yew
[docsrs-url]: https://docs.rs/split-yew/
[license-badge]: https://img.shields.io/github/license/aleb2000/split-yew
[license-url]: https://github.com/aleb2000/split-yew/blob/main/LICENSE-MIT
[yew-badge]: https://img.shields.io/badge/yew-0.21-009a5b
[yew-url]: https://crates.io/crates/yew/0.21.0


This library adds a Yew component wrapper for the [Split.js library](https://split.js.org/). Similar to the [react-split](https://github.com/nathancahill/split/tree/master/packages/react-split) component.

This library does NOT include a component for [split-grid](https://github.com/nathancahill/split/tree/master/packages/split-grid).

split-yew has been tested with split.js version *1.6.5*

## Usage

Start by including the split.js module in your index.html file header tag like so:

```html
<script type="importmap">
    {
        "imports": {
            "split.js": "https://unpkg.com/split.js@1.6.5/dist/split.es.js"
        }
    }
</script>
```

Make sure it is of type `importmap` and that the import is called `split.js`. Also make sure you choose the ES module version and not the CommonJS one. It is worth noting that split.js does not include a minified version of the ES module script, hence if you want one you will have to package it yourself. After that the library should work without issues.

Much like its React counterpart, you just have to surround the components you want to be in a resizable split view.

```rust
html! {
    <Split>
        <Component />
        // ...
        <Component />
    </Split>
}
```

### CSS

split.js does not include any styling and this component does not change that.
To make the split visible you should style the split root element's display layout. I personally recommend a flex layout.
To make the gutter visible you should edit its style using the `.gutter`, `.gutter-horizontal`, and `.gutter-vertical` CSS classes.

A real life example usage of split-yew is the [yew-playground](https://github.com/yewstack/yew-playground/)'s frontend.
The split style is set using TailwindCSS in the [app's source](https://github.com/yewstack/yew-playground/blob/47a481cd869efb1d521bb33a11b20f319ee3d751/frontend/src/app.rs#L93), and its [CSS stylesheet](https://github.com/yewstack/yew-playground/blob/47a481cd869efb1d521bb33a11b20f319ee3d751/frontend/styles/globals.scss#L12) shows how to style the gutter.

For more information on styling checkout [split.js's documentation](https://github.com/nathancahill/split/tree/master/packages/splitjs#css).

## Documentation

You can customize the split using props. I tried to keep compatibility with the react-split component, hence you can also mostly refer to [its reference](https://github.com/nathancahill/split/tree/master/packages/react-split#reference) as well as [Split.js's documentation](https://github.com/nathancahill/split/tree/master/packages/splitjs#documentation) with some minor changes to the API.

### All props

| Prop            | Type               | Description                                                                    | Docs                                                                                                    |
|-----------------|--------------------|--------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------|
| `class`         | `Classes`          | Classes to apply to the split container element                                | N/A                                                                                                     |
| `sizes`         | `Vec<f64>`         | Initial sizes of each element                                                  | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#sizes)                        |
| `min_size`      | `f64`              | Minimum size of all elements (if min_sizes is specified, this will be ignored) | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#minsize-default-100)         |
| `min_sizes`     | `Vec<f64>`         | Minimum size of each element                                                   | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#minsize-default-100)         |
| `max_size`      | `f64`              | Maximum size of all elements (if max_sizes is specified, this will be ignored) | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#maxsize-default-infinity)     |
| `max_sizes`     | `Vec<f64>`         | Maximum size of each element                                                   | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#maxsize-default-infinity)     |
| `expand_to_min` | `bool`             | Grow initial sizes to min_size (default: false)                                | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#expandtomin-default-false)   |
| `gutter_size`   | `f64`              | Gutter size in pixels (default: 10)                                            | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#gutterSize)                  |
| `gutter_align`  | `GutterAlign`      | Gutter alignment between elements (default: GutterAlign::Center)               | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#gutteralign-default-center)   |
| `snap_offset`   | `f64`              | Snap to minimum size offset in pixels (default: 30)                            | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#snapoffset-default-30)        |
| `drag_interval` | `f64`              | Number of pixels to drag (default: 1)                                          | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#draginterval-default-1)       |
| `direction`     | `Direction`        | Direction to split: horizontal or vertical (default: Direction::Horizontal)    | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#direction-default-horizontal) |
| `cursor`        | `Cursor`           | Cursor to display while dragging (default: Cursor::ColResize)                  | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#cursor-default-col-resize)    |
| `gutter`        | `js_sys::Function` | Called to create each gutter element                                           | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#gutter)                       |
| `element_style` | `js_sys::Function` | Called to set the style of each element                                        | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#elementstyle)                 |
| `gutter_style`  | `js_sys::Function` | Called to set the style of the gutter                                          | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#gutterstyle)                  |
| `on_drag`       | `js_sys::Function` | Called on drag                                                                 | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#ondrag-ondragstart-ondragend) |
| `on_drag_start` | `js_sys::Function` | Called on drag start                                                           | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#ondrag-ondragstart-ondragend) |
| `on_drag_end`   | `js_sys::Function` | Called on drag end                                                             | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#ondrag-ondragstart-ondragend) |
| `collapsed`     | `usize`            | This prop replaces the method call to `collapse(index)`                        | [docs](https://github.com/nathancahill/split/tree/master/packages/splitjs#collapseindex)                |


### Additional Details

#### `class` prop

The Split component includes a `class` prop you can use to specify classes for the div that wraps all of the inner components. This can be useful to style the split, especially when using something like TailwindCSS.

#### `min_size` and `max_size` props

These two props originally accept two possible different types, either a single number specifying the min/max size for all components, or an array of numbers, specifying min/max sizes for each component individually.

To emulate this behavior in Rust, split-yew has four different props:

- `min_size`/`max_size`: where you can specify a single value to apply to all components.
- `min_sizes`/`max_sizes`: where you can specify a vector of values, one for each component.

While you can specify, for instance, both a `min_size` and a `min_sizes` at the same time; the vector variant will always take priority, as shown in the following example:

```rust
html! {
    <Split min_size={500.0} min_sizes={vec![100.0, 200.0]}>
        <ComponentA />
        <ComponentB />
    </Split>
}
```

In this example the two components will have a min size of 100 and 200 respectively, while the `min_size={500.0}` will be ignored.

#### Function props

Props `gutter`, `element_style`, `gutter_style`, `on_drag`, `on_drag_start`, and `on_drag_end` are supposed to accept a function or closure. Unfortunately I was not able to represent them with a `yew::Callback` or `wasm_bindgen::Closure` type. The best thing I was able to do at the moment was use a `js_sys::Function` type, which can still be passed as prop, as well as to the split.js library itself.

This makes passing these props a bit inconvenient, as the `js_sys::Function` does not include any type information about function arguments or return type, however all that information is already available in the official split.js docs, so refer to that if you need to implement one of these functions.

For the implementation itself, you can simply implement a `wasm_bindgen::Closure` and convert it to a `js_sys::Function`. The following example shows how to create such a function for the `gutter` prop.

```rust
let my_gutter: js_sys::Function = Closure::<dyn Fn(js_sys::BigInt, String, web_sys::Element) -> web_sys::Element>::new(
        |index, direction, pair_element| {
            // Do something with the arugments and return a value of type web_sys::Element
        },
    )
    .into_js_value()
    .into()
```

As shown in the example, the final type of `my_gutter` will be a `js_sys::Function`. While I don't have too much experience with passing functions from Rust to JavaScript, one caveat I did find is that with numbers I could not use a normal Rust primitive, but use `js_sys::BigInt` instead.

## License

This project follows the [MIT license](./LICENSE-MIT).
