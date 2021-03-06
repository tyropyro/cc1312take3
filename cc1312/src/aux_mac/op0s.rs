#[doc = "Writer for register OP0S"]
pub type W = crate::W<u32, super::OP0S>;
#[doc = "Register OP0S `reset()`'s with value 0"]
impl crate::ResetValue for super::OP0S {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OP0_VALUE`"]
pub struct OP0_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> OP0_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - OP0_VALUE"]
    #[inline(always)]
    pub fn op0_value(&mut self) -> OP0_VALUE_W {
        OP0_VALUE_W { w: self }
    }
}
