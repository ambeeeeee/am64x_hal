#[doc = "Register `CPSW_NUSS_VBUSP_POLL_REG` reader"]
pub type R = crate::R<CpswNussVbuspPollRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_POLL_REG` writer"]
pub type W = crate::W<CpswNussVbuspPollRegSpec>;
#[doc = "Field `IPG` reader - 7:0\\]
MDIO IPG"]
pub type IpgR = crate::FieldReader;
#[doc = "Field `IPG` writer - 7:0\\]
MDIO IPG"]
pub type IpgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STATECHANGEMODE` reader - 30:30\\]
MDIO State Change Mode"]
pub type StatechangemodeR = crate::BitReader;
#[doc = "Field `STATECHANGEMODE` writer - 30:30\\]
MDIO State Change Mode"]
pub type StatechangemodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MANUALMODE` reader - 31:31\\]
MDIO Manual Mode"]
pub type ManualmodeR = crate::BitReader;
#[doc = "Field `MANUALMODE` writer - 31:31\\]
MDIO Manual Mode"]
pub type ManualmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
MDIO IPG"]
    #[inline(always)]
    pub fn ipg(&self) -> IpgR {
        IpgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
MDIO State Change Mode"]
    #[inline(always)]
    pub fn statechangemode(&self) -> StatechangemodeR {
        StatechangemodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
MDIO Manual Mode"]
    #[inline(always)]
    pub fn manualmode(&self) -> ManualmodeR {
        ManualmodeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
MDIO IPG"]
    #[inline(always)]
    #[must_use]
    pub fn ipg(&mut self) -> IpgW<CpswNussVbuspPollRegSpec> {
        IpgW::new(self, 0)
    }
    #[doc = "Bit 30 - 30:30\\]
MDIO State Change Mode"]
    #[inline(always)]
    #[must_use]
    pub fn statechangemode(&mut self) -> StatechangemodeW<CpswNussVbuspPollRegSpec> {
        StatechangemodeW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
MDIO Manual Mode"]
    #[inline(always)]
    #[must_use]
    pub fn manualmode(&mut self) -> ManualmodeW<CpswNussVbuspPollRegSpec> {
        ManualmodeW::new(self, 31)
    }
}
#[doc = "MDIO Poll Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_poll_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_poll_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspPollRegSpec;
impl crate::RegisterSpec for CpswNussVbuspPollRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_poll_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspPollRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_poll_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspPollRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_POLL_REG to value 0"]
impl crate::Resettable for CpswNussVbuspPollRegSpec {
    const RESET_VALUE: u32 = 0;
}
