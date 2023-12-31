contract;

abi Counter {
    #[storage(read)]
    fn get_count() -> u64;

    #[storage(read)]
    fn try_get_count() -> u64;

    #[storage(write, read)]
    fn increment_count(amount: u64) -> u64;
}

storage {
    counter: u64 = 1,
}

impl Counter for Contract {
    #[storage(read)]
    fn get_count() -> u64 {
        storage.counter.read()
    }

    #[storage(read)]
    fn try_get_count() -> u64 {
        storage.counter.try_read().unwrap_or(0)
    }


    #[storage(write, read)]
    fn increment_count(amount: u64) -> u64 {
        let current = storage.counter.try_read().unwrap_or(0);
        storage.counter.write(current + amount);
        storage.counter.read()
    }
}
