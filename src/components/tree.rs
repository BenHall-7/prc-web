use prc::param::ParamKind;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
// use prc::strum::AsStaticRef;
use prc::hash40::Hash40;

#[derive(Debug, Clone, Copy)]
pub struct ParamParent(pub Option<ParentInfo>);

#[derive(Debug, Clone, Copy)]
pub enum ParentInfo {
    /// Name of child node, index
    Struct(Hash40, usize),
    /// Index of child node
    List(usize),
}

impl ParamParent {
    fn get_name(&self) -> String {
        match self.0 {
            Some(ParentInfo::Struct(hash, _)) => format!("{}", hash),
            Some(ParentInfo::List(index)) => format!("{}", index),
            None => "root".into(),
        }
    }
}

#[derive(Debug, Clone, Properties)]
pub struct TreeProps {
    pub param: &'static ParamKind,
    pub parent: ParamParent,
    #[prop_or_default]
    pub expand: bool,
}

impl From<(usize, &'static (Hash40, ParamKind))> for TreeProps {
    fn from(f: (usize, &'static (Hash40, ParamKind))) -> Self {
        Self {
            param: &(f.1).1,
            parent: ParamParent(Some(ParentInfo::Struct((f.1).0, f.0))),
            expand: false,
        }
    }
}

impl From<(usize, &'static ParamKind)> for TreeProps {
    fn from(f: (usize, &'static ParamKind)) -> Self {
        Self {
            param: f.1,
            parent: ParamParent(Some(ParentInfo::List(f.0))),
            expand: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TreeMessage {
    ToggleExpand,
}

#[derive(Debug, Clone)]
pub struct ParamTreeNode {
    pub link: ComponentLink<Self>,
    pub parent: ParamParent,
    pub param: &'static ParamKind,
    pub expanded: bool,
}

impl ParamTreeNode {
    fn can_expand(&self) -> bool {
        match self.param {
            ParamKind::List(v) => !v.is_empty(),
            ParamKind::Struct(v) => !v.is_empty(),
            _ => false,
        }
    }

    fn is_expanded(&self) -> bool {
        self.can_expand() && self.expanded
    }

    fn view_wrapper(&self, inner: Html) -> Html {
        if self.parent.0.is_some() {
            html! { <li class="tree-container">{ inner }</li> }
        } else {
            html! { <div class="tree-container">{ inner }</div> }
        }
    }

    fn view_container<T: Into<TreeProps>, I: Iterator<Item=T>>(&self, children: I) -> Html {
        self.view_wrapper(
            html! { <>
                <div onclick=self.link.callback(|_| TreeMessage::ToggleExpand) class="tree-header">
                    {self.view_header_content()}
                </div>
                {if self.is_expanded() {
                    self.view_children(children)
                } else { html! {} }}
            </> }    
        )
    }

    fn view_children<T: Into<TreeProps>, I: Iterator<Item=T>>(&self, children: I) -> Html {
        html! {<ul>{
            children.map(|c| {
                let props: TreeProps = c.into();
                html! {
                    <ParamTreeNode
                        param=props.param
                        parent=props.parent
                    />
                }
            }).collect::<Html>()
        }</ul>}
    }

    fn view_header_content(&self) -> Html {
        if self.can_expand() {
            html! {<>
                <button class="expand-button">
                    <img src=if self.expanded {"/image/caret-down.png"} else {"/image/caret-right.png"} />
                </button>
                <p>{self.parent.get_name()}</p>
            </>}
        } else {
            html! {<p class="corrected">{self.parent.get_name()}</p>}
        }
    }

    fn view_value_type(&self) -> Html {
        html! {
            <li class="tree-container">
                <p class="corrected">
                    {self.parent.get_name()}
                </p>
            </li>
        }
    }
}

impl Component for ParamTreeNode {
    type Message = TreeMessage;
    type Properties = TreeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            parent: props.parent,
            param: props.param,
            expanded: props.expand,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TreeMessage::ToggleExpand => {
                self.expanded = !self.expanded;
                true
            }
        }
    }

    fn view(&self) -> Html {
        match self.param {
            ParamKind::Struct(children) => {
                self.view_container(children.iter().enumerate())
            }
            ParamKind::List(children) => {
                self.view_container(children.iter().enumerate())
            }
            _ => self.view_value_type(),
        }
    }
}
