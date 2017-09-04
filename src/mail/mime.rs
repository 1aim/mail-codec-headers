use std::ops::Deref;

use mime::{ Mime, BOUNDARY };

use error::*;
use utils::{ is_multipart_mime, HeaderTryInto };


#[derive(Debug)]
pub struct SinglepartMime( Mime );

impl SinglepartMime {
    pub fn new( mime: Mime ) -> Result<Self> {
        if !is_multipart_mime( &mime ) {
            Ok( SinglepartMime( mime ) )
        } else {
            Err( ErrorKind::NotSinglepartMime( mime ).into() )
        }
    }
}

impl HeaderTryInto<Mime> for SinglepartMime {
    fn try_into(self) -> Result<Mime> {
        Ok( self.0 )
    }
}

impl Into<Mime> for SinglepartMime {
    fn into( self ) -> Mime {
        self.0
    }
}

impl Deref for SinglepartMime {
    type Target = Mime;

    fn deref( &self ) -> &Mime {
        &self.0
    }
}

#[derive(Debug)]
pub struct MultipartMime( Mime );

impl MultipartMime {

    pub fn new( mime: Mime ) -> Result<Self> {
        if is_multipart_mime( &mime ) {
            check_boundary( &mime )?;
            Ok( MultipartMime( mime ) )
        }  else {
            Err( ErrorKind::NotMultipartMime( mime ).into() )
        }

    }
}

impl HeaderTryInto<Mime> for MultipartMime {
    fn try_into(self) -> Result<Mime> {
        Ok( self.0 )
    }
}

impl Into<Mime> for MultipartMime {
    fn into( self ) -> Mime {
        self.0
    }
}

impl Deref for MultipartMime {
    type Target = Mime;

    fn deref( &self ) -> &Mime {
        &self.0
    }
}

fn check_boundary( mime: &Mime ) -> Result<()> {
    mime.get_param( BOUNDARY )
        .map( |_|() )
        .ok_or_else( || ErrorKind::MultipartBoundaryMissing.into() )
}
