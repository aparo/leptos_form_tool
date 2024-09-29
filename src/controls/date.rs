use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, ControlRenderData, ValidationState,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{MaybeSignal, RwSignal, Signal, SignalSetter, View};
use std::rc::Rc;

/// Data used for the date control.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DateData {
    pub name: String,
    pub label: Option<String>,
    pub show_buttons: Option<MaybeSignal<bool>>,
    pub show_today: Option<MaybeSignal<bool>>,
    pub title: Option<String>,
    pub format: Option<String>,
    pub min: Option<MaybeSignal<String>>,
    pub max: Option<MaybeSignal<String>>,
}

impl<FD: FormToolData> ControlData<FD> for DateData {
    /// String to support integers or decimal point types.
    type ReturnType = String;

    fn render_control<FS: FormStyle>(
        fs: &FS,
        _fd: RwSignal<FD>,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        fs.date(control, value_getter, value_setter, validation_state)
    }
}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a date control and adds it to the form.
    pub fn date<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderFn<ControlBuilder<FD, DateData, FDT>>,
    ) -> Self {
        self.new_control(builder)
    }

    /// Bulids a date control using the form's context and adds
    /// it to the form.
    pub fn date_cx<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderCxFn<ControlBuilder<FD, DateData, FDT>, FD::Context>,
    ) -> Self {
        self.new_control_cx(builder)
    }
}

impl<FD: FormToolData, FDT> ControlBuilder<FD, DateData, FDT> {
    /// Sets the name of the date.
    ///
    /// This is used for the html element's "name" attribute.
    /// In forms, the name attribute is the key that the data is sent
    /// with.
    pub fn named(mut self, control_name: impl ToString) -> Self {
        self.data.name = control_name.to_string();
        self
    }

    /// Sets the label for the date.
    pub fn labeled(mut self, label: impl ToString) -> Self {
        self.data.label = Some(label.to_string());
        self
    }

    /// Sets the title of the date.
    pub fn title(mut self, title: impl ToString) -> Self {
        self.data.title = Some(title.to_string());
        self
    }

    /// Sets show today button.
    pub fn show_today(mut self, today: bool) -> Self {
        self.data.show_today = Some(MaybeSignal::Static(today));
        self.data.show_buttons = Some(MaybeSignal::Static(today));
        self
    }

    /// Sets show today button.
    pub fn show_buttons(mut self, show_buttons: bool) -> Self {
        self.data.show_buttons = Some(MaybeSignal::Static(show_buttons));
        self
    }

    /// Sets the format of the date.
    pub fn format(mut self, format: impl ToString) -> Self {
        self.data.format = Some(format.to_string());
        self
    }

    /// Sets the minimum value for the date.
    pub fn min(mut self, min: impl ToString) -> Self {
        self.data.min = Some(MaybeSignal::Static(min.to_string()));
        self
    }

    /// Sets the minimum value for the date to a signal.
    pub fn min_signal(mut self, min: Signal<String>) -> Self {
        self.data.min = Some(MaybeSignal::Dynamic(min));
        self
    }

    /// Sets the maximum value for the date.
    pub fn max(mut self, max: impl ToString) -> Self {
        self.data.max = Some(MaybeSignal::Static(max.to_string()));
        self
    }

    /// Sets the maximum value for the date to a signal.
    pub fn max_signal(mut self, max: Signal<String>) -> Self {
        self.data.max = Some(MaybeSignal::Dynamic(max));
        self
    }
}
