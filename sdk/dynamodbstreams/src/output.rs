// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output of a <code>ListStreams</code> operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListStreamsOutput {
    /// <p>A list of stream descriptors associated with the current account and endpoint.</p>
    pub streams: std::option::Option<std::vec::Vec<crate::model::Stream>>,
    /// <p>The stream ARN of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
    /// <p>If <code>LastEvaluatedStreamArn</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p>
    /// <p>If <code>LastEvaluatedStreamArn</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedStreamArn</code> is empty.</p>
    pub last_evaluated_stream_arn: std::option::Option<std::string::String>,
}
impl ListStreamsOutput {
    /// <p>A list of stream descriptors associated with the current account and endpoint.</p>
    pub fn streams(&self) -> std::option::Option<&[crate::model::Stream]> {
        self.streams.as_deref()
    }
    /// <p>The stream ARN of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
    /// <p>If <code>LastEvaluatedStreamArn</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p>
    /// <p>If <code>LastEvaluatedStreamArn</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedStreamArn</code> is empty.</p>
    pub fn last_evaluated_stream_arn(&self) -> std::option::Option<&str> {
        self.last_evaluated_stream_arn.as_deref()
    }
}
impl std::fmt::Debug for ListStreamsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListStreamsOutput");
        formatter.field("streams", &self.streams);
        formatter.field("last_evaluated_stream_arn", &self.last_evaluated_stream_arn);
        formatter.finish()
    }
}
/// See [`ListStreamsOutput`](crate::output::ListStreamsOutput).
pub mod list_streams_output {

    /// A builder for [`ListStreamsOutput`](crate::output::ListStreamsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) streams: std::option::Option<std::vec::Vec<crate::model::Stream>>,
        pub(crate) last_evaluated_stream_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `streams`.
        ///
        /// To override the contents of this collection use [`set_streams`](Self::set_streams).
        ///
        /// <p>A list of stream descriptors associated with the current account and endpoint.</p>
        pub fn streams(mut self, input: crate::model::Stream) -> Self {
            let mut v = self.streams.unwrap_or_default();
            v.push(input);
            self.streams = Some(v);
            self
        }
        /// <p>A list of stream descriptors associated with the current account and endpoint.</p>
        pub fn set_streams(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Stream>>,
        ) -> Self {
            self.streams = input;
            self
        }
        /// <p>The stream ARN of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
        /// <p>If <code>LastEvaluatedStreamArn</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p>
        /// <p>If <code>LastEvaluatedStreamArn</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedStreamArn</code> is empty.</p>
        pub fn last_evaluated_stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.last_evaluated_stream_arn = Some(input.into());
            self
        }
        /// <p>The stream ARN of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
        /// <p>If <code>LastEvaluatedStreamArn</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p>
        /// <p>If <code>LastEvaluatedStreamArn</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedStreamArn</code> is empty.</p>
        pub fn set_last_evaluated_stream_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.last_evaluated_stream_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`ListStreamsOutput`](crate::output::ListStreamsOutput).
        pub fn build(self) -> crate::output::ListStreamsOutput {
            crate::output::ListStreamsOutput {
                streams: self.streams,
                last_evaluated_stream_arn: self.last_evaluated_stream_arn,
            }
        }
    }
}
impl ListStreamsOutput {
    /// Creates a new builder-style object to manufacture [`ListStreamsOutput`](crate::output::ListStreamsOutput).
    pub fn builder() -> crate::output::list_streams_output::Builder {
        crate::output::list_streams_output::Builder::default()
    }
}

/// <p>Represents the output of a <code>GetShardIterator</code> operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetShardIteratorOutput {
    /// <p>The position in the shard from which to start reading stream records sequentially. A shard iterator specifies this position using the sequence number of a stream record in a shard.</p>
    pub shard_iterator: std::option::Option<std::string::String>,
}
impl GetShardIteratorOutput {
    /// <p>The position in the shard from which to start reading stream records sequentially. A shard iterator specifies this position using the sequence number of a stream record in a shard.</p>
    pub fn shard_iterator(&self) -> std::option::Option<&str> {
        self.shard_iterator.as_deref()
    }
}
impl std::fmt::Debug for GetShardIteratorOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetShardIteratorOutput");
        formatter.field("shard_iterator", &self.shard_iterator);
        formatter.finish()
    }
}
/// See [`GetShardIteratorOutput`](crate::output::GetShardIteratorOutput).
pub mod get_shard_iterator_output {

    /// A builder for [`GetShardIteratorOutput`](crate::output::GetShardIteratorOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) shard_iterator: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The position in the shard from which to start reading stream records sequentially. A shard iterator specifies this position using the sequence number of a stream record in a shard.</p>
        pub fn shard_iterator(mut self, input: impl Into<std::string::String>) -> Self {
            self.shard_iterator = Some(input.into());
            self
        }
        /// <p>The position in the shard from which to start reading stream records sequentially. A shard iterator specifies this position using the sequence number of a stream record in a shard.</p>
        pub fn set_shard_iterator(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.shard_iterator = input;
            self
        }
        /// Consumes the builder and constructs a [`GetShardIteratorOutput`](crate::output::GetShardIteratorOutput).
        pub fn build(self) -> crate::output::GetShardIteratorOutput {
            crate::output::GetShardIteratorOutput {
                shard_iterator: self.shard_iterator,
            }
        }
    }
}
impl GetShardIteratorOutput {
    /// Creates a new builder-style object to manufacture [`GetShardIteratorOutput`](crate::output::GetShardIteratorOutput).
    pub fn builder() -> crate::output::get_shard_iterator_output::Builder {
        crate::output::get_shard_iterator_output::Builder::default()
    }
}

/// <p>Represents the output of a <code>GetRecords</code> operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetRecordsOutput {
    /// <p>The stream records from the shard, which were retrieved using the shard iterator.</p>
    pub records: std::option::Option<std::vec::Vec<crate::model::Record>>,
    /// <p>The next position in the shard from which to start sequentially reading stream records. If set to <code>null</code>, the shard has been closed and the requested iterator will not return any more data.</p>
    pub next_shard_iterator: std::option::Option<std::string::String>,
}
impl GetRecordsOutput {
    /// <p>The stream records from the shard, which were retrieved using the shard iterator.</p>
    pub fn records(&self) -> std::option::Option<&[crate::model::Record]> {
        self.records.as_deref()
    }
    /// <p>The next position in the shard from which to start sequentially reading stream records. If set to <code>null</code>, the shard has been closed and the requested iterator will not return any more data.</p>
    pub fn next_shard_iterator(&self) -> std::option::Option<&str> {
        self.next_shard_iterator.as_deref()
    }
}
impl std::fmt::Debug for GetRecordsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetRecordsOutput");
        formatter.field("records", &self.records);
        formatter.field("next_shard_iterator", &self.next_shard_iterator);
        formatter.finish()
    }
}
/// See [`GetRecordsOutput`](crate::output::GetRecordsOutput).
pub mod get_records_output {

    /// A builder for [`GetRecordsOutput`](crate::output::GetRecordsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) records: std::option::Option<std::vec::Vec<crate::model::Record>>,
        pub(crate) next_shard_iterator: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `records`.
        ///
        /// To override the contents of this collection use [`set_records`](Self::set_records).
        ///
        /// <p>The stream records from the shard, which were retrieved using the shard iterator.</p>
        pub fn records(mut self, input: crate::model::Record) -> Self {
            let mut v = self.records.unwrap_or_default();
            v.push(input);
            self.records = Some(v);
            self
        }
        /// <p>The stream records from the shard, which were retrieved using the shard iterator.</p>
        pub fn set_records(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Record>>,
        ) -> Self {
            self.records = input;
            self
        }
        /// <p>The next position in the shard from which to start sequentially reading stream records. If set to <code>null</code>, the shard has been closed and the requested iterator will not return any more data.</p>
        pub fn next_shard_iterator(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_shard_iterator = Some(input.into());
            self
        }
        /// <p>The next position in the shard from which to start sequentially reading stream records. If set to <code>null</code>, the shard has been closed and the requested iterator will not return any more data.</p>
        pub fn set_next_shard_iterator(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.next_shard_iterator = input;
            self
        }
        /// Consumes the builder and constructs a [`GetRecordsOutput`](crate::output::GetRecordsOutput).
        pub fn build(self) -> crate::output::GetRecordsOutput {
            crate::output::GetRecordsOutput {
                records: self.records,
                next_shard_iterator: self.next_shard_iterator,
            }
        }
    }
}
impl GetRecordsOutput {
    /// Creates a new builder-style object to manufacture [`GetRecordsOutput`](crate::output::GetRecordsOutput).
    pub fn builder() -> crate::output::get_records_output::Builder {
        crate::output::get_records_output::Builder::default()
    }
}

/// <p>Represents the output of a <code>DescribeStream</code> operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeStreamOutput {
    /// <p>A complete description of the stream, including its creation date and time, the DynamoDB table associated with the stream, the shard IDs within the stream, and the beginning and ending sequence numbers of stream records within the shards.</p>
    pub stream_description: std::option::Option<crate::model::StreamDescription>,
}
impl DescribeStreamOutput {
    /// <p>A complete description of the stream, including its creation date and time, the DynamoDB table associated with the stream, the shard IDs within the stream, and the beginning and ending sequence numbers of stream records within the shards.</p>
    pub fn stream_description(&self) -> std::option::Option<&crate::model::StreamDescription> {
        self.stream_description.as_ref()
    }
}
impl std::fmt::Debug for DescribeStreamOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeStreamOutput");
        formatter.field("stream_description", &self.stream_description);
        formatter.finish()
    }
}
/// See [`DescribeStreamOutput`](crate::output::DescribeStreamOutput).
pub mod describe_stream_output {

    /// A builder for [`DescribeStreamOutput`](crate::output::DescribeStreamOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) stream_description: std::option::Option<crate::model::StreamDescription>,
    }
    impl Builder {
        /// <p>A complete description of the stream, including its creation date and time, the DynamoDB table associated with the stream, the shard IDs within the stream, and the beginning and ending sequence numbers of stream records within the shards.</p>
        pub fn stream_description(mut self, input: crate::model::StreamDescription) -> Self {
            self.stream_description = Some(input);
            self
        }
        /// <p>A complete description of the stream, including its creation date and time, the DynamoDB table associated with the stream, the shard IDs within the stream, and the beginning and ending sequence numbers of stream records within the shards.</p>
        pub fn set_stream_description(
            mut self,
            input: std::option::Option<crate::model::StreamDescription>,
        ) -> Self {
            self.stream_description = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeStreamOutput`](crate::output::DescribeStreamOutput).
        pub fn build(self) -> crate::output::DescribeStreamOutput {
            crate::output::DescribeStreamOutput {
                stream_description: self.stream_description,
            }
        }
    }
}
impl DescribeStreamOutput {
    /// Creates a new builder-style object to manufacture [`DescribeStreamOutput`](crate::output::DescribeStreamOutput).
    pub fn builder() -> crate::output::describe_stream_output::Builder {
        crate::output::describe_stream_output::Builder::default()
    }
}
