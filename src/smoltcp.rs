use core::marker::PhantomData;

use crate::EthernetDriver;
use smoltcp::phy::{Checksum, ChecksumCapabilities, Device, DeviceCapabilities, RxToken, TxToken};
use smoltcp::time::Instant;
use usb_device::bus::UsbBus;

pub trait BorrowDriver<T> {
    fn borrow<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R;

    fn borrow_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R;
}

pub struct Phy<'a, B, R>
where
    B: UsbBus,
    R: BorrowDriver<EthernetDriver<'a, B>>,
{
    driver: R,
    phantom: PhantomData<&'a B>,
}

impl<'a, B, R> Phy<'a, B, R>
where
    B: UsbBus,
    R: BorrowDriver<EthernetDriver<'a, B>>,
{
    pub fn new(driver: R) -> Self {
        Self {
            driver,
            phantom: PhantomData,
        }
    }
}

impl<'d, 'a: 'd, B, R> Device<'d> for Phy<'a, B, R>
where
    B: UsbBus,
    R: for<'x> BorrowDriver<EthernetDriver<'x, B>> + 'd,
{
    type RxToken = Token<'d, B, R>;
    type TxToken = Token<'d, B, R>;

    fn receive(&'d mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        if self.driver.borrow(|d| d.incoming_packet()) {
            Some((
                Token {
                    driver: &self.driver,
                    phantom: PhantomData,
                },
                Token {
                    driver: &self.driver,
                    phantom: PhantomData,
                },
            ))
        } else {
            None
        }
    }

    fn transmit(&'d mut self) -> Option<Self::TxToken> {
        Some(Token {
            driver: &self.driver,
            phantom: PhantomData,
        })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1500;
        caps.max_burst_size = None;
        caps.checksum = ChecksumCapabilities::ignored();
        caps.checksum.ipv4 = Checksum::Tx;
        caps.checksum.icmpv4 = Checksum::Tx;
        caps.checksum.udp = Checksum::Tx;
        caps.checksum.tcp = Checksum::Tx;
        caps
    }
}

pub struct Token<'a, Bus, Borrow>
where
    Bus: UsbBus + 'a,
    Borrow: BorrowDriver<EthernetDriver<'a, Bus>>,
{
    driver: &'a Borrow,
    phantom: PhantomData<&'a Bus>,
}
impl<'a, Bus, Borrow> RxToken for Token<'a, Bus, Borrow>
where
    Bus: UsbBus,
    Borrow: BorrowDriver<EthernetDriver<'a, Bus>>,
{
    fn consume<R, F>(self, _timestamp: Instant, f: F) -> smoltcp_crate::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp_crate::Result<R>,
    {
        if let Some(result) = self.driver.borrow_mut(|d| d.read_packet(f)) {
            return result;
        } else {
            // We should never reach this - PHY driver checks if there is
            // any incoming packet before creating RxToken
            error!("BUG! RX token failed to receive Ethernet packet");

            return Err(smoltcp::Error::Exhausted);
        }
    }
}
impl<'a, Bus, Borrow> TxToken for Token<'a, Bus, Borrow>
where
    Bus: UsbBus,
    Borrow: BorrowDriver<EthernetDriver<'a, Bus>>,
{
    fn consume<R, F>(self, _timestamp: Instant, len: usize, f: F) -> smoltcp_crate::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp_crate::Result<R>,
    {
        if let Some(result) = self.driver.borrow_mut(|d| d.prepare_packet(len, f)) {
            return result;
        } else {
            return Err(smoltcp::Error::Exhausted);
        }
    }
}
