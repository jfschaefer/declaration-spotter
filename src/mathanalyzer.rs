use libxml::tree::Node;


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IdentifierTags {
    First,
    FirstSeq,
    RelSeq,   // relations between identifiers, e.g. x1 < x2 < ... < xn
    Ellipsis,
}

#[derive(Clone)]
pub struct Identifier {
    pub node : Node,
    pub tags : Vec<IdentifierTags>,
}



pub fn find_potential_identifiers(math_node : Node) -> Vec<Identifier> {
    let mut results : Vec <Identifier> = Vec::new();

    assert_eq!(math_node.get_name(), "math");
    match get_first_identifier(math_node) {
        Some(x) => results.push(Identifier { node: x, tags: vec![IdentifierTags::First] } ),
        None => { }
    }

    match get_first_identifier_seq(math_node, None) {
        None => { }
        Some(nodes, separator) => {
            if nodes.len() > 1 {  // sequence has to have at least two elements ;)
                // check if \ldots was used:
                let mut ellipsis: bool = false;
                let mut pos = 0usize;
                while pos < nodes.len() {
                    if nodes[pos] == "\u{2026}" {
                        ellipsis = true;
                        nodes.remove[pos];
                        // don#t break, possibly multiple ellipses
                    } else {
                        pos += 1;
                    }
                    let mut tags = vec![IdentifierTags::FirstSeq];
                    if ellipsis {
                        tags.push(IdentifierTags::Ellipsis);
                    }
                    if separator != "," && separator != "" {
                        tags.push(IdentifierTags::RelSeq);
                    }
                    for n in nodes {
                        results.push(Identifier { node: n, tags: tags.clone() });
                    }
                }

            }
        }
    }
    
    results
}

fn get_first_identifier_seq(root_opt: Option<Node>, sep: Option<&str>) -> Option<(Vec<Node>, String)> {
    if root_opt.is_none() {
        return None;
    }
    let root = root_opt.unwrap();
    match &root.get_name() as &str { "mtext" => None,
        "annotation" | "xml-annotation" => None,
        "mfrac" | "mtable" => None,

        "mi" => 
            match get_first_identifier_seq(root.get_next_sibling(), sep) {
                Some((mut v, s)) => { v.push(root); Some((v, s)) },
                None => Some((vec![root], if sep.is_none() { "" } else { sep.unwrap() }.to_owned())),
            },
        "msub" | "msup" | "msubsup" => 
            match get_first_identifier_seq_helper(root.clone()) {
                Some(_) =>
                    match get_first_identifier_seq(root.get_next_sibling(), sep) {
                        Some((mut v, s)) => { v.push(root); Some((v, s)) },
                        // None => Some(vec![root]),
                        None => Some((vec![root], if sep.is_none() { "" } else { sep.unwrap() }.to_owned())),
                    },
                None => None,
            },
        "mo" => 
            match &root.get_content() as &str {
                "(" | "" | /* "\u{2026}" /* \ldots */ | */ "\u{27E8}" /* \langle */ =>
                    get_first_identifier_seq(root.get_next_sibling(), sep),
                "=" => None,
                x => match sep {
                    None => get_first_identifier_seq(root.get_next_sibling(), Some(x)),
                    Some(y) => if x == y {
                        get_first_identifier_seq(root.get_next_sibling(), Some(x))
                    } else { None },
                },
            },
        _ => get_first_identifier_seq(root.get_first_child(), sep),
    }
}


fn get_first_identifier_seq_helper(root: Node) -> Option<Node> {
    match root.get_first_child() {
        None => None,
        Some(x) =>
            if get_first_identifier(x).is_some() {
                Some(root)
            } else {
                None
            },
    }
}



fn get_first_identifier(root: Node) -> Option<Node> {
    match &root.get_name() as &str { "mtext" => None,
        "annotation" => None,
        "xml-annotation" => None,
        "mfrac" => None,
        "mtable" => None,
        "mi" => Some(root),
        "msub" => get_first_identifier_helper(root),
        "msup" => get_first_identifier_helper(root),
        "msubsup" => get_first_identifier_helper(root),
        "mo" => if root.get_content() == "(" {
            match root.get_next_sibling() {
                None => None,
                Some(x) => get_first_identifier(x),
            } } else {
                None
            },
        _ => match root.get_first_child() {
            None => None,
            Some(x) => get_first_identifier(x),
        }
    }
}


fn get_first_identifier_helper(root: Node) -> Option<Node> {
    match root.get_first_child() {
        None => None,
        Some(x) =>
            if get_first_identifier(x).is_some() {
                Some(root)
            } else {
                None
            },
    }
}

