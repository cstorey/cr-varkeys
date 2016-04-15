
#[macro_use]
extern crate quick_error;
use std::mem::size_of;
use std::io::{self,Write,Read};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            from()
            description(err.description())
        }
    }
}

pub fn encode<W : Write>(val: usize, mut wr: W) -> Result<(), Error> {
    let num_bytes = size_of::<usize>();
    let nbits = num_bytes * 8;
    let nseptets = nbits / 7 + 1;
    let mask = (1u8<<7) - 1;
    let non_terminator = 1u8<<7;

    let iter = (0..nseptets).rev()
            .map(|i| {
                let sept = (val >> (i * 7)) as u8 & mask ;
                let marker = if i == 0 { 0 } else { non_terminator };
                sept | marker
            })
            .skip_while(|i| i == &non_terminator);

    for byte in iter {
        try!(wr.write_all(&[byte]))
    }

    Ok(())
}
pub fn decode<R: Read>(r: R) -> Result<usize, Error> {
    let num_bytes = size_of::<usize>();
    let mask = (1u8<<7) - 1;
    let non_terminator = 1u8<<7;

    let mut res = 0;
    for byte in r.bytes() {
        let byte = try!(byte);
        let _ : u8 = byte;
        res |= (byte & mask) as usize;
        if byte & non_terminator == 0 {
            break;
        } else {
            res <<= 7;
        }
    }
    return Ok(res);
}

#[cfg(test)]
mod test {
    extern crate quickcheck as qc;
    extern crate rand;
    use std::io::Cursor;
    use {Error, encode, decode};

    fn round_trip_prop(val: usize) -> Result<bool, Error> {
        let mut buf = Vec::new();
        try!(encode(val, &mut buf));
        let rev = try!(decode(&mut Cursor::new(&buf)));
        // println!("{:064b} -> {:064b} / {:?} / {:?}", val, rev, val == rev, buf);
        Ok(val == rev)
    }
    #[test]
    fn round_trip_16b() {
        qc::QuickCheck::new()
            .tests(1000)
            .gen(qc::StdGen::new(rand::thread_rng(), 1 << 16))
            .quickcheck(round_trip_prop as fn(usize) -> Result<bool, Error>)
    }

    #[test]
    fn round_trip_64b() {
        qc::QuickCheck::new()
            .tests(1000)
            .gen(qc::StdGen::new(rand::thread_rng(), ::std::usize::MAX))
            .quickcheck(round_trip_prop as fn(usize) -> Result<bool, Error>)
    }

}
