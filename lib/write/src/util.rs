use arrow::ipc::reader::StreamReader;
use arrow::record_batch::RecordBatch;

type ArrowResult<T> = arrow::error::Result<T>;

pub fn read_record_batch_from_vec(bytes: Vec<u8>) -> ArrowResult<RecordBatch> {
    let mut stream_reader = StreamReader::try_new(bytes.as_slice(), None)?;
    let record_batch = stream_reader.next().unwrap()?;

    Ok(record_batch)
}
