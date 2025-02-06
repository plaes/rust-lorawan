use crate::mac;
use lorawan::certification::{parse_downlink_certification_messages, DownlinkDUTCommand};

#[derive(Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Response {
    NoUpdate,
    DutReset,
    TxPeriodicityChange(Option<u16>),
}

pub struct Certification {}

impl Certification {
    pub(crate) fn handle_message(&mut self, data: &[u8]) -> Response {
        use DownlinkDUTCommand::*;
        let messages = parse_downlink_certification_messages(data);
        for message in messages {
            match message {
                DutResetReq(..) => return Response::DutReset,
                TxPeriodicityChangeReq(payload) => {
                    if let Ok(periodicity) = payload.periodicity() {
                        return Response::TxPeriodicityChange(periodicity);
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
