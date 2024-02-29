
// Based off of sdk/transcribestreaming/src/event_stream_serde.rs
#[non_exhaustive]
#[derive(Debug)]
pub struct LambdaInvokeStreamUnmarshaller;

impl LambdaInvokeStreamUnmarshaller {
    pub fn new() -> Self {
        LambdaInvokeStreamUnmarshaller
    }
}
impl ::aws_smithy_eventstream::frame::UnmarshallMessage for LambdaInvokeStreamUnmarshaller {
    type Output = crate::types::LambdaInvokeStream;
    type Error = ::aws_smithy_types::error::ErrorMetadata;
    fn unmarshall(
        &self,
        message: &::aws_smithy_eventstream::frame::Message,
    ) -> std::result::Result<::aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>, ::aws_smithy_eventstream::error::Error>
    {
        let response_headers = ::aws_smithy_eventstream::smithy::parse_response_headers(message)?;
        match response_headers.message_type.as_str() {
            "event" => match response_headers.smithy_type.as_str() {
                "PayloadChunk" => {
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::LambdaInvokeStream::PayloadChunk(message.payload().clone()),
                    ))
                },
                "InvokeComplete" => {
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::LambdaInvokeStream::InvokeComplete(message.payload().clone()),
                    ))
                },
                _unknown_variant => Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                    crate::types::LambdaInvokeStream::Unknown,
                )),
            },
            "exception" => {
                let generic = match crate::protocol_serde::parse_event_stream_error_metadata(message.payload()) {
                    Ok(builder) => builder.build(),
                    Err(err) => ::aws_smithy_types::error::ErrorMetadata::default()
                };
                return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(generic));
            },
            value => {
                return Err(::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                    "unrecognized :message-type: {}",
                    value
                )));
            }
        }
    }
}
