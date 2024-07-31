use crate::prelude::*;
use register_rs::*;

use crate::{registers::*, RadioResult, RadioError};
use crate::constants::official_driver_constants as od_constants;
// use crate::Spirit1;

pub trait SpiritPacketFormats: Spirit1Hal
{
    fn configure_packet_protocol(&mut self, configuration: PacketConfiguration) -> RadioResult<()> {
        match configuration {
            PacketConfiguration::Basic(config) => self.configure_basic(config),
            _ => Err(RadioError::NotImplemented)
        }
    }

    fn configure_basic_filter(&mut self, opts: BasicAddressOpts) -> RadioResult<()> {
        let mut flt_opts: PcktFltOptions = self.read_register();
        flt_opts.dest_vs_source_addr = opts.filter_on_my_address;
        flt_opts.dest_vs_multicast_addr = opts.filter_on_multicast_address;
        flt_opts.dest_vs_broadcast_addr = opts.filter_on_broadcast_address;
        self.write_register(flt_opts)?;

        let mut goals: PcktFltGoals = self.read_register();
        goals.broadcast = opts.broadcast_address;
        goals.multicast = opts.multicast_address;
        goals.tx_source_addr = opts.my_address;
        self.write_register(goals)?;

        Ok(())
    }

    fn configure_basic(&mut self, opts: BasicProtocolOpts) -> RadioResult<()> {
        let mut protocol: Protocol = self.read_register();
        protocol.auto_pckt_flt = true;
        self.write_register(protocol)?;

        let mut flt_opts: PcktFltOptions = self.read_register();
        flt_opts.source_filtering = true;
        flt_opts.control_filtering = true;
        flt_opts.crc_check = if opts.crc_mode == CrcMode::NoCrc { false } else { true };
        self.write_register(flt_opts)?;

        let pkt_ctrl_4 = PcktCtrl4::new(
            if opts.address_field { 1 } else { 0 }, 
            opts.control_length.try_into()?
        );

        let mut pkt_ctrl_3 = PcktCtrl3::reset_value();
        pkt_ctrl_3.len_wid = opts.packet_length_width.max(1);
        pkt_ctrl_3.pckt_frmt = PacketFormat::Basic;

        let pkt_ctrl_2 = PcktCtrl2::new(
            opts.preamble_length.try_into()?,
            opts.sync_length.try_into()?,
            opts.fix_var_length
        );

        let pkt_ctrl_1 = PcktCtrl1::new(
            opts.crc_mode,
            opts.data_whitening,
            TxMode::Normal,
            opts.fec
        );

        self.write_register(pkt_ctrl_4)?;
        self.write_register(pkt_ctrl_3)?;
        self.write_register(pkt_ctrl_2)?;
        self.write_register(pkt_ctrl_1)?;

        self.write_register(Sync1::new(opts.sync_words.0))?;
        self.write_register(Sync2::new(opts.sync_words.1))?;
        self.write_register(Sync3::new(opts.sync_words.2))?;
        self.write_register(Sync4::new(opts.sync_words.3))?;

        Ok(())
    }
}

/// SPIRIT Basic Packet Init structure definition. This structure allows users to set
/// the main options for the Basic packet.
pub struct BasicProtocolOpts {
    /// Specifies the preamble length
    pub preamble_length: PreambleLength,
    /// Specifies the sync word length. The 32bit word passed (lSyncWords)
    /// will be stored in the SYNCx registers from the MSb until the 
    /// number of bytes in xSyncLength has been stored.
    pub sync_length: PacketSyncLength,
    /// Specifies the sync words
    pub sync_words: (u8, u8, u8, u8),
    /// Specifies if a fixed length of packet has to be used
    pub fix_var_length: PacketLengthMode,
    /// Specifies the size of the length of packet in bits.
    /// 
    /// This field is useful only if the field xFixVarLength is set to `BASIC_LENGTH_VAR`.
    /// 
    /// For Basic packets the length width is 
    /// log2( max payload length + control length (0 to 4) + address length (0 or 1)).
    pub packet_length_width: u8,
    /// Specifies the CRC word length of packet
    pub crc_mode: CrcMode,
    /// Specifies the length of a control field to be sent
    pub control_length: PacketControlLength,
    /// Specifies if the destination address has to be sent
    pub address_field: bool,
    /// Specifies if FEC has to be enabled
    pub fec: bool,
    /// Specifies if data whitening has to be enabled
    pub data_whitening: bool
}

/// SPIRIT Basic Packet address structure definition. This structure allows users to specify
/// the node/multicast/broadcast addresses and the correspondent filtering options.
pub struct BasicAddressOpts {
    /// If set RX packet is accepted if its destination address matches with `my_address`
    pub filter_on_my_address: bool,
    /// Specifies the TX packet source address (address of this node)
    pub my_address: u8,
    /// If set RX packet is accepted if its destination address matches with `multicast_address`
    pub filter_on_multicast_address: bool,
    /// Specifies the Multicast group address for this node
    pub multicast_address: u8,
    /// If set RX packet is accepted if its destination address matches with `broadcast_address`
    pub filter_on_broadcast_address: bool,
    /// Specifies the Broadcast address for this node.
    pub broadcast_address: u8
}

pub struct WMBusProtocolOpts {
    // TODO
}

pub struct STackProtocolOpts {
    // TODO
}

/// Before on-the-air transmission, raw data is properly cast into a packet structure. The 
/// SPIRIT1 offers a highly flexible and fully programmable packet; the structure of the packet, 
/// the number, the type, and the dimension of the fields inside the packet depend on one of the 
/// possible configuration settings. 
pub enum PacketConfiguration {
    Basic(BasicProtocolOpts),
    WMBus(WMBusProtocolOpts),
    STack(STackProtocolOpts),
}

/// Preamble length
#[derive(TryValued)]
pub enum PreambleLength {
    #[valued(1)]
    Bytes01,
    #[valued(2)]
    Bytes02,
    #[valued(3)]
    Bytes03,
    #[valued(4)]
    Bytes04,
    #[valued(5)]
    Bytes05,
    #[valued(6)]
    Bytes06,
    #[valued(7)]
    Bytes07,
    #[valued(8)]
    Bytes08,
    #[valued(9)]
    Bytes09,
    #[valued(10)]
    Bytes10,
    #[valued(11)]
    Bytes11,
    #[valued(12)]
    Bytes12,
    #[valued(13)]
    Bytes13,
    #[valued(14)]
    Bytes14,
    #[valued(15)]
    Bytes15,
    #[valued(16)]
    Bytes16,
    #[valued(17)]
    Bytes17,
    #[valued(18)]
    Bytes18,
    #[valued(19)]
    Bytes19,
    #[valued(20)]
    Bytes20,
    #[valued(21)]
    Bytes21,
    #[valued(22)]
    Bytes22,
    #[valued(23)]
    Bytes23,
    #[valued(24)]
    Bytes24,
    #[valued(25)]
    Bytes25,
    #[valued(26)]
    Bytes26,
    #[valued(27)]
    Bytes27,
    #[valued(28)]
    Bytes28,
    #[valued(29)]
    Bytes29,
    #[valued(30)]
    Bytes30,
    #[valued(31)]
    Bytes31,
    #[valued(32)]
    Bytes32 
}

#[derive(TryValued)]
pub enum PacketSyncLength {
    #[valued(1)]
    Bytes01,
    #[valued(2)]
    Bytes02,
    #[valued(3)]
    Bytes03,
    #[valued(4)]
    Bytes04,
}

#[derive(TryValued)]
pub enum PacketControlLength {
    #[valued(0)]
    Bytes0,
    #[valued(1)]
    Bytes01,
    #[valued(2)]
    Bytes02,
    #[valued(3)]
    Bytes03,
    #[valued(4)]
    Bytes04,
}