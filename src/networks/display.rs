use crate::networks::data::{NetworksList, list_items, sort_addresses};
use crate::ui::size::parse_full_one_decimal;
use ratatui::style::Style;
use ratatui::text::{Text, Line};
use ratatui::widgets::{Block, List, Paragraph};

// helper struct to render details & determine max scroll
pub struct NetworkDetails<'a> {
    pub content: Paragraph<'a>,
    pub max_scroll: u16,
}

pub fn display_list<'a>(networks: &'a Vec<NetworksList>) -> List<'a> {
    let items = list_items(networks);

    List::new(items)
        .block(Block::default())
        .highlight_symbol("> ")
        .highlight_style(Style::new().bold())
}

pub fn display_details<'a>(network: &NetworksList, vh: u16) -> NetworkDetails<'a> {
    let sorted_addresses = sort_addresses(&network.adresses);
    let mut addresses_list = Text::default();

    // general
    addresses_list.push_line(
        Line::from(format!("Operational state: {}", network.state))
    );
    addresses_list.push_line(
        Line::from(format!("MAC: {}", network.mac))
    );
    addresses_list.push_line(Line::from(""));

    // IPv4 list
    addresses_list.push_line(
        Line::from("IPv4:")
    );
    for ipv4 in sorted_addresses.ipv4 {
        addresses_list.push_line(
            Line::from(format!("{}/{}", ipv4.0, ipv4.1))
        );
    }
    addresses_list.push_line(Line::from(""));

    // IPv6 list
    addresses_list.push_line(
        Line::from("IPv6:")
    );
    for ipv6 in sorted_addresses.ipv6 {
        addresses_list.push_line(
            Line::from(format!("{}/{}", ipv6.0, ipv6.1))
        );
    }
    addresses_list.push_line(Line::from(""));

    // bytes received/transmitted
    addresses_list.push_line(
        Line::from("Bytes received/transmitted:")
    );
    addresses_list.push_line(
        Line::from(format!("Download: {}/s (Total: {})",
            parse_full_one_decimal(network.bytes_download_speed), parse_full_one_decimal(network.bytes_download_total)
        ))
    );
    addresses_list.push_line(
        Line::from(format!("Upload:   {}/s (Total: {})",
            parse_full_one_decimal(network.bytes_upload_speed), parse_full_one_decimal(network.bytes_upload_total)
        ))
    );
    addresses_list.push_line(Line::from(""));

    // packets received/transmitted
    addresses_list.push_line(
        Line::from("Packets received/transmitted:")
    );
    addresses_list.push_line(
        Line::from(format!("Download: {}/s (Total: {})",
            network.packets_download_speed, network.packets_download_total
        ))
    );
    addresses_list.push_line(
        Line::from(format!("Upload:   {}/s (Total: {})",
            network.packets_upload_speed, network.packets_upload_total
        ))
    );
    addresses_list.push_line(Line::from(""));

    // errors on receiving/transmitting
    addresses_list.push_line(
        Line::from("Errors on receiving/transmitting:")
    );
    addresses_list.push_line(
        Line::from(format!("Incoming:  {}/s (Total: {})",
            network.errors_on_download, network.errors_on_download_total
        ))
    );
    addresses_list.push_line(
        Line::from(format!("Outcoming: {}/s (Total: {})",
            network.errors_on_upload, network.errors_on_upload_total
        ))
    );

    NetworkDetails {
        content: Paragraph::new(addresses_list.clone()),
        max_scroll: (addresses_list.lines.len() as u16).saturating_sub(vh),
    }
}
