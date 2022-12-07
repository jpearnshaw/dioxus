// use proc_macro2::TokenStream;
// use quote::TokenStreamExt;
// use quote::{quote, ToTokens};
// use syn::{Expr, Ident, LitStr};

// #[cfg(any(feature = "hot-reload", debug_assertions))]
// pub fn try_parse_template(
//     rsx: &str,
//     location: OwnedCodeLocation,
//     previous_template: Option<DynamicTemplateContextBuilder>,
// ) -> Result<(OwnedTemplate, DynamicTemplateContextBuilder), Error> {
//     use crate::CallBody;

//     let call_body: CallBody =
//         syn::parse_str(rsx).map_err(|e| Error::ParseError(ParseError::new(e, location.clone())))?;
//     let mut template_builder = TemplateBuilder::from_roots_always(call_body.roots);
//     if let Some(prev) = previous_template {
//         template_builder = template_builder
//             .try_switch_dynamic_context(prev)
//             .ok_or_else(|| {
//                 Error::RecompileRequiredError(RecompileReason::Variable(
//                     "dynamic context updated".to_string(),
//                 ))
//             })?;
//     }
//     let dyn_ctx = template_builder.dynamic_context.clone();
//     Ok((template_builder.try_into_owned(&location)?, dyn_ctx))
// }

// use crate::{BodyNode, ElementAttr, FormattedSegment, Segment};

// struct TemplateElementBuilder {
//     tag: Ident,
//     attributes: Vec<TemplateAttributeBuilder>,
//     children: Vec<TemplateNodeId>,
//     listeners: Vec<usize>,
//     locally_static: bool,
// }

// #[cfg(any(feature = "hot-reload", debug_assertions))]
// type OwndedTemplateElement = TemplateElement<
//     Vec<TemplateAttribute<OwnedAttributeValue>>,
//     OwnedAttributeValue,
//     Vec<TemplateNodeId>,
//     Vec<usize>,
// >;

// impl TemplateElementBuilder {
//     #[cfg(any(feature = "hot-reload", debug_assertions))]
//     fn try_into_owned(self, location: &OwnedCodeLocation) -> Result<OwndedTemplateElement, Error> {
//         let Self {
//             tag,
//             attributes,
//             children,
//             listeners,
//             ..
//         } = self;
//         let (element_tag, element_ns) =
//             element_to_static_str(&tag.to_string()).ok_or_else(|| {
//                 Error::ParseError(ParseError::new(
//                     syn::Error::new(tag.span(), format!("unknown element: {}", tag)),
//                     location.clone(),
//                 ))
//             })?;

//         let mut owned_attributes = Vec::new();
//         for a in attributes {
//             owned_attributes.push(a.try_into_owned(location, element_tag, element_ns)?);
//         }

//         Ok(TemplateElement::new(
//             element_tag,
//             element_ns,
//             owned_attributes,
//             children,
//             listeners,
//         ))
//     }
// }

// impl ToTokens for TemplateElementBuilder {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let Self {
//             tag,
//             attributes,
//             children,
//             listeners,
//             ..
//         } = self;
//         let children = children.iter().map(|id| {
//             let raw = id.0;
//             quote! {TemplateNodeId(#raw)}
//         });
//         tokens.append_all(quote! {
//             TemplateElement::new(
//                 dioxus_elements::#tag::TAG_NAME,
//                 dioxus_elements::#tag::NAME_SPACE,
//                 &[#(#attributes),*],
//                 &[#(#children),*],
//                 &[#(#listeners),*],
//             )
//         })
//     }
// }

// enum AttributeName {
//     Ident(Ident),
//     Str(LitStr),
// }

// struct TemplateAttributeBuilder {
//     element_tag: Ident,
//     name: AttributeName,
//     value: TemplateAttributeValue<OwnedAttributeValue>,
// }

// impl TemplateAttributeBuilder {
//     #[cfg(any(feature = "hot-reload", debug_assertions))]
//     fn try_into_owned(
//         self,
//         location: &OwnedCodeLocation,
//         element_tag: &'static str,
//         element_ns: Option<&'static str>,
//     ) -> Result<TemplateAttribute<OwnedAttributeValue>, Error> {
//         let Self { name, value, .. } = self;
//         let (name, span, literal) = match name {
//             AttributeName::Ident(name) => (name.to_string(), name.span(), false),
//             AttributeName::Str(name) => (name.value(), name.span(), true),
//         };
//         let (name, namespace, volatile) = attrbute_to_static_str(&name, element_tag, element_ns)
//             .ok_or_else(|| {
//                 if literal {
//                     // literals will be captured when a full recompile is triggered
//                     Error::RecompileRequiredError(RecompileReason::Attribute(name.to_string()))
//                 } else {
//                     Error::ParseError(ParseError::new(
//                         syn::Error::new(span, format!("unknown attribute: {}", name)),
//                         location.clone(),
//                     ))
//                 }
//             })?;
//         let attribute = AttributeDiscription {
//             name,
//             namespace,
//             volatile,
//         };
//         Ok(TemplateAttribute { value, attribute })
//     }
// }

// impl ToTokens for TemplateAttributeBuilder {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let Self {
//             element_tag,
//             name,
//             value,
//         } = self;
//         let value = match value {
//             TemplateAttributeValue::Static(val) => {
//                 let val = match val {
//                     OwnedAttributeValue::Text(txt) => quote! {StaticAttributeValue::Text(#txt)},
//                     OwnedAttributeValue::Float32(f) => quote! {StaticAttributeValue::Float32(#f)},
//                     OwnedAttributeValue::Float64(f) => quote! {StaticAttributeValue::Float64(#f)},
//                     OwnedAttributeValue::Int32(i) => quote! {StaticAttributeValue::Int32(#i)},
//                     OwnedAttributeValue::Int64(i) => quote! {StaticAttributeValue::Int64(#i)},
//                     OwnedAttributeValue::Uint32(u) => quote! {StaticAttributeValue::Uint32(#u)},
//                     OwnedAttributeValue::Uint64(u) => quote! {StaticAttributeValue::Uint64(#u)},
//                     OwnedAttributeValue::Bool(b) => quote! {StaticAttributeValue::Bool(#b)},
//                     OwnedAttributeValue::Vec3Float(f1, f2, f3) => {
//                         quote! {StaticAttributeValue::Vec3Float(#f1, #f2, #f3)}
//                     }
//                     OwnedAttributeValue::Vec3Int(f1, f2, f3) => {
//                         quote! {StaticAttributeValue::Vec3Int(#f1, #f2, #f3)}
//                     }
//                     OwnedAttributeValue::Vec3Uint(f1, f2, f3) => {
//                         quote! {StaticAttributeValue::Vec3Uint(#f1, #f2, #f3)}
//                     }
//                     OwnedAttributeValue::Vec4Float(f1, f2, f3, f4) => {
//                         quote! {StaticAttributeValue::Vec4Float(#f1, #f2, #f3, #f4)}
//                     }
//                     OwnedAttributeValue::Vec4Int(f1, f2, f3, f4) => {
//                         quote! {StaticAttributeValue::Vec4Int(#f1, #f2, #f3, #f4)}
//                     }
//                     OwnedAttributeValue::Vec4Uint(f1, f2, f3, f4) => {
//                         quote! {StaticAttributeValue::Vec4Uint(#f1, #f2, #f3, #f4)}
//                     }
//                     OwnedAttributeValue::Bytes(b) => {
//                         quote! {StaticAttributeValue::Bytes(&[#(#b),*])}
//                     }
//                     OwnedAttributeValue::Any(_) => todo!(),
//                 };
//                 quote! {TemplateAttributeValue::Static(#val)}
//             }
//             TemplateAttributeValue::Dynamic(idx) => quote! {TemplateAttributeValue::Dynamic(#idx)},
//         };
//         match name {
//             AttributeName::Ident(name) => tokens.append_all(quote! {
//                 TemplateAttribute{
//                     attribute: dioxus_elements::#element_tag::#name,
//                     value: #value,
//                 }
//             }),
//             AttributeName::Str(lit) => tokens.append_all(quote! {
//                 TemplateAttribute{
//                     attribute: dioxus::prelude::AttributeDiscription{
//                         name: #lit,
//                         namespace: None,
//                         volatile: false
//                     },
//                     value: #value,
//                 }
//             }),
//         }
//     }
// }

// enum TemplateNodeTypeBuilder {
//     Element(TemplateElementBuilder),
//     Text(TextTemplate<Vec<TextTemplateSegment<String>>, String>),
//     DynamicNode(usize),
// }

// #[cfg(any(feature = "hot-reload", debug_assertions))]
// type OwnedTemplateNodeType = TemplateNodeType<
//     Vec<TemplateAttribute<OwnedAttributeValue>>,
//     OwnedAttributeValue,
//     Vec<TemplateNodeId>,
//     Vec<usize>,
//     Vec<TextTemplateSegment<String>>,
//     String,
// >;

// impl TemplateNodeTypeBuilder {
//     #[cfg(any(feature = "hot-reload", debug_assertions))]
//     fn try_into_owned(self, location: &OwnedCodeLocation) -> Result<OwnedTemplateNodeType, Error> {
//         match self {
//             TemplateNodeTypeBuilder::Element(el) => {
//                 Ok(TemplateNodeType::Element(el.try_into_owned(location)?))
//             }
//             TemplateNodeTypeBuilder::Text(txt) => Ok(TemplateNodeType::Text(txt)),
//             TemplateNodeTypeBuilder::DynamicNode(idx) => Ok(TemplateNodeType::DynamicNode(idx)),
//         }
//     }
// }

// impl ToTokens for TemplateNodeTypeBuilder {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         match self {
//             TemplateNodeTypeBuilder::Element(el) => tokens.append_all(quote! {
//                 TemplateNodeType::Element(#el)
//             }),
//             TemplateNodeTypeBuilder::Text(txt) => {
//                 let mut length = 0;

//                 let segments = txt.segments.iter().map(|seg| match seg {
//                     TextTemplateSegment::Static(s) => {
//                         length += s.len();
//                         quote!(TextTemplateSegment::Static(#s))
//                     }
//                     TextTemplateSegment::Dynamic(idx) => quote!(TextTemplateSegment::Dynamic(#idx)),
//                 });

//                 tokens.append_all(quote! {
//                     TemplateNodeType::Text(TextTemplate::new(&[#(#segments),*], #length))
//                 });
//             }
//             TemplateNodeTypeBuilder::DynamicNode(idx) => tokens.append_all(quote! {
//                 TemplateNodeType::DynamicNode(#idx)
//             }),
//         }
//     }
// }

// struct TemplateNodeBuilder {
//     id: TemplateNodeId,
//     depth: usize,
//     parent: Option<TemplateNodeId>,
//     node_type: TemplateNodeTypeBuilder,
//     fully_static: bool,
// }

// impl TemplateNodeBuilder {
//     #[cfg(any(feature = "hot-reload", debug_assertions))]
//     fn try_into_owned(self, location: &OwnedCodeLocation) -> Result<OwnedTemplateNode, Error> {
//         let TemplateNodeBuilder {
//             id,
//             node_type,
//             parent,
//             depth,
//             ..
//         } = self;
//         let node_type = node_type.try_into_owned(location)?;
//         Ok(OwnedTemplateNode {
//             id,
//             node_type,
//             parent,
//             depth,
//         })
//     }

//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let Self {
//             id,
//             node_type,
//             parent,
//             depth,
//             ..
//         } = self;
//         let raw_id = id.0;
//         let parent = match parent {
//             Some(id) => {
//                 let id = id.0;
//                 quote! {Some(TemplateNodeId(#id))}
//             }
//             None => quote! {None},
//         };

//         tokens.append_all(quote! {
//             TemplateNode {
//                 id: TemplateNodeId(#raw_id),
//                 node_type: #node_type,
//                 parent: #parent,
//                 depth: #depth,
//             }
//         })
//     }
// }

// #[derive(Default)]
// pub struct TemplateBuilder {
//     nodes: Vec<TemplateNodeBuilder>,
//     root_nodes: Vec<TemplateNodeId>,
//     dynamic_context: DynamicTemplateContextBuilder,
// }

// impl TemplateBuilder {
//     /// Create a template builder from nodes if it would improve performance to do so.
//     pub fn from_roots(roots: Vec<BodyNode>) -> Option<Self> {
//         let mut builder = Self::default();

//         for (i, root) in roots.into_iter().enumerate() {
//             let id = builder.build_node(root, None, vec![i], 0);
//             builder.root_nodes.push(id);
//         }

//         // only build a template if there is at least one static node
//         if builder
//             .nodes
//             .iter()
//             .all(|r| matches!(&r.node_type, TemplateNodeTypeBuilder::DynamicNode(_)))
//         {
//             None
//         } else {
//             Some(builder)
//         }
//     }

//     /// Create a template builder from nodes regardless of performance.
//     #[cfg(any(feature = "hot-reload", debug_assertions))]
//     fn from_roots_always(roots: Vec<BodyNode>) -> Self {
//         let mut builder = Self::default();

//         for (i, root) in roots.into_iter().enumerate() {
//             let id = builder.build_node(root, None, vec![i], 0);
//             builder.root_nodes.push(id);
//         }

//         builder
//     }

//     fn build_node(
//         &mut self,
//         node: BodyNode,
//         parent: Option<TemplateNodeId>,
//         path: Vec<usize>,
//         depth: usize,
//     ) -> TemplateNodeId {
//         let id = TemplateNodeId(self.nodes.len());
//         match node {
//             BodyNode::Element(el) => {
//                 let mut locally_static = true;
//                 let mut attributes = Vec::new();
//                 let mut listeners = Vec::new();
//                 for attr in el.attributes {
//                     match attr.attr {
//                         ElementAttr::AttrText { name, value } => {
//                             if let Some(static_value) = value.to_static() {
//                                 attributes.push(TemplateAttributeBuilder {
//                                     element_tag: el.name.clone(),
//                                     name: AttributeName::Ident(name),
//                                     value: TemplateAttributeValue::Static(
//                                         OwnedAttributeValue::Text(static_value),
//                                     ),
//                                 })
//                             } else {
//                                 locally_static = false;
//                                 attributes.push(TemplateAttributeBuilder {
//                                     element_tag: el.name.clone(),
//                                     name: AttributeName::Ident(name),
//                                     value: TemplateAttributeValue::Dynamic(
//                                         self.dynamic_context.add_attr(quote!(#value)),
//                                     ),
//                                 })
//                             }
//                         }
//                         ElementAttr::CustomAttrText { name, value } => {
//                             if let Some(static_value) = value.to_static() {
//                                 attributes.push(TemplateAttributeBuilder {
//                                     element_tag: el.name.clone(),
//                                     name: AttributeName::Str(name),
//                                     value: TemplateAttributeValue::Static(
//                                         OwnedAttributeValue::Text(static_value),
//                                     ),
//                                 })
//                             } else {
//                                 locally_static = false;
//                                 attributes.push(TemplateAttributeBuilder {
//                                     element_tag: el.name.clone(),
//                                     name: AttributeName::Str(name),
//                                     value: TemplateAttributeValue::Dynamic(
//                                         self.dynamic_context.add_attr(quote!(#value)),
//                                     ),
//                                 })
//                             }
//                         }
//                         ElementAttr::AttrExpression { name, value } => {
//                             locally_static = false;
//                             attributes.push(TemplateAttributeBuilder {
//                                 element_tag: el.name.clone(),
//                                 name: AttributeName::Ident(name),
//                                 value: TemplateAttributeValue::Dynamic(
//                                     self.dynamic_context.add_attr(quote!(#value)),
//                                 ),
//                             })
//                         }
//                         ElementAttr::CustomAttrExpression { name, value } => {
//                             locally_static = false;
//                             attributes.push(TemplateAttributeBuilder {
//                                 element_tag: el.name.clone(),
//                                 name: AttributeName::Str(name),
//                                 value: TemplateAttributeValue::Dynamic(
//                                     self.dynamic_context.add_attr(quote!(#value)),
//                                 ),
//                             })
//                         }
//                         ElementAttr::EventTokens { name, tokens } => {
//                             locally_static = false;
//                             listeners.push(self.dynamic_context.add_listener(name, tokens))
//                         }
//                     }
//                 }
//                 if let Some(key) = el.key {
//                     self.dynamic_context.add_key(quote!(
//                         dioxus::core::exports::bumpalo::format!(in __bump, "{}", #key)
//                             .into_bump_str()
//                     ));
//                 }
//                 self.nodes.push(TemplateNodeBuilder {
//                     id,
//                     node_type: TemplateNodeTypeBuilder::Element(TemplateElementBuilder {
//                         tag: el.name,
//                         attributes,
//                         children: Vec::new(),
//                         listeners,
//                         locally_static,
//                     }),
//                     parent,
//                     depth,
//                     fully_static: false,
//                 });
//                 let children: Vec<_> = el
//                     .children
//                     .into_iter()
//                     .enumerate()
//                     .map(|(i, child)| {
//                         let mut new_path = path.clone();
//                         new_path.push(i);
//                         self.build_node(child, Some(id), new_path, depth + 1)
//                     })
//                     .collect();
//                 let children_fully_static = children.iter().all(|c| self.nodes[c.0].fully_static);
//                 let parent = &mut self.nodes[id.0];
//                 parent.fully_static = locally_static && children_fully_static;
//                 if let TemplateNodeTypeBuilder::Element(element) = &mut parent.node_type {
//                     element.children = children;
//                 }
//             }

//             BodyNode::Component(comp) => {
//                 self.nodes.push(TemplateNodeBuilder {
//                     id,
//                     node_type: TemplateNodeTypeBuilder::DynamicNode(
//                         self.dynamic_context.add_node(BodyNode::Component(comp)),
//                     ),
//                     parent,
//                     depth,
//                     fully_static: false,
//                 });
//             }

//             BodyNode::Text(txt) => {
//                 let mut segments = Vec::new();
//                 let mut length = 0;
//                 let mut fully_static = true;

//                 for segment in txt.segments {
//                     segments.push(match segment {
//                         Segment::Literal(lit) => {
//                             length += lit.len();
//                             TextTemplateSegment::Static(lit)
//                         }
//                         Segment::Formatted(fmted) => {
//                             fully_static = false;
//                             TextTemplateSegment::Dynamic(self.dynamic_context.add_text(fmted))
//                         }
//                     })
//                 }

//                 self.nodes.push(TemplateNodeBuilder {
//                     id,
//                     node_type: TemplateNodeTypeBuilder::Text(TextTemplate::new(segments, length)),
//                     parent,
//                     depth,
//                     fully_static,
//                 });
//             }

//             BodyNode::RawExpr(expr) => {
//                 self.nodes.push(TemplateNodeBuilder {
//                     id,
//                     node_type: TemplateNodeTypeBuilder::DynamicNode(
//                         self.dynamic_context.add_node(BodyNode::RawExpr(expr)),
//                     ),
//                     parent,
//                     depth,
//                     fully_static: false,
//                 });
//             }
//         }
//         id
//     }

//     #[cfg(any(feature = "hot-reload", debug_assertions))]
//     pub fn try_switch_dynamic_context(
//         mut self,
//         dynamic_context: DynamicTemplateContextBuilder,
//     ) -> Option<Self> {
//         let attribute_mapping: HashMap<String, usize> = dynamic_context
//             .attributes
//             .iter()
//             .enumerate()
//             .map(|(i, ts)| (ts.to_string(), i))
//             .collect();
//         let text_mapping: HashMap<String, usize> = dynamic_context
//             .text
//             .iter()
//             .enumerate()
//             .map(|(i, ts)| (ts.to_token_stream().to_string(), i))
//             .collect();
//         let listener_mapping: HashMap<(String, Expr), usize> = dynamic_context
//             .listeners
//             .iter()
//             .enumerate()
//             .map(|(i, ts)| (ts.clone(), i))
//             .collect();
//         let node_mapping: HashMap<String, usize> = dynamic_context
//             .nodes
//             .iter()
//             .enumerate()
//             .map(|(i, ts)| (ts.to_token_stream().to_string(), i))
//             .collect();

//         for node in &mut self.nodes {
//             match &mut node.node_type {
//                 TemplateNodeTypeBuilder::Element(element) => {
//                     for listener in &mut element.listeners {
//                         *listener =
//                             *listener_mapping.get(&self.dynamic_context.listeners[*listener])?;
//                     }
//                     for attribute in &mut element.attributes {
//                         if let TemplateAttributeValue::Dynamic(idx) = &mut attribute.value {
//                             *idx = *attribute_mapping
//                                 .get(&self.dynamic_context.attributes[*idx].to_string())?;
//                         }
//                     }
//                 }
//                 TemplateNodeTypeBuilder::Text(txt) => {
//                     for seg in &mut txt.segments {
//                         if let TextTemplateSegment::Dynamic(idx) = seg {
//                             *idx = *text_mapping.get(
//                                 &self.dynamic_context.text[*idx]
//                                     .to_token_stream()
//                                     .to_string(),
//                             )?;
//                         }
//                     }
//                 }
//                 TemplateNodeTypeBuilder::DynamicNode(idx) => {
//                     *idx = *node_mapping.get(
//                         &self.dynamic_context.nodes[*idx]
//                             .to_token_stream()
//                             .to_string(),
//                     )?;
//                 }
//             }
//         }
//         self.dynamic_context = dynamic_context;

//         Some(self)
//     }

//     #[cfg(any(feature = "hot-reload", debug_assertions))]
//     pub fn try_into_owned(self, location: &OwnedCodeLocation) -> Result<OwnedTemplate, Error> {
//         let mut nodes = Vec::new();
//         let dynamic_mapping = self.dynamic_mapping(&nodes);
//         let dynamic_path = self.dynamic_path();
//         for node in self.nodes {
//             nodes.push(node.try_into_owned(location)?);
//         }

//         Ok(OwnedTemplate {
//             nodes,
//             root_nodes: self.root_nodes,
//             dynamic_mapping,
//             dynamic_path,
//         })
//     }

//     #[cfg(any(feature = "hot-reload", debug_assertions))]
//     pub fn dynamic_mapping(
//         &self,
//         resolved_nodes: &Vec<OwnedTemplateNode>,
//     ) -> OwnedDynamicNodeMapping {
//         let dynamic_context = &self.dynamic_context;
//         let mut node_mapping = vec![None; dynamic_context.nodes.len()];
//         let nodes = &self.nodes;
//         for n in nodes {
//             if let TemplateNodeTypeBuilder::DynamicNode(idx) = &n.node_type {
//                 node_mapping[*idx] = Some(n.id)
//             }
//         }
//         let mut text_mapping = vec![Vec::new(); dynamic_context.text.len()];
//         for n in nodes {
//             if let TemplateNodeTypeBuilder::Text(txt) = &n.node_type {
//                 for seg in &txt.segments {
//                     match seg {
//                         TextTemplateSegment::Static(_) => (),
//                         TextTemplateSegment::Dynamic(idx) => text_mapping[*idx].push(n.id),
//                     }
//                 }
//             }
//         }
//         let mut attribute_mapping = vec![Vec::new(); dynamic_context.attributes.len()];
//         for n in nodes {
//             if let TemplateNodeTypeBuilder::Element(el) = &n.node_type {
//                 for (i, attr) in el.attributes.iter().enumerate() {
//                     match attr.value {
//                         TemplateAttributeValue::Static(_) => (),
//                         TemplateAttributeValue::Dynamic(idx) => {
//                             attribute_mapping[idx].push((n.id, i));
//                         }
//                     }
//                 }
//             }
//         }
//         let mut listener_mapping = Vec::new();
//         for n in nodes {
//             if let TemplateNodeTypeBuilder::Element(el) = &n.node_type {
//                 if !el.listeners.is_empty() {
//                     listener_mapping.push(n.id);
//                 }
//             }
//         }

//         let mut volatile_mapping = Vec::new();
//         for n in resolved_nodes {
//             if let TemplateNodeType::Element(el) = &n.node_type {
//                 for (i, attr) in el.attributes.iter().enumerate() {
//                     if attr.attribute.volatile {
//                         volatile_mapping.push((n.id, i));
//                     }
//                 }
//             }
//         }

//         OwnedDynamicNodeMapping::new(
//             node_mapping,
//             text_mapping,
//             attribute_mapping,
//             volatile_mapping,
//             listener_mapping,
//         )
//     }

//     fn dynamic_path(&self) -> Option<OwnedPathSeg> {
//         let mut last_seg: Option<OwnedPathSeg> = None;
//         let mut nodes_to_insert_after = Vec::new();
//         // iterating from the last root to the first
//         for root in self.root_nodes.iter().rev() {
//             let root_node = &self.nodes[root.0];
//             if let TemplateNodeTypeBuilder::DynamicNode(_) = root_node.node_type {
//                 match &mut last_seg {
//                     // if there has been no static nodes, we can append the child to the parent node
//                     None => nodes_to_insert_after.push(*root),
//                     // otherwise we insert the child before the last static node
//                     Some(seg) => {
//                         seg.ops.push(UpdateOp::InsertBefore(*root));
//                     }
//                 }
//             } else if let Some(mut new) = self.construct_path_segment(*root) {
//                 if let Some(last) = last_seg.take() {
//                     match new.traverse {
//                         OwnedTraverse::Halt => {
//                             new.traverse = OwnedTraverse::NextSibling(Box::new(last));
//                         }
//                         OwnedTraverse::FirstChild(b) => {
//                             new.traverse = OwnedTraverse::Both(Box::new((*b, last)));
//                         }
//                         _ => unreachable!(),
//                     }
//                 } else {
//                     for node in nodes_to_insert_after.drain(..) {
//                         new.ops.push(UpdateOp::InsertAfter(node));
//                     }
//                 }
//                 last_seg = Some(new);
//             } else if let Some(last) = last_seg.take() {
//                 last_seg = Some(OwnedPathSeg {
//                     ops: Vec::new(),
//                     traverse: OwnedTraverse::NextSibling(Box::new(last)),
//                 });
//             }
//         }
//         last_seg
//     }

//     fn construct_path_segment(&self, node_id: TemplateNodeId) -> Option<OwnedPathSeg> {
//         let n = &self.nodes[node_id.0];
//         if n.fully_static {
//             return None;
//         }
//         match &n.node_type {
//             TemplateNodeTypeBuilder::Element(el) => {
//                 let mut last_seg: Option<OwnedPathSeg> = None;
//                 let mut children_to_append = Vec::new();
//                 // iterating from the last child to the first
//                 for child in el.children.iter().rev() {
//                     let child_node = &self.nodes[child.0];
//                     if let TemplateNodeTypeBuilder::DynamicNode(_) = child_node.node_type {
//                         match &mut last_seg {
//                             // if there has been no static nodes, we can append the child to the parent node
//                             None => children_to_append.push(*child),
//                             // otherwise we insert the child before the last static node
//                             Some(seg) => {
//                                 seg.ops.push(UpdateOp::InsertBefore(*child));
//                             }
//                         }
//                     } else if let Some(mut new) = self.construct_path_segment(*child) {
//                         if let Some(last) = last_seg.take() {
//                             match new.traverse {
//                                 OwnedTraverse::Halt => {
//                                     new.traverse = OwnedTraverse::NextSibling(Box::new(last));
//                                 }
//                                 OwnedTraverse::FirstChild(b) => {
//                                     new.traverse = OwnedTraverse::Both(Box::new((*b, last)));
//                                 }
//                                 _ => unreachable!(),
//                             }
//                         }
//                         last_seg = Some(new);
//                     } else if let Some(last) = last_seg.take() {
//                         last_seg = Some(OwnedPathSeg {
//                             ops: Vec::new(),
//                             traverse: OwnedTraverse::NextSibling(Box::new(last)),
//                         });
//                     }
//                 }
//                 let mut ops = Vec::new();
//                 if !el.locally_static || n.parent.is_none() {
//                     ops.push(UpdateOp::StoreNode(node_id));
//                 }
//                 for child in children_to_append.into_iter().rev() {
//                     ops.push(UpdateOp::AppendChild(child));
//                 }
//                 Some(OwnedPathSeg {
//                     ops,
//                     traverse: match last_seg {
//                         Some(last) => OwnedTraverse::FirstChild(Box::new(last)),
//                         None => OwnedTraverse::Halt,
//                     },
//                 })
//             }
//             TemplateNodeTypeBuilder::Text(_) => Some(OwnedPathSeg {
//                 ops: vec![UpdateOp::StoreNode(n.id)],
//                 traverse: dioxus_core::OwnedTraverse::Halt,
//             }),
//             TemplateNodeTypeBuilder::DynamicNode(_) => unreachable!(
//                 "constructing path segment for dynamic nodes is handled in the parent node"
//             ),
//         }
//     }
// }

// impl ToTokens for TemplateBuilder {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let Self {
//             nodes,
//             root_nodes,
//             dynamic_context,
//         } = self;

//         let mut node_mapping = vec![None; dynamic_context.nodes.len()];
//         for n in nodes {
//             if let TemplateNodeTypeBuilder::DynamicNode(idx) = &n.node_type {
//                 node_mapping[*idx] = Some(n.id);
//             }
//         }
//         let mut text_mapping = vec![Vec::new(); dynamic_context.text.len()];
//         for n in nodes {
//             if let TemplateNodeTypeBuilder::Text(txt) = &n.node_type {
//                 for seg in &txt.segments {
//                     match seg {
//                         TextTemplateSegment::Static(_) => (),
//                         TextTemplateSegment::Dynamic(idx) => text_mapping[*idx].push(n.id),
//                     }
//                 }
//             }
//         }
//         let mut attribute_mapping = vec![Vec::new(); dynamic_context.attributes.len()];
//         for n in nodes {
//             if let TemplateNodeTypeBuilder::Element(el) = &n.node_type {
//                 for (i, attr) in el.attributes.iter().enumerate() {
//                     match attr.value {
//                         TemplateAttributeValue::Static(_) => (),
//                         TemplateAttributeValue::Dynamic(idx) => {
//                             attribute_mapping[idx].push((n.id, i));
//                         }
//                     }
//                 }
//             }
//         }
//         let mut listener_mapping = Vec::new();
//         for n in nodes {
//             if let TemplateNodeTypeBuilder::Element(el) = &n.node_type {
//                 if !el.listeners.is_empty() {
//                     listener_mapping.push(n.id);
//                 }
//             }
//         }

//         let root_nodes = root_nodes.iter().map(|id| {
//             let raw = id.0;
//             quote! { TemplateNodeId(#raw) }
//         });
//         let node_mapping_quoted = node_mapping.iter().map(|op| match op {
//             Some(id) => {
//                 let raw_id = id.0;
//                 quote! {Some(TemplateNodeId(#raw_id))}
//             }
//             None => quote! {None},
//         });
//         let text_mapping_quoted = text_mapping.iter().map(|inner| {
//             let raw = inner.iter().map(|id| id.0);
//             quote! {&[#(TemplateNodeId(#raw)),*]}
//         });
//         let attribute_mapping_quoted = attribute_mapping.iter().map(|inner| {
//             let raw = inner.iter().map(|(id, _)| id.0);
//             let indecies = inner.iter().map(|(_, idx)| idx);
//             quote! {&[#((TemplateNodeId(#raw), #indecies)),*]}
//         });
//         let listener_mapping_quoted = listener_mapping.iter().map(|id| {
//             let raw = id.0;
//             quote! {TemplateNodeId(#raw)}
//         });
//         let mut nodes_quoted = TokenStream::new();
//         for n in nodes {
//             n.to_tokens(&mut nodes_quoted);
//             quote! {,}.to_tokens(&mut nodes_quoted);
//         }

//         let dynamic_path = match self.dynamic_path() {
//             Some(seg) => {
//                 let seg = quote_owned_segment(seg);
//                 quote! {Some(#seg)}
//             }
//             None => quote! {None},
//         };

//         let quoted = quote! {
//             {
//                 const __NODES: dioxus::prelude::StaticTemplateNodes = &[#nodes_quoted];
//                 const __TEXT_MAPPING: &'static [&'static [dioxus::prelude::TemplateNodeId]] = &[#(#text_mapping_quoted),*];
//                 const __ATTRIBUTE_MAPPING: &'static [&'static [(dioxus::prelude::TemplateNodeId, usize)]] = &[#(#attribute_mapping_quoted),*];
//                 const __ROOT_NODES: &'static [dioxus::prelude::TemplateNodeId] = &[#(#root_nodes),*];
//                 const __NODE_MAPPING: &'static [Option<dioxus::prelude::TemplateNodeId>] = &[#(#node_mapping_quoted),*];
//                 const __NODES_WITH_LISTENERS: &'static [dioxus::prelude::TemplateNodeId] = &[#(#listener_mapping_quoted),*];
//                 static __VOLITALE_MAPPING_INNER: dioxus::core::exports::once_cell::sync::Lazy<Vec<(dioxus::prelude::TemplateNodeId, usize)>> = dioxus::core::exports::once_cell::sync::Lazy::new(||{
//                     // check each property to see if it is volatile
//                     let mut volatile = Vec::new();
//                     for n in __NODES {
//                         if let TemplateNodeType::Element(el) = &n.node_type {
//                             for (i, attr) in el.attributes.iter().enumerate() {
//                                 if attr.attribute.volatile {
//                                     volatile.push((n.id, i));
//                                 }
//                             }
//                         }
//                     }
//                     volatile
//                 });
//                 static __VOLITALE_MAPPING: &'static dioxus::core::exports::once_cell::sync::Lazy<Vec<(dioxus::prelude::TemplateNodeId, usize)>> = &__VOLITALE_MAPPING_INNER;
//                 static __STATIC_VOLITALE_MAPPING: dioxus::prelude::LazyStaticVec<(dioxus::prelude::TemplateNodeId, usize)> = LazyStaticVec(__VOLITALE_MAPPING);
//                 static __TEMPLATE: dioxus::prelude::Template = Template::Static(&StaticTemplate {
//                     nodes: __NODES,
//                     root_nodes: __ROOT_NODES,
//                     dynamic_mapping: StaticDynamicNodeMapping::new(__NODE_MAPPING, __TEXT_MAPPING, __ATTRIBUTE_MAPPING, __STATIC_VOLITALE_MAPPING, __NODES_WITH_LISTENERS),
//                     dynamic_path: #dynamic_path,
//                 });

//                 let __bump = __cx.bump();
//                 __cx.template_ref(dioxus::prelude::TemplateId(get_line_num!()), __TEMPLATE.clone(), #dynamic_context)
//             }
//         };

//         tokens.append_all(quoted)
//     }
// }

// #[derive(Default, Clone, Debug)]
// pub struct DynamicTemplateContextBuilder {
//     nodes: Vec<BodyNode>,
//     text: Vec<FormattedSegment>,
//     attributes: Vec<TokenStream>,
//     listeners: Vec<(String, Expr)>,
//     key: Option<TokenStream>,
// }

// impl DynamicTemplateContextBuilder {
//     fn add_node(&mut self, node: BodyNode) -> usize {
//         let node_id = self.nodes.len();

//         self.nodes.push(node);

//         node_id
//     }

//     fn add_text(&mut self, text: FormattedSegment) -> usize {
//         let text_id = self.text.len();

//         self.text.push(text);

//         text_id
//     }

//     fn add_attr(&mut self, attr: TokenStream) -> usize {
//         let attr_id = self.attributes.len();

//         self.attributes.push(attr);

//         attr_id
//     }

//     fn add_listener(&mut self, name: Ident, listener: Expr) -> usize {
//         let listener_id = self.listeners.len();

//         self.listeners.push((name.to_string(), listener));

//         listener_id
//     }

//     fn add_key(&mut self, key: TokenStream) {
//         self.key = Some(key);
//     }
// }

// impl ToTokens for DynamicTemplateContextBuilder {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let nodes = &self.nodes;
//         let text = &self.text;
//         let attributes = &self.attributes;
//         let listeners_names = self
//             .listeners
//             .iter()
//             .map(|(n, _)| syn::parse_str::<Ident>(n).expect(n));
//         let listeners_exprs = self.listeners.iter().map(|(_, e)| e);
//         let key = match &self.key {
//             Some(k) => quote!(Some(#k)),
//             None => quote!(None),
//         };
//         tokens.append_all(quote! {
//             TemplateContext {
//                 nodes: __cx.bump().alloc([#(#nodes),*]),
//                 text_segments: __cx.bump().alloc([#(&*dioxus::core::exports::bumpalo::format!(in __bump, "{}", #text).into_bump_str()),*]),
//                 attributes: __cx.bump().alloc([#({#attributes}.into_value(__cx.bump())),*]),
//                 listeners: __cx.bump().alloc([#(dioxus_elements::on::#listeners_names(__cx, #listeners_exprs)),*]),
//                 key: #key,
//             }
//         })
//     }
// }

// fn quote_owned_segment(seg: OwnedPathSeg) -> proc_macro2::TokenStream {
//     let OwnedPathSeg { ops, traverse } = seg;

//     let ops = ops
//         .into_iter()
//         .map(|op| match op {
//             UpdateOp::StoreNode(id) => {
//                 let id = quote_template_node_id(id);
//                 quote!(UpdateOp::StoreNode(#id))
//             }
//             UpdateOp::InsertBefore(id) => {
//                 let id = quote_template_node_id(id);
//                 quote!(UpdateOp::InsertBefore(#id))
//             }
//             UpdateOp::InsertAfter(id) => {
//                 let id = quote_template_node_id(id);
//                 quote!(UpdateOp::InsertAfter(#id))
//             }
//             UpdateOp::AppendChild(id) => {
//                 let id = quote_template_node_id(id);
//                 quote!(UpdateOp::AppendChild(#id))
//             }
//         })
//         .collect::<Vec<_>>();

//     let traverse = quote_owned_traverse(traverse);

//     quote! {
//         StaticPathSeg {
//             ops: &[#(#ops),*],
//             traverse: #traverse,
//         }
//     }
// }

// fn quote_owned_traverse(traverse: OwnedTraverse) -> proc_macro2::TokenStream {
//     match traverse {
//         OwnedTraverse::Halt => {
//             quote! {StaticTraverse::Halt}
//         }
//         OwnedTraverse::FirstChild(seg) => {
//             let seg = quote_owned_segment(*seg);
//             quote! {StaticTraverse::FirstChild(&#seg)}
//         }
//         OwnedTraverse::NextSibling(seg) => {
//             let seg = quote_owned_segment(*seg);
//             quote! {StaticTraverse::NextSibling(&#seg)}
//         }
//         OwnedTraverse::Both(b) => {
//             let (child, sibling) = *b;
//             let child = quote_owned_segment(child);
//             let sibling = quote_owned_segment(sibling);
//             quote! {StaticTraverse::Both(&(#child, #sibling))}
//         }
//     }
// }

// fn quote_template_node_id(id: TemplateNodeId) -> proc_macro2::TokenStream {
//     let raw = id.0;
//     quote! {
//         TemplateNodeId(#raw)
//     }
// }