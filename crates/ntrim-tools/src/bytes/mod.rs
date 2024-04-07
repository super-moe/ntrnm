use bytes::{BufMut, BytesMut};

pub trait BytePacketBuilder: BufMut {
    #[inline]
    fn put_packet(&mut self, builder: &mut dyn FnMut(&mut BytesMut))
        where Self: Sized
    {
        let mut buf = BytesMut::new();
        builder(&mut buf);
        self.put(buf);
    }

    #[inline]
    fn put_packet_with_i32_len(&mut self, builder: &mut dyn FnMut(&mut BytesMut) -> i32)
        where Self: Sized
    {
        let mut buf = BytesMut::new();
        let len = builder(&mut buf);
        self.put_i32(len);
        self.put(buf);
    }

    #[inline]
    fn put_packet_with_i16_len(&mut self, builder: &mut dyn FnMut(&mut BytesMut) -> i16)
        where Self: Sized
    {
        let mut buf = BytesMut::new();
        let len = builder(&mut buf);
        self.put_i16(len);
        self.put(buf);
    }

    #[inline]
    fn put_bytes_with_i32_len(&mut self, bytes: &[u8], len: usize)
        where Self: Sized
    {
        self.put_i32(len as i32);
        self.put_slice(bytes);
    }

    #[inline]
    fn put_bytes_with_i16_len(&mut self, bytes: &[u8], len: usize)
        where Self: Sized
    {
        self.put_i16(len as i16);
        self.put_slice(bytes);
    }
}

impl BytePacketBuilder for BytesMut {}
impl<T: BufMut + ?Sized> BytePacketBuilder for &mut T {}
impl<T: BufMut + ?Sized> BytePacketBuilder for Box<T> {}
impl BytePacketBuilder for Vec<u8> {}