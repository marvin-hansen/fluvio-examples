use crate::prelude::{FirstDataBar, MessageType};
use sbe_bindings::first_data_bar_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{FirstDataBarDecoder, MessageHeaderDecoder, ReadBuf, SbeResult};

impl FirstDataBar {
    pub fn decode_first_data_bar_message(buffer: &[u8]) -> SbeResult<FirstDataBar> {
        let mut csg = FirstDataBarDecoder::default();
        let buf = ReadBuf::new(buffer);

        let header = MessageHeaderDecoder::default().wrap(buf, 0);
        assert_eq!(SBE_TEMPLATE_ID, header.template_id());
        csg = csg.header(header);

        let sbe_message_type = csg.message_type();
        let message_type = MessageType::from(sbe_message_type as u16);
        assert_eq!(message_type, MessageType::LastDataBar);

        let symbol_id = csg.symbol_id();

        let message = FirstDataBar::new(symbol_id);

        Ok(message)
    }
}
