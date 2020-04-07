use yew::{Component, ComponentLink, Properties, Html, html};
use prc::param::{ParamKind};
use prc::strum::AsStaticRef;
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
pub enum TreeNode {
    /// Contains children, parent info, and a property for whether it is expanded
    Struct(Vec<TreeNode>, Option<ParentInfo>, bool),
    /// Contains children, parent info, and a property for whether it is expanded
    List(Vec<TreeNode>, Option<ParentInfo>, bool),
    /// Contains the real param and parent info
    Value(ParamKind, Option<ParentInfo>),
}

impl TreeNode {
    #[inline]
    pub fn can_expand(&self) -> bool {
        match self {
            TreeNode::Struct(children, _, _) => !children.is_empty(),
            TreeNode::List(children, _, _) => !children.is_empty(),
            _ => false
        }
    }
}

impl Component for TreeNode {
    type Message = ();
    type Properties = TreeProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let param = props.param;
        match param {
            ParamKind::Struct(mut s) => {
                TreeNode::Struct(
                    s.drain(..).enumerate().map(|(i, (h, p))| {
                        TreeNode::create(
                            TreeProps::new(
                                p,
                                Some(ParentInfo::Struct(h, i)),
                                true
                            ),
                            ComponentLink::default()
                        )
                    }).collect::<Vec<_>>(),

                    props.parent,

                    props.expand
                )
            }
            ParamKind::List(mut l) => {
                TreeNode::List(
                    l.drain(..).enumerate().map(|(i, p)| {
                        TreeNode::create(
                            TreeProps::new(
                                p,
                                Some(ParentInfo::List(i)),
                                true
                            ),
                            ComponentLink::default()
                        )
                    }).collect::<Vec<_>>(),

                    props.parent,

                    props.expand
                )
            }
            _ => {
                TreeNode::Value(param, props.parent)
            }
        }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        macro_rules! get_html_with_children {
            ($main_tag:ident, $node_text:expr) => {
                html! {<$main_tag>
                    { if self.can_expand() {
                        html! {<button> 
                            { if *expanded {
                                html! {<image src="image/angle-down-solid.svg"/>}
                            } else {
                                html! {<image src="image/angle-down-solid.svg"/>}
                            }}
                        </button>}
                    } else { html!{} }}
                    <p>{$node_text}</p>
                    { if *expanded {
                        html! {<ul>
                            {children.iter().map(|c| c.view()).collect::<Html>()}    
                        </ul>}
                    } else {
                        html! {}
                    }}
                </$main_tag>}
            };
        }
        macro_rules! get_html {
            ($main_tag:ident, $node_text:expr) => {
                html! {<$main_tag><p>{$node_text}</p></$main_tag>}
            };
        }

        match self {
            TreeNode::Struct(children, parent_info, expanded) => {
                if let Some(parent) = parent_info {
                    get_html_with_children!(li, parent.get_name() + " (Struct)")
                } else {
                    get_html_with_children!(div, "Root (Struct)")
                }
            }
            TreeNode::List(children, parent_info, expanded) => {
                let parent = parent_info.unwrap();
                get_html_with_children!(li, parent.get_name() + " (List)")
            }
            TreeNode::Value(param, parent_info) => {
                let parent = parent_info.unwrap();
                get_html!(li, parent.get_name() + param.as_ref())
            }
        }
    }
}