use crate::form_input::inputs::OptionEnum;
use entities::Currency;
use yew::AttrValue;

impl OptionEnum for Currency {
    fn display(&self) -> AttrValue {
        self.identifier.clone().into()
    }
}
