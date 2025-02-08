use crate::mac;
use crate::radio::RadioBuffer;
use lorawan::certification::{
    parse_downlink_certification_messages, DownlinkDUTCommand, EchoPayloadAnsCreator,
};
use lorawan::keys::CryptoFactory;

pub(crate) const CERTIFICATION_PORT: u8 = 224;

#[derive(Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Response {
    NoUpdate,
    DutJoin,
    DutReset,
    DutVersions,
    AdrBitChange(bool),
    TxPeriodicityChange(Option<u16>),
    TxFramesCtrlReq(Option<bool>),
    UplinkReady,
}

pub struct Certification {
    pending_uplink: Option<heapless::Vec<u8, 256>>,
}

impl Certification {
    pub fn new() -> Self {
        Self { pending_uplink: None }
    }
    pub(crate) fn handle_message(&mut self, data: &[u8]) -> Response {
        use DownlinkDUTCommand::*;
        let messages = parse_downlink_certification_messages(data);
        for message in messages {
            match message {
                AdrBitChangeReq(payload) => {
                    if let Ok(adr) = payload.adr_enable() {
                        return Response::AdrBitChange(adr);
                    }
                }
                DutJoinReq(..) => return Response::DutJoin,
                DutResetReq(..) => return Response::DutReset,
                DutVersionsReq(..) => return Response::DutVersions,
                EchoPayloadReq(payload) => {
                    let mut buf: heapless::Vec<u8, 256> = heapless::Vec::new();
                    let mut ans = EchoPayloadAnsCreator::new();
                    ans.payload(payload.payload());
                    buf.extend_from_slice(ans.build()).unwrap();
                    self.pending_uplink = Some(buf);
                    return Response::UplinkReady;
                }
                TxPeriodicityChangeReq(payload) => {
                    if let Ok(periodicity) = payload.periodicity() {
                        return Response::TxPeriodicityChange(periodicity);
                    }
                }
                TxFramesCtrlReq(payload) => {
                    if let Ok(frametype) = payload.frame_type_override() {
                        return Response::TxFramesCtrlReq(frametype);
                    }
                }
            }
        }
        /* TODO: Report.. ? */
        Response::NoUpdate
    }

    pub(crate) fn setup_send<C: CryptoFactory + Default, const N: usize>(
        &mut self,
        mut state: &mut mac::State,
        buf: &mut RadioBuffer<N>,
    ) -> mac::Result<mac::FcntUp> {
        let send_data = mac::SendData {
            fport: CERTIFICATION_PORT,
            data: self.pending_uplink.as_ref().unwrap(),
            confirmed: false,
        };
        match &mut state {
            mac::State::Joined(ref mut session) => {
                Ok(session.prepare_buffer::<C, N>(&send_data, buf))
            }
            mac::State::Otaa(_) => Err(mac::Error::NotJoined),
            mac::State::Unjoined => Err(mac::Error::NotJoined),
        }
    }
}

impl From<Response> for mac::Response {
    fn from(m: Response) -> Self {
        mac::Response::Certification(m)
    }
}
