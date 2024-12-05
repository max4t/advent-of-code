use std::{collections::HashMap, rc::Rc};

trait Node {
    fn compute(self: &Self, circuit: &HashMap::<String, Element>, mem: &mut HashMap::<String, u16>) -> u16;
}

#[derive(Debug, Clone)]
enum Element {
    Variable(String),

    Constant(u16),
    Not(Box<Element>),
    And(Box<Element>, Box<Element>),
    Or(Box<Element>, Box<Element>),
    LShift(Box<Element>, Box<Element>),
    RShift(Box<Element>, Box<Element>),
}

impl Element {
    fn new_value(s: &str) -> Self {
        if let Ok(i) = s.trim().parse::<u16>() {
            Element::Constant(i)
        } else {
            Element::Variable(s.trim().to_owned())
        }
    }

    fn dependencies(self: &Self) -> Vec<String> {
        match self {
            Element::Variable(name) => vec![name.clone()],
            Element::Constant(_) => vec![],
            Element::Not(node) => node.dependencies(),
            Element::And(node, node1) => [node.dependencies(), node1.dependencies()].concat(),
            Element::Or(node, node1) => [node.dependencies(), node1.dependencies()].concat(),
            Element::LShift(node, node1) => [node.dependencies(), node1.dependencies()].concat(),
            Element::RShift(node, node1) => [node.dependencies(), node1.dependencies()].concat(),
        }
    }
}

impl Node for Element {
    fn compute(self: &Self, circuit: &HashMap::<String, Element>, mem: &mut HashMap::<String, u16>) -> u16 {
        match self {
            Element::Variable(name) => {
                if mem.get(name).is_none() {
                    let res = circuit.get(name).unwrap().compute(circuit, mem);
                    mem.insert(name.to_string(), res);
                }
                *mem.get(name).unwrap()
                // *mem.entry(name.to_string()).or_insert_with(|| {
                // })
                    // circuit.get_mut(dbg!(name).as_str())
                    //     .unwrap().clone()
                    //     .compute(circuit, mem)
            },
            Element::Constant(i) => *i,
            Element::Not(node) => !node.compute(circuit, mem),
            Element::And(node, node1) => node.compute(circuit, mem) & node1.compute(circuit, mem),
            Element::Or(node, node1) => node.compute(circuit, mem) | node1.compute(circuit, mem),
            Element::LShift(node, node1) => node.compute(circuit, mem) << node1.compute(circuit, mem),
            Element::RShift(node, node1) => node.compute(circuit, mem) >> node1.compute(circuit, mem),
        }
    }
}

pub fn basic_solve(s: &[&[&str]]) -> u16 {
    let instructions = s.iter().map(|&instructs| {
        if instructs[0] == "NOT" {
            (instructs[3].to_owned(), Element::Not(Box::new(Element::new_value(instructs[1]))))
        } else if instructs.len() == 3 {
            (instructs[2].to_owned(), Element::new_value(instructs[0]))
        } else {
            match instructs[1] {
                "AND" => (instructs[4].to_owned(), Element::And(Box::new(Element::new_value(instructs[0])), Box::new(Element::new_value(instructs[2])))),
                "OR" => (instructs[4].to_owned(), Element::Or(Box::new(Element::new_value(instructs[0])), Box::new(Element::new_value(instructs[2])))),
                "LSHIFT" => (instructs[4].to_owned(), Element::LShift(Box::new(Element::new_value(instructs[0])), Box::new(Element::new_value(instructs[2])))),
                "RSHIFT" => (instructs[4].to_owned(), Element::RShift(Box::new(Element::new_value(instructs[0])), Box::new(Element::new_value(instructs[2])))),
                _ => panic!("unknown command"),
            }
        }
    });
    let mut circuit = HashMap::<String, Element>::from_iter(instructions.map(|(a, b)| (a, b)));
    // dbg!(&circuit);
    // dbg!(&circuit["a"]);
    // dbg!(&circuit["lx"]);
    // dbg!(&circuit["lv"]);
    // dbg!(&circuit["e"]);
    // dbg!(&circuit["b"]);
    // dbg!(&circuit["f"]);
    let mut cache = HashMap::<String, u16>::new();

    let res = circuit.get("a").unwrap()
        .compute(&circuit, &mut cache);
    circuit.insert("b".to_string(), Element::Constant(res));
    cache.clear();
    circuit.get("a").unwrap()
        .compute(&circuit, &mut cache)
    // circuit.iter().for_each(|(el, node)| {
    //     for ele in node.dependencies() {
    //         println!("{} -> {};", el, ele)
    //     }
    // });
    // 0
}
