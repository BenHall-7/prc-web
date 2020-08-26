use lazy_static::lazy_static;
use prc::param::{ParamKind};
use std::io::Cursor;
use yew::prelude::*;
// use yew::services::storage::{Area, StorageService};

use crate::components::{ParamParent, ParamTreeNode};

const TEST_FILE: &[u8] = include_bytes!("test.prc");

lazy_static! {
    static ref ROOT: ParamKind = {
        let mut reader = Cursor::new(&TEST_FILE);
        let data = prc::read_stream(&mut reader).unwrap();
        ParamKind::Struct(data)
    };
}

pub struct App {
    // link: ComponentLink<Self>,
    // storage: StorageService,
    state: State,
}

pub struct State {
    param_root: &'static ParamKind,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {
            // link,
            // storage: StorageService::new(Area::Session).unwrap(),
            state: State { param_root: &ROOT },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="app">
                <header>
                    <h1>{"PRC Web Editor"}</h1>
                </header>
                <section class="tree-view">
                    <ParamTreeNode
                        param=self.state.param_root
                        parent=ParamParent(None)
                        expand=true
                    />
                </section>
            </div>
        }
    }

    
}
