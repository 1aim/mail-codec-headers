use ascii::AsciiStr;

use error::*;
use codec::{ MailEncoder, MailEncodable };

pub use utils::DateTime;

impl<E> MailEncodable<E> for DateTime where E: MailEncoder {

    fn encode(&self, encoder: &mut E) -> Result<()> {
        let as_str = self.to_rfc2822();
        let ascii = unsafe { AsciiStr::from_ascii_unchecked( &*as_str ) };
        encoder.write_str( ascii );
        Ok( () )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use codec::test_utils::*;

    ec_test!{ simple, {
        Some( DateTime::test_time( 45 ) )
    } => ascii => [
        LinePart( "Tue,  6 Aug 2013 04:11:45 +0000" )
    ]}
}