use dioxus::prelude::*;
use rand::RngCore;

#[component]
pub fn DirectoryInput(
    value: String,
    label: String,
    is_valid: bool,
    onchange: EventHandler<FormEvent>,
    onpickerclicked: EventHandler<MouseEvent>,
) -> Element {
    let mut rng = rand::rng();
    let seed = rng.next_u32();
    let btn_id = format!("btn_{}", seed);
    let input_id = format!("input_{}", seed);

    rsx!(
        form {
            label {
                class:"form-label",
                for: input_id.clone(),
                { label.clone() }
            }
            div {
                class: "input-group",
                span {
                    class: "input-group-text",
                    i {
                        class: "bi bi-folder",
                    }
                }
                input {
                    type: "text",
                    class: "form-control ".to_string() + if !is_valid { "is-invalid" } else { "" },
                    value,
                    oninput: onchange,
                    aria_label: label.clone(),
                    aria_describedby: btn_id.clone(),
                    id: input_id.clone(),
                }
                button {
                    id: btn_id,
                    class: "btn btn-outline-secondary",
                    type: "button",
                    onclick: onpickerclicked,
                    i {
                        class: "bi bi-box-arrow-up-right",
                    }
                }
            }
        }
    )
}
