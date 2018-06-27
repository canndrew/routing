// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use super::{TestNode, create_connected_nodes, poll_all, verify_invariant_for_all_nodes};
use routing::{Event, EventStream};
use routing::mock_crust::Network;

// Drop node at index and verify its own section receives NodeLost.
fn drop_node(nodes: &mut Vec<TestNode>, index: usize) {
    let node = nodes.remove(index);
    let name = node.name();
    let close_names = node.close_names();

    drop(node);

    let _ = poll_all(nodes, &mut []);

    for node in nodes.iter_mut().filter(|n| close_names.contains(&n.name())) {
        loop {
            match node.try_next_ev() {
                Ok(Event::NodeLost(lost_name, _)) if lost_name == name => break,
                Ok(_) => (),
                _ => panic!("Event::NodeLost({:?}) not received", name),
            }
        }
    }
}

#[test]
fn node_drops() {
    let group_size = 8;
    let network = Network::new(group_size, None);
    let mut nodes = create_connected_nodes(&network, group_size + 2);
    drop_node(&mut nodes, 0);

    verify_invariant_for_all_nodes(&mut nodes);
}

#[test]
fn node_restart() {
    // Idea of test: if a node disconnects from all other nodes, it should restart
    // (with the exception of the first node which is special).
    let group_size = 5;
    let network = Network::new(group_size, None);
    let mut nodes = create_connected_nodes(&network, group_size);

    // Drop all but last node:
    while nodes.len() > 1 {
        drop_node(&mut nodes, 0);
    }

    let _ = poll_all(&mut nodes, &mut []);

    expect_next_event!(nodes[0], Event::RestartRequired);
}
