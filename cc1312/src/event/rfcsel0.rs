#[doc = "Reader of register RFCSEL0"]
pub type R = crate::R<u32, super::RFCSEL0>;
#[doc = "Reader of field `EV`"]
pub type EV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - EV"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
