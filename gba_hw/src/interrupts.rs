/// `SourceSet` represents a collection of interrupt sources
bitflags! {
    pub flags SourceSet: u16 {
        #[doc = "Vertical blank, enabled in `dispstat`"]
        const VBLANK    = 1 << 0,

        #[doc = "Horizontal blank, enabled in `dispstat`"]
        const HBLANK    = 1 << 1,

        #[doc = "Vertical counter, enabled in `dispstat`"]
        const VCOUNTER  = 1 << 2,

        #[doc = "Timer 0 overflow"]
        const TIMER_0   = 1 << 3,
        #[doc = "Timer 1 overflow"]
        const TIMER_1   = 1 << 4,
        #[doc = "Timer 2 overflow"]
        const TIMER_2   = 1 << 5,
        #[doc = "Timer 3 overflow"]
        const TIMER_3   = 1 << 6,

        #[doc = "Serial interface"]
        const SIO       = 1 << 7,

        #[doc = "DMA 0 complete"]
        const DMA_0     = 1 << 8,
        #[doc = "DMA 1 complete"]
        const DMA_1     = 1 << 9,
        #[doc = "DMA 2 complete"]
        const DMA_2     = 1 << 10,
        #[doc = "DMA 3 complete"]
        const DMA_3     = 1 << 11,

        #[doc = "Keypad pressed, enabled in `keypad_control`"]
        const KEYPAD    = 1 << 12,

        #[doc = "Cartridge interrupt for 3rd party hardware"]
        const EXTERNAL  = 1 << 13,
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MasterEnable {
    InterruptsDisabled,
    InterruptsEnabled,
}

use ::{ReadOnly, WriteOnly, ReadWrite};


register!(
    /// `master_enable` is the master interrupt enable/disable flag.
    pub master_enable: ReadWrite<MasterEnable> => 0x4000208);

register!(
    /// `enable` is the interrupt enable register.
    ///
    /// Any interrupts in the set will be enabled.
    ///
    /// Note that interrupts also have to be enabled in their corresponding peripheral.
    pub enable: ReadWrite<SourceSet> => 0x4000200);

register!(
    /// `sources` is the interrupt flag register.
    ///
    /// Inside an interrupt context, this indicates the
    /// source or sources that caused the current interrupt
    pub sources: ReadOnly<SourceSet> => 0x4000202);

register!(
    /// `irq_acknowledge` is the special interrupt acknowledge
    /// register.
    ///
    /// Interrupt handlers *must* write to this register indicating
    /// which sources were handled during the interrupt.  If not,
    /// the interrupt will continue to fire until the source is
    /// acknowledged.
    ///
    /// Note that you should only `write` the sources you handled;
    /// using `|=` is not the correct thing to do here.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gba_hw::interrupts;
    /// // indicate that we have handled VBLANK
    /// unsafe {
    ///     interrupts::irq_acknowledge().write(interrupts::VBLANK);
    /// }
    ///
    /// // indicate that we have handled all sources
    /// unsafe {
    ///     interrupts::irq_acknowledge().write(interrupts::SourceSet::all());
    /// }
    /// ```
    pub irq_acknowledge: WriteOnly<SourceSet> => 0x4000202);

register!(
    /// `irq_acknowledge_bios` is the special interrupt acknowledge
    /// register for the BIOS.
    ///
    /// Interrupt handlers should write to this register if the
    /// application uses any of the `wait..` functions from the bios.
    ///
    /// usage is identical to `irq_acknowledge`.
    pub irq_acknowledge_bios: WriteOnly<SourceSet> => 0x03007FF8);
