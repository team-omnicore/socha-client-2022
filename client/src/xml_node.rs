use crate::game::Game;
use crate::team::Team;
use game_lib::board::Board;
use game_lib::gamestate::Gamestate;
use game_lib::piece::PieceType;
use std::collections::{HashMap, VecDeque};
use std::io::BufReader;
use std::net::TcpStream;
use xml::reader::XmlEvent;
use xml::EventReader;

#[derive(Debug)]
pub struct XmlNode {
    pub name: String,
    pub data: String,
    pub attributes: HashMap<String, Vec<String>>,
    pub children: Vec<XmlNode>,
}

impl XmlNode {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            data: String::new(),
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn child(&self, name: &str) -> Option<&XmlNode> {
        for child in self.children.iter() {
            if child.name.as_str() == name {
                return Some(child);
            }
        }
        None
    }

    pub fn read_from(xml_parser: &mut EventReader<BufReader<&TcpStream>>) -> Self {
        let mut node_stack: VecDeque<XmlNode> = VecDeque::new();
        let mut has_received_first = false;
        let mut final_node: Option<XmlNode> = None;

        loop {
            match xml_parser.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let mut node = XmlNode::new();
                    node.name = name.local_name;
                    for attribute in attributes {
                        let attrib_name = attribute.name.local_name;
                        if !node.attributes.contains_key(&attrib_name) {
                            node.attributes.insert(attrib_name.to_string(), Vec::new());
                        }
                        node.attributes
                            .get_mut(&attrib_name)
                            .unwrap()
                            .push(attribute.value.to_string());
                    }
                    node_stack.push_back(node);
                    has_received_first = true;
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    if node_stack.len() > 2 {
                        let child = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to pop off new child element");
                        let mut node = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to hook up new child element");
                        node.children.push(child);
                        node_stack.push_back(node);
                    } else if has_received_first {
                        final_node = Some(node_stack.pop_back().expect(
                            "Unexpectedly found empty XML node stack while trying to return node",
                        ));
                    }
                }
                Ok(XmlEvent::Characters(content)) => {
                    node_stack.back_mut().expect("Unexpectedly found empty XML node stack while trying to add characters").data += content.as_str();
                }
                Err(_) => {
                    break;
                }
                _ => {}
            }
            if final_node.is_some() {
                break;
            }
        }
        final_node.unwrap()
    }
}

pub struct XmlState<'a>(pub &'a XmlNode, pub Team);

impl<'a> From<&'a XmlState<'a>> for Gamestate {
    fn from(xml_state: &'a XmlState<'a>) -> Self {
        let node = &xml_state.0;
        let client_team = xml_state.1;

        let mut gamestate = Gamestate::new();
        let turn: u8 = node
            .attributes
            .get("turn")
            .unwrap()
            .get(0)
            .unwrap()
            .parse()
            .unwrap();
        gamestate.round = turn;

        let ambers = node.child("ambers").expect("No amber node available");
        for child in &ambers.children {
            let team = child
                .child("team")
                .expect("No score for team present")
                .clone()
                .data
                .parse::<Team>()
                .expect("Failed to parse team");

            let points = child
                .child("int")
                .expect("No points present")
                .data
                .parse::<u8>()
                .expect("Failed to parse points");

            if client_team == team {
                gamestate.score.bytes[0] = points;
            } else {
                gamestate.score.bytes[1] = points;
            }
        }

        let board_node = node.child("board").unwrap();
        gamestate.board = Board::from(&XmlState(&board_node, client_team));

        gamestate
    }
}

impl<'a> From<&XmlState<'a>> for Board {
    fn from(xml_state: &XmlState) -> Self {
        let client_team = xml_state.1;
        let node = &xml_state.0;

        let mut board = Board::new();
        let pieces = node.child("pieces").unwrap();

        for entry in &pieces.children {
            let coordinates = entry.child("coordinates").unwrap();
            let x = coordinates
                .attributes
                .get("x")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<u8>()
                .expect("Failed to parse coordinates while deserializing");

            let y = coordinates
                .attributes
                .get("y")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<u8>()
                .expect("Failed to parse coordinates while deserializing");

            let piece_node = entry.child("piece").unwrap();

            let piece_type = PieceType::from(
                piece_node
                    .attributes
                    .get("type")
                    .unwrap()
                    .get(0)
                    .expect("Failed to match piece"),
            );

            let piece_team = piece_node
                .attributes
                .get("team")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<Team>()
                .expect("Failed to parse Team while deserializing");

            let stacked = match piece_node
                .attributes
                .get("count")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<u8>()
            {
                Ok(2) => true,
                _ => false,
            };

            let sq = 8 * y + (7 - x);
            let pos = (((sq >> 3) | (sq << 3)) & 63) ^ 7; //(((sq >> 3) | (sq << 3)) & 63) ^ 56;

            board.set_piece(pos, piece_type, client_team == piece_team, stacked);
        }

        if client_team == Team::ONE {
            board.rotate180();
        }
        board
    }
}
