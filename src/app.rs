use prc::param::{ParamKind, ParamStruct};
use web_sys::Window;
use yew::services::storage::{Area, StorageService};
use yew::prelude::*;
use lazy_static::lazy_static;
use std::io::Cursor;

use crate::components::{ParamTreeNode, ParamParent};

const TEST_FILE: &[u8] = include_bytes!("etc.prc");

lazy_static!(
    static ref ROOT: ParamKind = {
        let mut reader = Cursor::new(&TEST_FILE);
        let data = prc::read_stream(&mut reader).unwrap();
        ParamKind::Struct(data)
    };
);

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

pub struct State {
    param_root: &'static ParamKind,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            storage: StorageService::new(Area::Session).unwrap(),
            state: State {
                param_root: &ROOT,
            }
        } 
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"etc.prc"}</h1>
                <ParamTreeNode
                    param=self.state.param_root
                    parent=ParamParent(None)
                    expand=true
                />
            </div>
        }
    }
}