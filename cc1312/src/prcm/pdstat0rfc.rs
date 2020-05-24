#[doc = "Reader of register PDSTAT0RFC"]
pub type R = crate::R<u32, super::PDSTAT0RFC>;
#[doc = "Reader of field `ON`"]
pub type ON_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ON"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new((self.bits & 0x01) != 0)
    }
}
