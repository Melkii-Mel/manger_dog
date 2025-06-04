use entities::Currency;
use yew::AttrValue;
use crate::form_input::inputs::OptionEnum;

impl OptionEnum for Currency {
    fn display(&self) -> AttrValue {
        self.identifier.clone().into()
    }
}