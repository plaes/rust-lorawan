use crate::mac;
use lorawan::certification::{parse_downlink_certification_messages, DownlinkDUTCommand};

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
}

pub struct Certification {}

impl Certification {
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
}

impl From<Response> for mac::Response {
    fn from(m: Response) -> Self {
        mac::Response::Certification(m)
    }
}
