use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;

/// RECUERDO DE INTERNET: SPSC Ring Buffer para mi voz de Dr. Xeno.
/// Kukuku... Joseph, esto es diez mil millones por ciento más eficiente.
/// He aplicado 'Cache Padding' (simulado con espacio extra) para evitar el False Sharing.

pub struct DaithonRingBuffer<T, const N: usize> {
    // Espacio de caché para evitar False Sharing entre el productor y el consumidor
    _pad0: [u8; 64],
    buffer: [UnsafeCell<T>; N],
    _pad1: [u8; 64],
    head: AtomicUsize, // Escrito por el consumidor, leído por el productor
    _pad2: [u8; 64],
    tail: AtomicUsize, // Escrito por el productor, leído por el consumidor
    _pad3: [u8; 64],
}

// Nota: En una implementación real usaríamos crossbeam_utils::CachePadded
// Pero como estamos en un entorno puro, la ciencia requiere soluciones creativas.

impl<T: Default + Copy, const N: usize> DaithonRingBuffer<T, N> {
    pub fn new() -> Self {
        let mut buffer = unsafe { std::mem::MaybeUninit::<[UnsafeCell<T>; N]>::uninit().assume_init() };
        for i in 0..N {
            buffer[i] = UnsafeCell::new(T::default());
        }
        
        Self {
            _pad0: [0; 64],
            buffer,
            _pad1: [0; 64],
            head: AtomicUsize::new(0),
            _pad2: [0; 64],
            tail: AtomicUsize::new(0),
            _pad3: [0; 64],
        }
    }

    /// Kukuku... Joseph, observa la elegancia del acceso lock-free.
    pub fn push(&self, value: T) -> Result<(), &str> {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) % N;
        
        if next_tail == self.head.load(Ordering::Acquire) {
            return Err("Buffer lleno. Ineficiente.");
        }
        
        unsafe {
            *self.buffer[tail].get() = value;
        }
        
        self.tail.store(next_tail, Ordering::Release);
        Ok(())
    }

    pub fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        
        if head == self.tail.load(Ordering::Acquire) {
            return None; // No hay datos para procesar.
        }
        
        let value = unsafe { *self.buffer[head].get() };
        self.head.store((head + 1) % N, Ordering::Release);
        
        Some(value)
    }
}
