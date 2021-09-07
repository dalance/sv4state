use num_traits::{FromPrimitive, PrimInt, WrappingShr};
use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub struct Sv4State<T: Copy + Debug> {
    pub v: T,
    pub z: T,
    pub x: T,
}

impl<T: Copy + Debug + PrimInt + WrappingShr> std::fmt::Binary for Sv4State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let payload_width = T::zero().count_zeros();
        let mut buf = if f.alternate() {
            String::from("0b")
        } else {
            String::from("")
        };

        for i in 0..payload_width {
            let v = (self.v.wrapping_shr(payload_width - i - 1)) & T::one();
            let z = (self.z.wrapping_shr(payload_width - i - 1)) & T::one();
            let x = (self.x.wrapping_shr(payload_width - i - 1)) & T::one();

            if z == T::one() {
                buf.push_str("z")
            } else if x == T::one() {
                buf.push_str("x")
            } else if v == T::one() {
                buf.push_str("1")
            } else {
                buf.push_str("0")
            }
        }
        write!(f, "{}", buf)
    }
}

impl<T: Copy + Debug + PrimInt + WrappingShr + FromPrimitive + std::fmt::LowerHex>
    std::fmt::LowerHex for Sv4State<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let payload_width = T::zero().count_zeros();
        let mut buf = if f.alternate() {
            String::from("0x")
        } else {
            String::from("")
        };

        let all_hi = T::from_u32(15).unwrap();

        for i in 0..payload_width / 4 {
            let v = (self.v.wrapping_shr(payload_width - (i + 1) * 4)) & all_hi;
            let z = (self.z.wrapping_shr(payload_width - (i + 1) * 4)) & all_hi;
            let x = (self.x.wrapping_shr(payload_width - (i + 1) * 4)) & all_hi;

            if z == all_hi {
                buf.push_str("z")
            } else if z != T::zero() {
                buf.push_str("Z")
            } else if x == all_hi {
                buf.push_str("x")
            } else if x != T::zero() {
                buf.push_str("X")
            } else {
                buf.push_str(&format!("{:x}", v))
            }
        }
        write!(f, "{}", buf)
    }
}

impl<T: Copy + Debug + PrimInt + FromPrimitive> Sv4State<T> {
    pub fn from_dpi(data: &[u64]) -> Vec<Self> {
        let payload_width = T::zero().count_zeros() as usize;
        let bit_width = 32 * data.len();
        let len = if bit_width % payload_width == 0 {
            bit_width / payload_width
        } else {
            bit_width / payload_width + 1
        };

        let mut ret = Vec::new();
        for i in 0..len {
            let mut v = T::zero();
            let mut z = T::zero();
            let mut x = T::zero();

            for j in 0..(payload_width / 8) {
                // byte index
                let index = i * payload_width / 8 + j;

                if index / 4 >= data.len() {
                    break;
                }

                let aval = (data[index / 4] >> (index % 4) * 8 + 0) & 0xff;
                let bval = (data[index / 4] >> (index % 4) * 8 + 32) & 0xff;
                let aval = T::from_u64(aval).unwrap();
                let bval = T::from_u64(bval).unwrap();
                let aval = aval << (j * 8);
                let bval = bval << (j * 8);

                v = v | (aval & !bval);
                z = z | (bval & !aval);
                x = x | (bval & aval);
            }

            ret.push(Sv4State { v, z, x });
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_dpi_u8() {
        let sv_u8 = Sv4State::<u8>::from_dpi(&[0x00000000_01234567, 0xffffffff_89abcdef]);

        assert_eq!(sv_u8[0].v, 0x67);
        assert_eq!(sv_u8[0].z, 0x00);
        assert_eq!(sv_u8[0].x, 0x00);
        assert_eq!(sv_u8[1].v, 0x45);
        assert_eq!(sv_u8[1].z, 0x00);
        assert_eq!(sv_u8[1].x, 0x00);
        assert_eq!(sv_u8[2].v, 0x23);
        assert_eq!(sv_u8[2].z, 0x00);
        assert_eq!(sv_u8[2].x, 0x00);
        assert_eq!(sv_u8[3].v, 0x01);
        assert_eq!(sv_u8[3].z, 0x00);
        assert_eq!(sv_u8[3].x, 0x00);
        assert_eq!(sv_u8[4].v, 0x00);
        assert_eq!(sv_u8[4].z, 0x10);
        assert_eq!(sv_u8[4].x, 0xef);
        assert_eq!(sv_u8[5].v, 0x00);
        assert_eq!(sv_u8[5].z, 0x32);
        assert_eq!(sv_u8[5].x, 0xcd);
        assert_eq!(sv_u8[6].v, 0x00);
        assert_eq!(sv_u8[6].z, 0x54);
        assert_eq!(sv_u8[6].x, 0xab);
        assert_eq!(sv_u8[7].v, 0x00);
        assert_eq!(sv_u8[7].z, 0x76);
        assert_eq!(sv_u8[7].x, 0x89);
    }

    #[test]
    fn from_dpi_u16() {
        let sv_u16 = Sv4State::<u16>::from_dpi(&[0x00000000_01234567, 0xffffffff_89abcdef]);

        assert_eq!(sv_u16[0].v, 0x4567);
        assert_eq!(sv_u16[0].z, 0x0000);
        assert_eq!(sv_u16[0].x, 0x0000);
        assert_eq!(sv_u16[1].v, 0x0123);
        assert_eq!(sv_u16[1].z, 0x0000);
        assert_eq!(sv_u16[1].x, 0x0000);
        assert_eq!(sv_u16[2].v, 0x0000);
        assert_eq!(sv_u16[2].z, 0x3210);
        assert_eq!(sv_u16[2].x, 0xcdef);
        assert_eq!(sv_u16[3].v, 0x0000);
        assert_eq!(sv_u16[3].z, 0x7654);
        assert_eq!(sv_u16[3].x, 0x89ab);
    }

    #[test]
    fn from_dpi_u32() {
        let sv_u32 = Sv4State::<u32>::from_dpi(&[0x00000000_01234567, 0xffffffff_89abcdef]);

        assert_eq!(sv_u32[0].v, 0x01234567);
        assert_eq!(sv_u32[0].z, 0x00000000);
        assert_eq!(sv_u32[0].x, 0x00000000);
        assert_eq!(sv_u32[1].v, 0x00000000);
        assert_eq!(sv_u32[1].z, 0x76543210);
        assert_eq!(sv_u32[1].x, 0x89abcdef);
    }

    #[test]
    fn from_dpi_u64() {
        let sv_u64 = Sv4State::<u64>::from_dpi(&[0x00000000_01234567, 0xffffffff_89abcdef]);

        assert_eq!(sv_u64[0].v, 0x0000000001234567);
        assert_eq!(sv_u64[0].z, 0x7654321000000000);
        assert_eq!(sv_u64[0].x, 0x89abcdef00000000);
    }

    #[test]
    fn from_dpi_u128() {
        let sv_u128 = Sv4State::<u128>::from_dpi(&[0x00000000_01234567, 0xffffffff_89abcdef]);

        assert_eq!(sv_u128[0].v, 0x0000000001234567);
        assert_eq!(sv_u128[0].z, 0x7654321000000000);
        assert_eq!(sv_u128[0].x, 0x89abcdef00000000);
    }

    #[test]
    fn format_binary() {
        let sv_u16 = Sv4State::<u16>::from_dpi(&[0x00000000_01234567, 0xffffffff_89abcdef]);

        assert_eq!(format!("{:b}", sv_u16[0]), "0100010101100111");
        assert_eq!(format!("{:#b}", sv_u16[0]), "0b0100010101100111");
        assert_eq!(format!("{:b}", sv_u16[1]), "0000000100100011");
        assert_eq!(format!("{:#b}", sv_u16[1]), "0b0000000100100011");
        assert_eq!(format!("{:b}", sv_u16[2]), "xxzzxxzxxxxzxxxx");
        assert_eq!(format!("{:#b}", sv_u16[2]), "0bxxzzxxzxxxxzxxxx");
        assert_eq!(format!("{:b}", sv_u16[3]), "xzzzxzzxxzxzxzxx");
        assert_eq!(format!("{:#b}", sv_u16[3]), "0bxzzzxzzxxzxzxzxx");
    }

    #[test]
    fn format_lower_hex() {
        let sv_u32 = Sv4State::<u32>::from_dpi(&[0x00000000_01234567, 0xffffffff_89abcdef]);

        assert_eq!(format!("{:x}", sv_u32[0]), "01234567");
        assert_eq!(format!("{:#x}", sv_u32[0]), "0x01234567");
        assert_eq!(format!("{:x}", sv_u32[1]), "ZZZZZZZx");
        assert_eq!(format!("{:#x}", sv_u32[1]), "0xZZZZZZZx");
    }
}
