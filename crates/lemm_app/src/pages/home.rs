use dioxus::prelude::*;
use rfd::AsyncFileDialog;

use crate::data::{Padding, StepUnit};
use crate::{
    components::{Button, Container},
    data::AppState,
};

#[component]
pub fn Home() -> Element {
    rsx! {
        Container {
            padding: Padding::all(StepUnit::Positive(3)),

            h1 {
                "DSLE Mod Manager"
            }

            Container {
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus feugiat mi sit amet turpis facilisis, ut placerat mi egestas. Fusce pharetra vitae enim quis vulputate. Proin scelerisque eu nisi at lobortis. Cras vitae arcu lacinia, malesuada felis sed, laoreet erat. Integer a venenatis velit, vitae porttitor velit. Vivamus quis odio eget leo aliquam tempor. Ut eget feugiat sem. Phasellus egestas elit eget vestibulum rutrum. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. In non sapien et magna pulvinar vulputate ac vitae tellus. Suspendisse malesuada rutrum nibh, sit amet pellentesque lectus ultrices at. Etiam consectetur vehicula mauris id dapibus. Praesent quis odio orci. Nulla vehicula dolor purus, nec aliquet quam iaculis et. Aliquam erat volutpat. Etiam ac arcu nec nisl iaculis venenatis nec sit amet erat."                }
                p {
                    "Curabitur auctor, orci at mattis ultrices, nisl lectus bibendum mauris, mattis ullamcorper arcu mi a elit. Ut non velit a neque viverra ullamcorper. Vestibulum eget enim sit amet quam mattis rhoncus sit amet et sapien. Nunc fermentum tincidunt pharetra. Quisque in bibendum mi. Vivamus iaculis, mauris eget tempor vehicula, velit justo volutpat mauris, sit amet convallis purus purus quis nunc. Vestibulum tincidunt facilisis lectus, eget vestibulum dolor tempus at. Nam pharetra sagittis ex eu facilisis. Interdum et malesuada fames ac ante ipsum primis in faucibus. Ut at ligula sem. Nunc quis erat magna. Donec semper, erat eget eleifend varius, leo ex porttitor augue, ac dignissim metus nulla nec felis. Curabitur suscipit imperdiet enim. "                }
            }
        }
    }
}
