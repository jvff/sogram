use std::mem;

pub struct BytesToFloats<I>
where
    I: Iterator<Item = u8>,
{
    bytes: I,
}

impl<I> BytesToFloats<I>
where
    I: Iterator<Item = u8>,
{
    pub fn new(bytes: I) -> Self {
        BytesToFloats { bytes }
    }
}

impl<I> Iterator for BytesToFloats<I>
where
    I: Iterator<Item = u8>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut word = 0;

        for _ in 0..4 {
            word >>= 8;
            word |= (self.bytes.next()? as u32) << 24;
        }

        let value = unsafe { mem::transmute::<u32, f32>(word) };

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let values = vec![1.0, -2.5, 3.0, -0.000016, 160432.5];
        let mut bytes = Vec::new();

        for value in values.iter() {
            let mut word = unsafe { mem::transmute::<f32, u32>(*value) };

            for _ in 0..4 {
                bytes.push((word & 0xFF) as u8);
                word >>= 8;
            }
        }

        let result: Vec<f32> = BytesToFloats::new(bytes.into_iter()).collect();

        assert_eq!(result, values);
    }
}
