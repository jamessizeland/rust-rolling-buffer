use std::fmt::Error;

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Rolling Buffer that will fill to capacity and then start overwriting the oldest data first.
struct RollingBuffer<T>
where
    T: Copy + Default,
{
    capacity: usize,
    buffer: Vec<T>,
    current_index: usize,
    count: usize,
}

impl<T> RollingBuffer<T>
where
    T: Copy + Default,
{
    /// Create a new buffer and allocate memory for it immediately up to a size of capacity.
    fn new(capacity: usize) -> RollingBuffer<T> {
        RollingBuffer {
            capacity,
            buffer: vec![Default::default(); capacity],
            current_index: 0,
            count: 0,
        }
    }
    /// Add a new value to the buffer, if it has reached capacity it will overwrite the oldest datapoint.
    fn add(&mut self, value: T) -> bool {
        self.buffer[self.current_index] = value;
        self.current_index = (self.current_index + 1) % self.capacity;
        self.count = std::cmp::min(self.count + 1, self.capacity);
        self.is_full()
    }
    /// How many valid values are in the buffer?
    fn len(&self) -> usize {
        self.count
    }
    /// Report is the buffer has reached capacity and is now overwriting old data.
    fn is_full(&self) -> bool {
        self.count == self.capacity
    }
    /// Return a single value from the buffer at the position requested.  
    /// If the buffer has hit capacity this will be the position in the valid data -
    /// where index 0 is the newest datapoint, 1 is the second newest, and so on.
    ///
    /// If index > capacity, returns the oldest valid number
    fn get(&self, index: usize) -> &T {
        if index >= self.capacity {
            return &self.buffer[(self.current_index + self.capacity - 1 + self.capacity
                - self.count)
                % self.capacity];
        }
        &self.buffer[(self.current_index + index + self.capacity - self.count) % self.capacity]
    }
    /// Return all of the valid values in the buffer, in the order they were added.
    fn values(&self) -> Vec<T> {
        let mut values = Vec::with_capacity(self.count);
        for i in (0..self.count).rev() {
            values.push(*self.get(i));
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn it_works() {
        let start = Instant::now();
        let mut buff = RollingBuffer::new(20);
        for num in 0..40 {
            // let val = ALPHABET.chars().nth(num).unwrap_or('Z');
            let val = num;
            buff.add(val);
            let _val = buff.values();
            println!("len: {} - {:?}", buff.len(), buff.values());
        }
        for num in 0..40 {
            println!("len: {} - {:?}", buff.len(), buff.get(num));
        }
        println!("runtime: {:?}", start.elapsed());
        assert_eq!(5, 0);
    }
}
