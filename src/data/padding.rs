use super::StepUnit;

#[derive(Clone, PartialEq, Default)]
pub struct Padding {
    top: StepUnit,
    left: StepUnit,
    bottom: StepUnit,
    right: StepUnit,
}

impl Padding {
    pub fn xy(vertical: StepUnit, horizontal: StepUnit) -> Self {
        Self {
            top: vertical.clone(),
            left: horizontal.clone(),
            bottom: vertical,
            right: horizontal,
        }
    }

    pub fn all(all: StepUnit) -> Self {
        Self {
            top: all.clone(),
            bottom: all.clone(),
            left: all.clone(),
            right: all,
        }
    }

    pub fn new(top: StepUnit, right: StepUnit, bottom: StepUnit, left: StepUnit) -> Self {
        Self {
            top,
            right,
            left,
            bottom,
        }
    }

    pub fn to_classes(&self) -> String {
        return format!(
            "pt-{} ps-{} pe-{} pb-{}",
            self.top, self.left, self.right, self.bottom
        );
    }
}
