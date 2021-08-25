// If you need more than 10 variables to pass through to the buffer then you should probably create
// a new Buffer and/or normalize your data. You can also extend the BytesArray if absolutely necessary.

//todo Working Progress

type Buffer3<T, U, V> = (T, U, V);

type Buffer4<T, U, V, W> = (T, U, V, W);

type Buffer5<T, U, V, W, X> = (T, U, V, W, X);

#[derive(Copy, Clone)]
pub struct BytesArray<Q: bytemuck::Pod + bytemuck::Zeroable>(pub Q);

impl<Q: bytemuck::Pod + bytemuck::Zeroable> BytesArray<Q> {
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let buf = vec![self.0];
        bytemuck::cast_slice(&buf).to_owned()
    }
}

pub struct Bytes2Array<Q: bytemuck::Pod + bytemuck::Zeroable, R: bytemuck::Pod + bytemuck::Zeroable>(
    Q,
    R,
);

impl<Q: bytemuck::Pod + bytemuck::Zeroable, R: bytemuck::Pod + bytemuck::Zeroable>
    Bytes2Array<Q, R>
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let first = vec![self.0];
        let second = vec![self.1];

        let mut prepend: Vec<u8> = bytemuck::cast_slice(&first).to_vec();

        {
            let mut append: Vec<u8> = bytemuck::cast_slice(&second).to_vec();
            prepend.append(&mut append);
        }

        prepend
    }

    pub fn retrieve_type<T: bytemuck::Pod>(&self, bytes: Vec<u8>) -> T {
        let data: &T = bytemuck::from_bytes(&bytes);
        data.to_owned()
    }
}

pub struct Bytes3Array<
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
>(Buffer3<Q, R, S>);

pub struct Bytes4Array<
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
>(Buffer4<Q, R, S, T>);

pub struct Bytes5Array<Q, R, S, T, U>(Buffer5<Q, R, S, T, U>)
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable;

pub struct Bytes6Array<Q, R, S, T, U, V>(Buffer3<Q, R, S>, Buffer3<T, U, V>)
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable,
    V: bytemuck::Pod + bytemuck::Zeroable;

pub struct Bytes7Array<Q, R, S, T, U, V, W>(Buffer4<Q, R, S, T>, Buffer3<U, V, W>)
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable,
    V: bytemuck::Pod + bytemuck::Zeroable,
    W: bytemuck::Pod + bytemuck::Zeroable;

pub struct Bytes8Array<Q, R, S, T, U, V, W, X>(Buffer4<Q, R, S, T>, Buffer4<U, V, W, X>)
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable,
    V: bytemuck::Pod + bytemuck::Zeroable,
    W: bytemuck::Pod + bytemuck::Zeroable,
    X: bytemuck::Pod + bytemuck::Zeroable;

pub struct Bytes9Array<Q, R, S, T, U, V, W, X, Y>(Buffer5<Q, R, S, T, U>, Buffer4<V, W, X, Y>)
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable,
    V: bytemuck::Pod + bytemuck::Zeroable,
    W: bytemuck::Pod + bytemuck::Zeroable,
    X: bytemuck::Pod + bytemuck::Zeroable,
    Y: bytemuck::Pod + bytemuck::Zeroable;

pub struct Bytes10Array<Q, R, S, T, U, V, W, X, Y, Z>(
    Buffer5<Q, R, S, T, U>,
    Buffer5<V, W, X, Y, Z>,
)
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable,
    V: bytemuck::Pod + bytemuck::Zeroable,
    W: bytemuck::Pod + bytemuck::Zeroable,
    X: bytemuck::Pod + bytemuck::Zeroable,
    Y: bytemuck::Pod + bytemuck::Zeroable,
    Z: bytemuck::Pod + bytemuck::Zeroable;

#[cfg(test)]
mod test {
    use crate::util::container::bytes::{Bytes2Array, BytesArray};

    #[derive(Default, Debug, bytemuck::Zeroable, bytemuck::Pod, Copy, Clone)]
    #[repr(C)]
    struct MockStruct {
        pub arg0: [f32; 3],
        pub arg1: f32,
        pub arg2: [f32; 2],
    }

    #[derive(Default, Debug, bytemuck::Zeroable, bytemuck::Pod, Copy, Clone)]
    #[repr(C)]
    struct MockStruct1 {
        pub arg0: [f32; 3],
        pub arg1: f32,
        pub arg2: [f32; 2],
        pub arg3: [i32; 2],
    }

    #[test]
    fn byte_test() {
        let mock_missing_arg3 = MockStruct {
            arg0: [3.0, 1.0, 10.0],
            arg1: 20.0,
            arg2: [1.5, 2.5],
        };

        let mock = MockStruct1 {
            arg0: [3.0, 1.0, 10.0],
            arg1: 20.0,
            arg2: [1.5, 2.5],
            arg3: [32, 100],
        };

        let byte_array: Bytes2Array<MockStruct, [i32; 2]> = Bytes2Array {
            0: mock_missing_arg3,
            1: [32, 100], //arg 3
        };

        let byte_buffer01: Vec<u8> = bytemuck::cast_slice(&[mock]).to_vec();
        let byte_buffer02: Vec<u8> = byte_array.retrieve_bytes();

        assert_eq!(byte_buffer01.len(), byte_buffer02.len());

        assert!(byte_buffer01.eq(&byte_buffer02));

        let struct_data: MockStruct1 = byte_array.retrieve_type::<MockStruct1>(byte_buffer02);

        //We can convert on struct to another. We can populate the data from the struct with the data passed in the Byte_Array.
        println!(
            "Converted MockStruct {:?}\n to  MockStruct1 {:?}",
            mock_missing_arg3, struct_data
        );
    }
}
