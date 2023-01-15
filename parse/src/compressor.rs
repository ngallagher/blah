static LINE_LIMIT: u32 = 32000;

#[warn(dead_code)]
struct Source<'a> {
    original: &'a [char], 
    source: &'a [char], 
    lines: &'a [u16],
    types: &'a [u16], 
    count: u32
}


struct SourceCompressor {

}

impl SourceCompressor {

    fn compress() {
        
    }
}