use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{
    Sizable,
    button::{Button, ButtonVariants},
    checkbox::Checkbox,
    form::{field, v_form},
    h_flex,
    input::{Input, InputState},
};

pub struct RegistrationForm {
    first_name: Entity<InputState>,
    last_name: Entity<InputState>,
    email: Entity<InputState>,
    password: Entity<InputState>,
    confirm_password: Entity<InputState>,
    terms_accepted: bool,
}

impl RegistrationForm {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let first_name =
            cx.new(|closure_cx| InputState::new(window, closure_cx).placeholder("First name..."));
        let last_name =
            cx.new(|closure_cx| InputState::new(window, closure_cx).placeholder("Last name..."));
        let email =
            cx.new(|closure_cx| InputState::new(window, closure_cx).placeholder("Valid email..."));
        let password =
            cx.new(|closure_cx| InputState::new(window, closure_cx).placeholder("Password..."));
        let confirm_password = cx.new(|closure_cx| {
            InputState::new(window, closure_cx).placeholder("Confirm password...")
        });
        let terms_accepted = false;

        Self {
            first_name,
            last_name,
            email,
            password,
            confirm_password,
            terms_accepted,
        }
    }
}

impl Render for RegistrationForm {
    fn render(
        &mut self,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_form()
            .large()
            .child(
                field()
                    .label("Personal Information")
                    .label_indent(true)
                    .child(
                        h_flex()
                            .gap_3()
                            .child(div().flex_1().child(Input::new(&self.first_name)))
                            .child(div().flex_1().child(Input::new(&self.last_name))),
                    ),
            )
            .child(
                field()
                    .label("Email")
                    .required(true)
                    .child(Input::new(&self.email)),
            )
            .child(
                field()
                    .label("Password")
                    .required(true)
                    .description("Must be at least 8 characters")
                    .child(Input::new(&self.password)),
            )
            .child(
                field()
                    .label("Confirm Password")
                    .required(true)
                    .child(Input::new(&self.confirm_password)),
            )
            .child(
                field().label_indent(false).child(
                    Checkbox::new("terms")
                        .label("I agree to the Terms of Service")
                        .checked(self.terms_accepted)
                        .on_click(cx.listener(|this, checked, _, cx| {
                            this.terms_accepted = *checked;
                            cx.notify();
                        })),
                ),
            )
            .child(
                field().label_indent(false).child(
                    Button::new("register")
                        .primary()
                        .large()
                        .w_full()
                        .child("Create Account"),
                ),
            )
    }
}
