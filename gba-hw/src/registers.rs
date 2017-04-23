use vcell::VolatileCell;

pub struct ReadOnly<T: Copy> {
    value: VolatileCell<T>
}

impl<T: Copy> ReadOnly<T> {
    pub fn read(&self) -> T {
        self.value.get()
    }
}

pub struct WriteOnly<T: Copy> {
    value: VolatileCell<T>
}

impl<T: Copy> WriteOnly<T> {
    pub fn write(&mut self, t: T) {
        self.value.set(t)
    }
}

pub struct ReadWrite<T: Copy> {
    value: VolatileCell<T>
}

impl<T: Copy> ReadWrite<T> {
    pub fn write(&mut self, t: T) {
        self.value.set(t)
    }

    pub fn read(&self) -> T {
        self.value.get()
    }
}

