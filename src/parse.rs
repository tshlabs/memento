//


use nom::{be_u32, be_f32, be_f64};

use file::{AggregationType, Metadata, ArchiveInfo, Header};

//trace_macros!(true);

// Basic data types used by the Whisper database format, all big-endian.
named!(parse_u32<&[u8], u32>, flat_map!(take!(4), be_u32));
named!(parse_f32<&[u8], f32>, flat_map!(take!(4), be_f32));
named!(parse_f64<&[u8], f64>, flat_map!(take!(8), be_f64));

// Metadata types
named!(parse_aggregation_type<&[u8], AggregationType>,
       do_parse!(
           val: parse_u32 >>
           agg: expr_opt!(match val {
               1 => Some(AggregationType::Average),
               2 => Some(AggregationType::Sum),
               3 => Some(AggregationType::Last),
               4 => Some(AggregationType::Max),
               5 => Some(AggregationType::Min),
               6 => Some(AggregationType::AvgZero),
               7 => Some(AggregationType::AbsMax),
               8 => Some(AggregationType::AbsMin),
               _ => None,
           }) >>
           (agg)
       )
);

named!(parse_max_retention<&[u8], u32>, call!(parse_u32));
named!(parse_x_files_factor<&[u8], f32>, call!(parse_f32));
named!(parse_archive_count<&[u8], u32>, call!(parse_u32));

named!(parse_metadata<&[u8], Metadata>,
       do_parse!(
           agg: call!(parse_aggregation_type) >>
           ret: call!(parse_max_retention) >>
           xff: call!(parse_x_files_factor) >>
           ac: call!(parse_archive_count) >>
           md: value!(Metadata::new(agg, ret, xff, ac)) >>
           (md)
       )
);

// Archive info types
named!(parse_archive_offset<&[u8], u32>, call!(parse_u32));
named!(parse_archive_secs_per_point<&[u8], u32>, call!(parse_u32));
named!(parse_archive_num_points<&[u8], u32>, call!(parse_u32));

named!(parse_archive_info<&[u8], ArchiveInfo>,
       do_parse!(
           off: call!(parse_archive_offset) >>
           spp: call!(parse_archive_secs_per_point) >>
           np: call!(parse_archive_num_points) >>
           ai: value!(ArchiveInfo::new(off, spp, np)) >>
           (ai)
       )
);

// Parse the entire file header
named!(parse_header<&[u8], Header>,
       do_parse!(
           metadata: call!(parse_metadata) >>
           archives: count!(parse_archive_info, metadata.archive_count() as usize) >>
           header: value!(Header::new(metadata, archives)) >>
           (header)
       )
);

#[cfg(test)]
mod tests {
    use std::mem;
    use nom::IResult;
    use super::{parse_u32, parse_f32, parse_f64};

    // TODO: probably going to need to use byteorder here

    #[test]
    fn test_parse_u32() {
        let expected = 2342u32;
        let as_bytes: [u8; 4] = unsafe { mem::transmute(expected.to_be()) };
        assert_eq!(IResult::Done(&b""[..], expected), parse_u32(&as_bytes));
    }

    #[test]
    fn test_parse_f32() {
    }

    #[test]
    fn test_parse_f64() {
    }
}