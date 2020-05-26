use exitfailure::ExitFailure;
use failure::Error;
use i3ipc::{
    event::Event,
    reply::{Node, NodeLayout},
    I3Connection, I3EventListener, Subscription,
};

fn switch_splitting(i3_conn: &mut I3Connection, node: Node) -> Result<(), Error> {

    // Only toggle split on focused, tiled windows
    if !should_toggle(&node) {
        return Ok(());
    }

    let width = node.rect.2;
    let height = node.rect.3;

    let new_layout = if height > width {
        NodeLayout::SplitV
    } else {
        NodeLayout::SplitH
    };

    let command = format!("{:?}", new_layout);
    let result = i3_conn.run_command(&command)?;

    if !result.outcomes[0].success {
        eprintln!("Error: Switch failed with err {:?}", result);
    }

    Ok(())
}

fn should_toggle(node: &Node) -> bool {
    let layout = &node.layout;
    node.focused && (*layout == NodeLayout::SplitV || *layout == NodeLayout::SplitH)
}

fn main() -> Result<(), ExitFailure> {
    let mut listener = I3EventListener::connect()?;
    let subs = [Subscription::Window];
    listener.subscribe(&subs)?;

    let mut i3_conn = I3Connection::connect()?;

    for event in listener.listen() {
        match event? {
            Event::WindowEvent(e) => {
                if let Err(error) = switch_splitting(&mut i3_conn, e.container) {
                    eprintln!("switch_splitting error: {}", error);
                }
            },
            _ => {},
        }
    }

    Ok(())
}
