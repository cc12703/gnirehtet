use log::info;





pub struct IcmpHeader<'a> {
    raw: &'a [u8],
    data: &'a IcmpHeaderData,
}


pub struct IcmpHeaderMut<'a> {
    raw: &'a mut [u8],
    data: &'a mut IcmpHeaderData,
}



#[derive(Clone)]
#[derive(Debug)]
pub struct IcmpHeaderData {
    msg_type: u8,
    code: u8,
}


#[allow(dead_code)]
impl IcmpHeaderData {
    pub fn parse(raw: &[u8]) -> Self {
        Self {
            msg_type: raw[0],
            code: raw[1],
        }
    }

    #[inline]
    pub fn bind<'c, 'a: 'c, 'b: 'c>(&'a self, raw: &'b [u8]) -> IcmpHeader<'c> {
        IcmpHeader::new(raw, self)
    }

    #[inline]
    pub fn bind_mut<'c, 'a: 'c, 'b: 'c>(&'a mut self, raw: &'b mut [u8]) -> IcmpHeaderMut<'c> {
        IcmpHeaderMut::new(raw, self)
    }

    #[inline]
    pub fn msg_type(&self) -> u8 {
        self.msg_type
    }

    #[inline]
    pub fn code(&self) -> u8 {
        self.code
    }
}



macro_rules! icmp_header_common {
    ($name:ident, $raw_type:ty, $data_type:ty) => {
        // for readability, declare structs manually outside the macro
        #[allow(dead_code)]
        impl<'a> $name<'a> {
            pub fn new(raw: $raw_type, data: $data_type) -> Self {
                Self {
                    raw: raw,
                    data: data,
                }
            }

            #[inline]
            pub fn raw(&self) -> &[u8] {
                self.raw
            }

            #[inline]
            pub fn data(&self) -> &IcmpHeaderData {
                self.data
            }

            
        }
    };
}

icmp_header_common!(IcmpHeader, &'a [u8], &'a IcmpHeaderData);
icmp_header_common!(IcmpHeaderMut, &'a mut [u8], &'a mut IcmpHeaderData);




#[allow(dead_code)]
impl<'a> IcmpHeaderMut<'a> {
    #[inline]
    pub fn raw_mut(&mut self) -> &mut [u8] {
        self.raw
    }

    #[inline]
    pub fn data_mut(&mut self) -> &mut IcmpHeaderData {
        self.data
    }

}