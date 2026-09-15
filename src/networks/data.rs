use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use sysinfo::{InterfaceOperationalState, MacAddr, NetworkData, Networks};
use ratatui::widgets::ListItem;

pub struct InterfaceIp {
    pub addr: IpAddr,
    pub prefix: u8,
}

pub struct SortedAddresses {
    pub ipv4: Vec<(Ipv4Addr, u8)>,
    pub ipv6: Vec<(Ipv6Addr, u8)>,
}

pub struct NetworksList {
    pub state: InterfaceOperationalState,
    pub mac: MacAddr,
    pub interface_name: String,
    pub adresses: Vec<InterfaceIp>,
    pub bytes_download_speed: f64,
    pub bytes_upload_speed: f64,
    pub bytes_download_total: f64,
    pub bytes_upload_total: f64,
    pub packets_download_speed: u64,
    pub packets_upload_speed: u64,
    pub packets_download_total: u64,
    pub packets_upload_total: u64,
    pub errors_on_download: u64,
    pub errors_on_upload: u64,
    pub errors_on_download_total: u64,
    pub errors_on_upload_total: u64,
}

impl NetworksList {
    pub fn update(networks: &Networks) -> Vec<Self> {
        let mut list: Vec<Self> = Vec::new();

        for (interface_name, interface_data) in networks {
            let network_info = Self {
                state: interface_data.operational_state(),
                mac: interface_data.mac_address(),
                interface_name: interface_name.to_string(),
                adresses: InterfaceIp::addresses(interface_data),
                bytes_download_speed: interface_data.received() as f64,
                bytes_upload_speed: interface_data.transmitted() as f64,
                bytes_download_total: interface_data.total_received() as f64,
                bytes_upload_total: interface_data.total_transmitted() as f64,
                packets_download_speed: interface_data.packets_received(),
                packets_upload_speed: interface_data.packets_transmitted(),
                packets_download_total: interface_data.total_packets_received(),
                packets_upload_total: interface_data.total_packets_transmitted(),
                errors_on_download: interface_data.errors_on_received(),
                errors_on_upload: interface_data.errors_on_transmitted(),
                errors_on_download_total: interface_data.total_errors_on_received(),
                errors_on_upload_total: interface_data.total_errors_on_transmitted(),
            };

            list.push(network_info);
        }

        list
    }
}

impl InterfaceIp {
    pub fn addresses(data: &NetworkData) -> Vec<Self> {
        data.ip_networks()
            .iter()
            .map(|network| Self {
                addr: network.addr,
                prefix: network.prefix,
            })
            .collect()
    }
}

// helper function to sort IPv4 & IPv6 addresses
pub fn sort_addresses(addresses: &Vec<InterfaceIp>) -> SortedAddresses {
    let mut ipv4_list: Vec<(Ipv4Addr, u8)> = Vec::new();
    let mut ipv6_list: Vec<(Ipv6Addr, u8)> = Vec::new();

    for address in addresses {
        match address.addr {
            IpAddr::V4(ipv4_parsed) => {
                ipv4_list.push((ipv4_parsed, address.prefix));
            }
            IpAddr::V6(ipv6_parsed) => {
                ipv6_list.push((ipv6_parsed, address.prefix));
            }
        }
    }

    if ipv4_list.is_empty() {
        ipv4_list.push((Ipv4Addr::UNSPECIFIED, 0));
    }
    if ipv6_list.is_empty() {
        ipv6_list.push((Ipv6Addr::UNSPECIFIED, 0));
    }

    SortedAddresses {
        ipv4: ipv4_list,
        ipv6: ipv6_list,
    }
}

pub fn list_items<'a>(networks: &Vec<NetworksList>) -> Vec<ListItem<'a>> {
    let mut list: Vec<ListItem> = Vec::new();

    for interface in networks {
        list.push(ListItem::new(interface.interface_name.clone()));
    }

    list
}
