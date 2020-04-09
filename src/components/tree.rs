use yew::{Component, ComponentLink, Properties, Html, html};
use prc::param::{ParamKind};
// use prc::strum::AsStaticRef;
use prc::hash40::Hash40;

#[derive(Debug, Clone, Copy)]
pub enum ParentInfo {
    /// Name of child node, index, and param
    Struct(Hash40, usize),
    /// Index of child node and param
    List(usize),
}

impl ParentInfo {
    pub fn get_name(&self) -> String {
        match self {
            ParentInfo::Struct(hash, _) => format!("{}", hash),
            ParentInfo::List(index) => format!("{}", index),
        }
    }
}

#[derive(Debug, Clone, Properties)]
pub struct TreeProps {
    pub param: ParamKind,

    pub parent: Option<ParentInfo>,

    #[prop_or_default]
    pub expand: bool,
}

impl TreeProps {
    pub fn new(param: ParamKind, parent: Option<ParentInfo>, expand: bool) -> Self {
        TreeProps { param, parent, expand }
    }
}

#[derive(Debug, Clone)]
pub enum TreeMessage {
    ToggleExpand
}

#[derive(Debug, Clone)]
pub enum TreeNode {
    Struct {
        link: ComponentLink<Self>,
        children: Vec<TreeNode>,
        parent: Option<ParentInfo>,
        expanded: bool,
    },
    List {
        link: ComponentLink<Self>,
        children: Vec<TreeNode>,
        parent: ParentInfo,
        expanded: bool,
    },
    Value {
        link: ComponentLink<Self>,
        param: ParamKind,
        parent: ParentInfo,
    },
}

impl TreeNode {
    #[inline]
    pub fn can_expand(&self) -> bool {
        match self {
            TreeNode::Struct { children, .. } => !children.is_empty(),
            TreeNode::List { children, .. } => !children.is_empty(),
            _ => false
        }
    }

    #[inline]
    pub fn link(&self) -> &ComponentLink<Self> {
        match self {
            TreeNode::Struct { link, .. } => link,
            TreeNode::List { link, .. } => link,
            TreeNode::Value { link, .. } => link,
        }
    }
}

impl Component for TreeNode {
    type Message = TreeMessage;
    type Properties = TreeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let param = props.param;
        match param {
            ParamKind::Struct(mut s) => {
                TreeNode::Struct {
                    children: s.drain(..).enumerate().map(|(i, (h, p))| {
                        TreeNode::create(
                            TreeProps::new(
                                p,
                                Some(ParentInfo::Struct(h, i)),
                                true
                            ),
                            ComponentLink::default()
                        )
                    }).collect::<Vec<_>>(),

                    parent: props.parent,
                    expanded: props.expand,
                    link,
                }
            }
            ParamKind::List(mut l) => {
                TreeNode::List {
                    children: l.drain(..).enumerate().map(|(i, p)| {
                        TreeNode::create(
                            TreeProps::new(
                                p,
                                Some(ParentInfo::List(i)),
                                true
                            ),
                            ComponentLink::default()
                        )
                    }).collect::<Vec<_>>(),

                    parent: props.parent.unwrap(),
                    expanded: props.expand,
                    link,
                }
            }
            _ => {
                TreeNode::Value { 
                    param, 
                    parent: props.parent.unwrap(),
                    link,
                }
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            TreeMessage::ToggleExpand => {
                match self {
                    TreeNode::Struct { expanded, .. } => {
                        *expanded = !*expanded
                    }
                    TreeNode::List { expanded, .. } => {
                        *expanded = !*expanded
                    }
                    _ => {}
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        macro_rules! get_html_with_children {
            ($main_tag:ident, $node_text:expr) => {
                html! {<$main_tag class="treeitem">
                    { if self.can_expand() {
                        html! {<button onclick=self.link().callback(|_| TreeMessage::ToggleExpand)> 
                            { if *expanded {
                                html! {<image src="/image/angle-down-solid.svg"/>}
                            } else {
                                html! {<image src="/image/angle-right-solid.svg"/>}
                            }}
                        </button>}
                    } else { html!{} }}
                    <p>{$node_text}</p>
                    <ul hidden=!*expanded>
                        {children.iter().map(|c| c.view()).collect::<Html>()}
                    </ul>
                </$main_tag>}
            };
        }

        macro_rules! get_html {
            ($main_tag:ident, $node_text:expr) => {
                html! {<$main_tag class="treeitem"><p>{$node_text}</p></$main_tag>}
            };
        }

        match self {
            TreeNode::Struct { children, parent: parent_info, expanded, .. } => {
                if let Some(parent) = parent_info {
                    get_html_with_children!(li, parent.get_name())
                } else {
                    get_html_with_children!(div, "root")
                }
            }
            TreeNode::List { children, parent, expanded, .. } => {
                get_html_with_children!(li, parent.get_name())
            }
            TreeNode::Value { parent, .. } => {
                get_html!(li, parent.get_name())
            }
        }
    }
}