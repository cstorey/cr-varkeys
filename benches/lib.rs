#![feature(test)]
extern crate test;
extern crate cr_varkeys;

use std::io::Cursor;
use std::default::Default;
use cr_varkeys::{Error, encode, decode};

#[bench] fn encoding_0       ( b: &mut test::Bencher )  { bench_encoding ( 1<<0, b     )  }
#[bench] fn encoding_2p8    ( b: &mut test::Bencher )  { bench_encoding ( 1<<8, b  )  }
#[bench] fn encoding_2p16   ( b: &mut test::Bencher )  { bench_encoding ( 1<<16, b )  }
#[bench] fn encoding_2p24 ( b: &mut test::Bencher )  { bench_encoding ( 1<<24, b )  }
#[bench] fn encoding_2p32   ( b: &mut test::Bencher )  { bench_encoding ( 1<<32, b )  }
#[bench] fn encoding_2p40 ( b: &mut test::Bencher )  { bench_encoding ( 1<<40, b )  }
#[bench] fn encoding_2p48 ( b: &mut test::Bencher )  { bench_encoding ( 1<<48, b )  }
#[bench] fn encoding_2p56 ( b: &mut test::Bencher )  { bench_encoding ( 1<<56, b )  }
#[bench] fn encoding_2p63 ( b: &mut test::Bencher )  { bench_encoding ( 1<<63, b )  }

#[bench] fn decoding_0       ( b: &mut test::Bencher )  { bench_decoding ( 1<<0, b     )  }
#[bench] fn decoding_2p8    ( b: &mut test::Bencher )  { bench_decoding ( 1<<8, b  )  }
#[bench] fn decoding_2p16   ( b: &mut test::Bencher )  { bench_decoding ( 1<<16, b )  }
#[bench] fn decoding_2p24 ( b: &mut test::Bencher )  { bench_decoding ( 1<<24, b )  }
#[bench] fn decoding_2p32   ( b: &mut test::Bencher )  { bench_decoding ( 1<<32, b )  }
#[bench] fn decoding_2p40 ( b: &mut test::Bencher )  { bench_decoding ( 1<<40, b )  }
#[bench] fn decoding_2p48 ( b: &mut test::Bencher )  { bench_decoding ( 1<<48, b )  }
#[bench] fn decoding_2p56 ( b: &mut test::Bencher )  { bench_decoding ( 1<<56, b )  }
#[bench] fn decoding_2p63 ( b: &mut test::Bencher )  { bench_decoding ( 1<<63, b )  }


fn bench_encoding(n: usize, b: &mut test::Bencher) {
    let mut bytes = Vec::new();
    encode(n, &mut bytes).expect("encode");
    assert_eq!(n, decode(Cursor::new(&bytes)).expect("decode"));
    b.bytes = bytes.len() as u64;
    // { use std::io::Write; writeln!(::std::io::stderr(), "{:?}:{:?}", n, b.bytes); }

    b.iter(|| {
        bytes.clear();
        encode(n, &mut bytes).expect("encode");
        test::black_box(&bytes);
    })
}

fn bench_decoding(n: usize, b: &mut test::Bencher) {
    let mut bytes = Vec::new();
    encode(n, &mut bytes).expect("encode");
    assert_eq!(n, decode(Cursor::new(&bytes)).expect("decode"));
    b.bytes = bytes.len() as u64;

    b.iter(|| {
        let val = decode(Cursor::new(&bytes)).expect("decode");
        test::black_box(val);
    })
}

