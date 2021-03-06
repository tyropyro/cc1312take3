#[doc = "Reader of register PWRSTAT"]
pub type R = crate::R<u32, super::PWRSTAT>;
#[doc = "Writer for register PWRSTAT"]
pub type W = crate::W<u32, super::PWRSTAT>;
#[doc = "Register PWRSTAT `reset()`'s with value 0x03c0_0003"]
impl crate::ResetValue for super::PWRSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03c0_0003
    }
}
#[doc = "Reader of field `JTAG_PD_ON`"]
pub type JTAG_PD_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JTAG_PD_ON`"]
pub struct JTAG_PD_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_PD_ON_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `AUX_BUS_RESET_DONE`"]
pub type AUX_BUS_RESET_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_BUS_RESET_DONE`"]
pub struct AUX_BUS_RESET_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_BUS_RESET_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `AUX_RESET_DONE`"]
pub type AUX_RESET_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_RESET_DONE`"]
pub struct AUX_RESET_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_RESET_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - JTAG_PD_ON"]
    #[inline(always)]
    pub fn jtag_pd_on(&self) -> JTAG_PD_ON_R {
        JTAG_PD_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AUX_BUS_RESET_DONE"]
    #[inline(always)]
    pub fn aux_bus_reset_done(&self) -> AUX_BUS_RESET_DONE_R {
        AUX_BUS_RESET_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AUX_RESET_DONE"]
    #[inline(always)]
    pub fn aux_reset_done(&self) -> AUX_RESET_DONE_R {
        AUX_RESET_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - JTAG_PD_ON"]
    #[inline(always)]
    pub fn jtag_pd_on(&mut self) -> JTAG_PD_ON_W {
        JTAG_PD_ON_W { w: self }
    }
    #[doc = "Bit 1 - AUX_BUS_RESET_DONE"]
    #[inline(always)]
    pub fn aux_bus_reset_done(&mut self) -> AUX_BUS_RESET_DONE_W {
        AUX_BUS_RESET_DONE_W { w: self }
    }
    #[doc = "Bit 0 - AUX_RESET_DONE"]
    #[inline(always)]
    pub fn aux_reset_done(&mut self) -> AUX_RESET_DONE_W {
        AUX_RESET_DONE_W { w: self }
    }
}
