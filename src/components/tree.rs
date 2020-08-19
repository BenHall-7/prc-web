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
}

impl Component for ParamTreeNode {
    type Message = TreeMessage;
    type Properties = TreeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let param = Param::from(props.param);
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
        macro_rules! get_html_with_children {
            ($main_tag:ident, $node_text:expr) => {
                html! {
                    <$main_tag class="treeitem">
                        { if self.can_expand() {
                            html! {
                                <button onclick=self.link.callback(|_| TreeMessage::ToggleExpand) class="expand-button">
                                    <img src=if self.expanded {"/image/angle-down-solid.svg"} else {"/image/angle-right-solid.svg"} />
                                </button>
                            }
                        } else { html!{} }}
                        <p>{$node_text}</p>
                        <ul hidden=!self.expanded>
                            { children.iter().enumerate().map(|c| {
                                let props: TreeProps = c.into();
                                html! {
                                    <ParamTreeNode
                                        param=props.param
                                        parent=props.parent
                                        expand=true
                                    />
                                }
                            }).collect::<Html>() }
                        </ul>
                    </$main_tag>
                }
            };
        }

        macro_rules! get_html {
            ($main_tag:ident, $node_text:expr) => {
                html! {<$main_tag class="treeitem"><p>{$node_text}</p></$main_tag>}
            };
        }

        match self.param {
            ParamKind::Struct(children) => {
                if let Some(_) = self.parent.0 {
                    get_html_with_children!(li, self.parent.get_name())
                } else {
                    get_html_with_children!(div, "root")
                }
            }
            ParamKind::List(children) => get_html_with_children!(li, self.parent.get_name()),
            ParamKind::Bool(_) => get_html!(li, self.parent.get_name()),
            ParamKind::I8(_) => get_html!(li, self.parent.get_name()),
            ParamKind::U8(_) => get_html!(li, self.parent.get_name()),
            ParamKind::I16(_) => get_html!(li, self.parent.get_name()),
            ParamKind::U16(_) => get_html!(li, self.parent.get_name()),
            ParamKind::I32(_) => get_html!(li, self.parent.get_name()),
            ParamKind::U32(_) => get_html!(li, self.parent.get_name()),
            ParamKind::Float(_) => get_html!(li, self.parent.get_name()),
            ParamKind::Hash(_) => get_html!(li, self.parent.get_name()),
            ParamKind::Str(_) => get_html!(li, self.parent.get_name()),
        }
    }
}
