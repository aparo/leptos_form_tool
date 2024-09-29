use super::FormStyle;
use crate::controls::{
    button::ButtonData, checkbox::CheckboxData, date::DateData, heading::HeadingData,
    hidden::HiddenData, output::OutputData, radio_buttons::RadioButtonsData, select::SelectData,
    slider::SliderData, spacer::SpacerData, stepper::StepperData, submit::SubmitData,
    text_area::TextAreaData, text_input::TextInputData, ControlRenderData, UpdateEvent,
    ValidationState,
};
use leptos::*;
use std::rc::Rc;
use web_sys::MouseEvent;

/// Styling attributes for the [`FbFormStyle`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FbStyleAttr {
    /// Set the width of the control out of 12.
    /// Defaults to 12/12 (full width).
    Width(u32),
    /// Adds a tooltip to the control.
    /// This sets the html title attribute, which shows the text when the
    /// user hovers their mouse over the control for a couple seconds.
    Tooltip(String),
}

/// A complete useable example for defining a form style.
///
/// This can be used directly in by your form, or you can copy `grid_form.rs`
/// into your project and make any neccesary change. You will also want to
/// copy `grid_form.scss` from the git repo and put that in the `styles`
/// directory for your leptos project to get all the styling.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FbFormStyle;

impl FbFormStyle {
    fn input_class() -> &'static str {
        "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
    }

    fn input_class_error() -> &'static str {
        "bg-red-50 border border-red-500 text-red-900 placeholder-red-700 text-sm rounded-lg focus:ring-red-500 dark:bg-gray-700 focus:border-red-500 block w-full p-2.5 dark:text-red-500 dark:placeholder-red-500 dark:border-red-500"
    }

    fn text_class() -> &'static str {
        "block w-full p-4 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 text-base focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
    }

    fn text_class_error() -> &'static str {
        "block w-full p-4 bg-red-50 border border-red-500 text-red-900 placeholder-red-700 text-base rounded-lg  focus:ring-red-500 focus:border-red-500 dark:bg-gray-700 dark:border-red-600 dark:placeholder-red-500 dark:text-red-500 dark:focus:ring-red-500 dark:focus:border-red-500"
    }

    fn radio_class() -> &'static str {
        "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
    }

    fn radio_class_error() -> &'static str {
        "w-4 h-4 text-red-900 bg-gray-100 border-red-500 focus:ring-red-500 dark:focus:ring-red-600 dark:ring-offset-red-800 focus:ring-2 dark:bg-red-700 dark:border-red-600"
    }

    fn label_class() -> &'static str {
        "block mb-2 text-sm font-medium text-gray-900 dark:text-white"
    }

    fn label_class_error() -> &'static str {
        "block mb-2 text-sm font-medium text-red-700 dark:text-red-500"
    }

    fn class_error_message() -> &'static str {
        "mt-2 text-sm text-red-600 dark:text-red-500"
    }

    fn common_component(
        &self,
        styles: &[<FbFormStyle as FormStyle>::StylingAttributes],
        parent_class: &'static str,
        inner: View,
    ) -> View {
        let mut width = 12;
        let mut tooltip = None;
        for style in styles.iter() {
            match style {
                FbStyleAttr::Width(w) => width = *w,
                FbStyleAttr::Tooltip(t) => tooltip = Some(t),
            }
        }

        view! {
            <div class=parent_class style:grid-column=format!("span {}", width) title=tooltip>
                {inner}
            </div>
        }
        .into_view()
    }
}
impl FormStyle for FbFormStyle {
    type StylingAttributes = FbStyleAttr;

    fn form_frame(&self, form: ControlRenderData<Self, View>) -> View {
        view! { <div class="grid grid-cols-12 gap-4">{form.data}</div> }.into_view()
    }

    /// A common function that wraps the given view in the styles
    fn custom_component(&self, styles: &[Self::StylingAttributes], inner: View) -> View {
        self.common_component(styles, "custom_component_parent", inner)
    }

    fn group(&self, group: Rc<ControlRenderData<Self, View>>) -> View {
        let view = view! { <div class="form_group form_grid">{&group.data}</div> }.into_view();

        self.common_component(&group.styles, "group_parent", view)
    }

    fn spacer(&self, control: Rc<ControlRenderData<Self, SpacerData>>) -> View {
        self.common_component(
            &control.styles,
            "spacer_parent",
            view! { <div style:height=control.data.height.as_ref()></div> }.into_view(),
        )
    }

    fn heading(
        &self,
        control: Rc<ControlRenderData<Self, HeadingData>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        use crate::controls::heading::HeadingLevel::*;

        let title = move || value_getter.map(|v| v.get()).unwrap_or_default();

        let view = match control.data.level {
            H1 => view! { <h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white"> {title} </h1> }.into_view(),
            H2 => view! { <h2 class="text-4xl font-extrabold dark:text-white"> {title} </h2> }.into_view(),
            H3 => view! { <h3 class="text-3xl font-extrabold dark:text-white"> {title} </h3> }.into_view(),
            H4 => view! { <h4 class="text-2xl font-extrabold dark:text-white"> {title} </h4> }.into_view(),
        };

        self.common_component(&control.styles, "flew flex-row", view)
    }

    fn submit(
        &self,
        control: Rc<ControlRenderData<Self, SubmitData>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        let title = move || value_getter.map(|v| v.get()).unwrap_or_default();

        self.common_component(
            &control.styles,
            "",
            view! { <input type="submit" value=title class="flex items-center bg-sky-500 hover:bg-sky-700 px-5 py-1 text-sm rounded-full font-semibold text-white" /> }.into_view(),
        )
    }

    fn button(
        &self,
        control: Rc<ControlRenderData<Self, ButtonData>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        let action = control.data.action.clone();
        let on_click = move |ev: MouseEvent| {
            if let Some(ref action) = action {
                action(ev)
            }
        };

        let title = move || value_getter.map(|v| v.get()).unwrap_or_default();

        let view = view! {
            <button type="button" class="flex items-center bg-sky-500 hover:bg-sky-700 px-5 py-1 text-sm rounded-full font-semibold text-white" on:click=on_click>
                {title}
            </button>
        }
        .into_view();

        self.common_component(&control.styles, "", view)
    }

    fn output(
        &self,
        control: Rc<ControlRenderData<Self, OutputData>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        let view = view! { <span>{move || value_getter.map(|g| g.get())}</span> }.into_view();
        self.common_component(&control.styles, "output_parent", view)
    }

    fn hidden(
        &self,
        control: Rc<ControlRenderData<Self, HiddenData>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        let value_getter = move || value_getter.map(|g| g.get());
        view! {
            <input
                name=&control.data.name
                prop:value=value_getter
                style="visibility: hidden; position: absolute;"
            />
        }
        .into_view()
    }

    fn text_input(
        &self,
        control: Rc<ControlRenderData<Self, TextInputData>>,
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let input_class = move || {
            if validation_state.get().is_err() {
                Self::input_class_error()
            } else {
                Self::input_class()
            }
        };

        let input = view! {
            <input
                type=control.data.input_type
                id=&control.data.name
                name=&control.data.name
                placeholder=control.data.placeholder.as_ref()
                class=input_class
                // class=input_class
                // class=("form_input_invalid", move || validation_state.get().is_err())
                prop:value=move || value_getter.get()
            />
        };

        let input = match control.data.update_event {
            UpdateEvent::OnFocusout => input.on(ev::focusout, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
            UpdateEvent::OnInput => input.on(ev::input, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
            UpdateEvent::OnChange => input.on(ev::change, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
        };

        let label_class = move || {
            if validation_state.get().is_err() {
                Self::label_class_error()
            } else {
                Self::label_class()
            }
        };

        let class_error_message = Self::class_error_message();

        let view = view! {
            <label for=&control.data.name class=label_class>
                {control.data.label.as_ref()}
            </label>
            {input}
            <Show when=move || validation_state.get().is_err()>
                <p class=class_error_message>{move || validation_state.get().take_msg()}</p>
            </Show>
        }
        .into_view();

        self.common_component(&control.styles, "", view)
    }

    fn text_area(
        &self,
        control: Rc<ControlRenderData<Self, TextAreaData>>,
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let input_class = move || {
            if validation_state.get().is_err() {
                Self::text_class_error()
            } else {
                Self::text_class()
            }
        };

        let input = view! {
            <textarea
                id=&control.data.name
                name=&control.data.name
                placeholder=control.data.placeholder.as_ref()
                prop:value=move || value_getter.get()
                style="resize: vertical;"
                class=input_class
            ></textarea>
        };

        let input = match control.data.update_event {
            UpdateEvent::OnFocusout => input.on(ev::focusout, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
            UpdateEvent::OnInput => input.on(ev::input, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
            UpdateEvent::OnChange => input.on(ev::change, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
        };

        let label_class = move || {
            if validation_state.get().is_err() {
                Self::label_class_error()
            } else {
                Self::label_class()
            }
        };

        let class_error_message = Self::class_error_message();

        let view = view! {
            <label for=&control.data.name class=label_class>
                {control.data.label.as_ref()}
            </label>
            {input}
            <Show when=move || validation_state.get().is_err()>
                <p class=class_error_message>{move || validation_state.get().take_msg()}</p>
            </Show>
        }
        .into_view();

        self.common_component(&control.styles, "", view)
    }

    fn radio_buttons(
        &self,
        control: Rc<ControlRenderData<Self, RadioButtonsData>>,
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let input_class = move || {
            if validation_state.get().is_err() {
                Self::radio_class_error()
            } else {
                Self::radio_class()
            }
        };
        let buttons_view = control
            .data
            .options
            .iter()
            .map(|(display, value)| {
                let display = display.clone();
                let value = value.clone();
                let value_clone = value.clone();
                let value_clone2 = value.clone();
                view! {
                    <div class="flex items-center mb-4">
                        <input
                            type="radio"
                            id=&value
                            name=&control.data.name
                            value=&value
                            class=input_class
                            prop:checked=move || { value_getter.get() == value_clone }
                            on:input=move |ev| {
                                let new_value = event_target_checked(&ev);
                                if new_value {
                                    value_setter.set(value_clone2.clone());
                                }
                            }
                        />

                        <label for=&value class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300">{display}</label>
                    </div>
                }
            })
            .collect_view();

        let label_class = move || {
            if validation_state.get().is_err() {
                Self::label_class_error()
            } else {
                Self::label_class()
            }
        };

        let class_error_message = Self::class_error_message();

        let view = view! {
            <label for=&control.data.name class=label_class>
                {control.data.label.as_ref()}
            </label>
            <div
                class="flex flex-col"
                class:form_input_invalid=move || validation_state.get().is_err()
            >
                {buttons_view}
            </div>
            <Show when=move || validation_state.get().is_err()>
                <p class=class_error_message>{move || validation_state.get().take_msg()}</p>
            </Show>
        }
        .into_view();

        self.common_component(&control.styles, "", view)
    }

    fn select(
        &self,
        control: Rc<ControlRenderData<Self, SelectData>>,
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let control_clone = control.clone();
        let options_view = move || {
            control_clone
            .data
            .options
            .get()
            .iter()
            .map(|(display, value)| {
                let display = display.clone();
                let value = value.clone();
                view! {
                    <option value=value.clone() selected=move || { value_getter.get() == *value }>
                        {display}
                    </option>
                }
            })
            .collect_view()
        };

        let blank_option_view = control.data.blank_option.as_ref().map(|display| {
            view! {
                <option value="" selected=move || { value_getter.get().as_str() == "" }>
                    {display}
                </option>
            }
        });

        let class_error_message = Self::class_error_message();
        let label_class = move || {
            if validation_state.get().is_err() {
                Self::label_class_error()
            } else {
                Self::label_class()
            }
        };
        let input_class = move || {
            if validation_state.get().is_err() {
                Self::input_class_error()
            } else {
                Self::input_class()
            }
        };

        let view = view! {
            <label for=&control.data.name class=label_class>
                {control.data.label.as_ref()}
            </label>
            <select
                id=&control.data.name
                name=&control.data.name
                class=input_class
                on:input=move |ev| {
                    value_setter.set(event_target_value(&ev));
                }
            >
                {blank_option_view}
                {options_view}
            </select>
            <Show when=move || validation_state.get().is_err()>
                <p class=class_error_message>{move || validation_state.get().take_msg()}</p>
            </Show>
        }
        .into_view();

        self.common_component(&control.styles, "", view)
    }

    fn checkbox(
        &self,
        control: Rc<ControlRenderData<Self, CheckboxData>>,
        value_getter: Signal<bool>,
        value_setter: SignalSetter<bool>,
    ) -> View {
        let label = control
            .data
            .label
            .clone()
            .unwrap_or(control.data.name.clone());

        let view = view! {
            <div class="flex items-center mb-4">
                <input
                    type="checkbox"
                    id=&control.data.name
                    name=&control.data.name
                    style="margin: auto 0;"
                    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                    prop:checked=value_getter
                    on:input=move |ev| {
                        let new_value = event_target_checked(&ev);
                        value_setter.set(new_value);
                    }
                />
                <label
                    for=&control.data.name
                    class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                    // class=("form_checkbox_checked", move || value_getter.get())
                    // class=("form_checkbox_unchecked", move || !value_getter.get())
                >
                    {label}
                </label>
            </div>
        }
        .into_view();

        self.common_component(&control.styles, "", view)
    }

    fn stepper(
        &self,
        control: Rc<ControlRenderData<Self, StepperData>>,
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let label_class = move || {
            if validation_state.get().is_err() {
                Self::label_class_error()
            } else {
                Self::label_class()
            }
        };

        let class_error_message = Self::class_error_message();
        let input_class = move || {
            if validation_state.get().is_err() {
                Self::input_class_error()
            } else {
                Self::input_class()
            }
        };

        let view = view! {
            <label for=&control.data.name class=label_class>
                {control.data.label.as_ref()}
            </label>
            <input
                type="number"
                id=&control.data.name
                name=&control.data.name
                step=control.data.step.clone()
                min=control.data.min.clone()
                max=control.data.max.clone()
                class=input_class
                prop:value=move || value_getter.get()
                on:input=move |ev| {
                    value_setter.set(event_target_value(&ev));
                }
            />
            <Show when=move || validation_state.get().is_err()>
                <p class=class_error_message>{move || validation_state.get().take_msg()}</p>
            </Show>
        }
        .into_view();

        self.common_component(&control.styles, "", view)
    }

    fn slider(
        &self,
        control: Rc<ControlRenderData<Self, SliderData>>,
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let label_class = move || {
            if validation_state.get().is_err() {
                Self::label_class_error()
            } else {
                Self::label_class()
            }
        };

        let class_error_message = Self::class_error_message();
        let input_class = move || {
            if validation_state.get().is_err() {
                Self::input_class_error()
            } else {
                Self::input_class()
            }
        };
        // let min_label = move || match control.data.min.clone() {} format!("Min ({})", );

        let view = view! {
            <div class="relative mb-6">
                <label for=&control.data.name class=label_class>
                    {control.data.label.as_ref()}
                </label>
                <input
                    type="range"
                    id=&control.data.name
                    name=&control.data.name
                    min=control.data.min.clone()
                    max=control.data.max.clone()
                    class=input_class
                    prop:value=move || value_getter.get()
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        value_setter.set(value);
                    }
                />
                // <span class="text-sm text-gray-500 dark:text-gray-400 absolute start-0 -bottom-6">{min_label()}</span>
                // <p class="text-sm text-gray-500 dark:text-gray-400 absolute end-0 -bottom-6">Max ({move || control.data.max.clone()})</p>
                <Show when=move || validation_state.get().is_err()>
                    <p class=class_error_message>{move || validation_state.get().take_msg()}</p>
                </Show>
            </div>
        }
        .into_view();

        self.common_component(&control.styles, "", view)
    }

    fn date(
        &self,
        control: Rc<ControlRenderData<Self, DateData>>,
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let label_class = move || {
            if validation_state.get().is_err() {
                Self::label_class_error()
            } else {
                Self::label_class()
            }
        };

        //"bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full ps-10 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
        let class_error_message = Self::class_error_message();
        let input_class = move || {
            if validation_state.get().is_err() {
                Self::input_class_error()
            } else {
                Self::input_class()
            }
        };
        // let min_label = move || match control.data.min.clone() {} format!("Min ({})", );

        let view = view! {
            <label for=&control.data.name class=label_class>
            {control.data.label.as_ref()}
            </label>
            <div class="relative mb-6">
                // <div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
                //     <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                //         <path d="M20 4a2 2 0 0 0-2-2h-2V1a1 1 0 0 0-2 0v1h-3V1a1 1 0 0 0-2 0v1H6V1a1 1 0 0 0-2 0v1H2a2 2 0 0 0-2 2v2h20V4ZM0 18a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8H0v10Zm5-8h10a1 1 0 0 1 0 2H5a1 1 0 0 1 0-2Z"/>
                //     </svg>
                // </div>
                <input
                    type="text"
                    id=&control.data.name
                    name=&control.data.name
                    min=control.data.min.clone()
                    max=control.data.max.clone()
                    datepicker=""
                    // datepicker-title=&control.data.title
                    placeholder="Select date"
                    class=input_class
                    prop:value=move || value_getter.get()
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        value_setter.set(value);
                    }
            />
        // <span class="text-sm text-gray-500 dark:text-gray-400 absolute start-0 -bottom-6">{min_label()}</span>
                // <p class="text-sm text-gray-500 dark:text-gray-400 absolute end-0 -bottom-6">Max ({move || control.data.max.clone()})</p>
                <Show when=move || validation_state.get().is_err()>
                    <p class=class_error_message>{move || validation_state.get().take_msg()}</p>
                </Show>
            </div>
        }
        .into_view();

        self.common_component(&control.styles, "", view)
    }
}
