fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???

        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }

    #[test]
    fn clone_slice_out_of_array() {
        let mut nice_slice = [0; 3];

        {
            let a = [1, 2, 3, 4, 5];

            nice_slice.clone_from_slice(&a[1..4]);
        }

        assert_eq!([2, 3, 4], nice_slice);
    }
}
