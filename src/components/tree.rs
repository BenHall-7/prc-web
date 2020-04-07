use yew::{Component, ComponentLink, Properties, Html, html};
use prc::param::{ParamKind};
use prc::hash40::Hash40;

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
        match self {
            TreeNode::Struct(children, parent_info, expanded) => {
                if let Some(parent) = parent_info {
                    html! {<li>
                        <p>{parent.get_name() + " (Struct)"}</p>
                        {
                            if *expanded {
                                html! {<ul>
                                    {children.iter().map(|c| c.view()).collect::<Html>()}    
                                </ul>}
                            } else {
                                html! {}
                            }
                        }
                    </li>}
                } else {
                    html! {<div class="treeviewroot">
                        <p class="treeviewitem">{"Root (Struct)"}</p>
                        {
                            if *expanded {
                                html! {<ul>
                                    {children.iter().map(|c| c.view()).collect::<Html>()}    
                                </ul>}
                            } else {
                                html! {}
                            }
                        }
                    </div>}
                }
            }
            TreeNode::List(children, parent_info, expanded) => {
                let parent = parent_info.unwrap();
                html! {<li>
                    <p>{parent.get_name() + " (List)"}</p>
                    {
                        if *expanded {
                            html! {<ul>
                                {children.iter().map(|c| c.view()).collect::<Html>()}    
                            </ul>}
                        } else {
                            html! {}
                        }
                    }
                </li>}
            }
            TreeNode::Value(param, parent_info) => {
                let parent = parent_info.unwrap();
                html! {<li>
                    <p>{parent.get_name() + match param {
                        ParamKind::Struct(_) => unreachable!(),
                        ParamKind::List(_) => unreachable!(),
                        ParamKind::Bool(_) => " (Bool)",
                        ParamKind::U8(_) => " (U8)",
                        ParamKind::I8(_) => " (I8)",
                        ParamKind::U16(_) => " (U16)",
                        ParamKind::I16(_) => " (I16)",
                        ParamKind::U32(_) => " (U32)",
                        ParamKind::I32(_) => " (I32)",
                        ParamKind::Float(_) => " (Float)",
                        ParamKind::Hash(_) => " (Hash)",
                        ParamKind::Str(_) => " (Str)",
                    }}</p>
                </li>}
            }
        }
    }
}