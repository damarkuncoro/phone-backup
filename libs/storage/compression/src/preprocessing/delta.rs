/// Delta encoder/decoder preprocessing for transforming monotonic or sequential data.
pub struct DeltaEncoder;

impl DeltaEncoder {
    /// Applies forward 1-byte delta transformation: out[0] = in[0], out[i] = in[i] - in[i-1] (wrapping).
    pub fn encode(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut output = Vec::with_capacity(data.len());
        output.push(data[0]);

        for i in 1..data.len() {
            output.push(data[i].wrapping_sub(data[i - 1]));
        }

        output
    }

    /// Reverses 1-byte delta transformation: out[0] = in[0], out[i] = out[i-1] + in[i] (wrapping).
    pub fn decode(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut output = Vec::with_capacity(data.len());
        output.push(data[0]);

        for i in 1..data.len() {
            let prev = output[i - 1];
            output.push(prev.wrapping_add(data[i]));
        }

        output
    }
}
