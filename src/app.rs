use prc::param::{ParamKind, ParamStruct};
use yew::services::storage::{Area, StorageService};
use yew::prelude::*;
use std::io::Cursor;

use super::components::{TreeNode};

const TEST_FILE: &[u8] = include_bytes!("etc.prc"); 

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

pub struct State {
    param_root: ParamStruct,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let param_root = prc::read_stream(&mut Cursor::new(&TEST_FILE)).unwrap();
        App {
            link,
            storage: StorageService::new(Area::Session).unwrap(),
            state: State {
                param_root
            }
        } 
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let param = ParamKind::Struct(self.state.param_root.clone());
        html! {
            <div>
                <h1>{"etc.prc"}</h1>
                <TreeNode param=param parent=None expand=true/>
            </div>
            
        }
    }
}