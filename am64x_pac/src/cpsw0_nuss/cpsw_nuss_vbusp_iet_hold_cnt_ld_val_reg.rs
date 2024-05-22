#[doc = "Register `CPSW_NUSS_VBUSP_IET_HOLD_CNT_LD_VAL_REG` reader"]
pub type R = crate::R<CpswNussVbuspIetHoldCntLdValRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_IET_HOLD_CNT_LD_VAL_REG` writer"]
pub type W = crate::W<CpswNussVbuspIetHoldCntLdValRegSpec>;
#[doc = "Field `IET_HOLD_CNT_LD_VAL` reader - 7:0\\]
IET_HOLD_CNT_LD_VAL"]
pub type IetHoldCntLdValR = crate::FieldReader;
#[doc = "Field `IET_HOLD_CNT_LD_VAL` writer - 7:0\\]
IET_HOLD_CNT_LD_VAL"]
pub type IetHoldCntLdValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
IET_HOLD_CNT_LD_VAL"]
    #[inline(always)]
    pub fn iet_hold_cnt_ld_val(&self) -> IetHoldCntLdValR {
        IetHoldCntLdValR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
IET_HOLD_CNT_LD_VAL"]
    #[inline(always)]
    #[must_use]
    pub fn iet_hold_cnt_ld_val(&mut self) -> IetHoldCntLdValW<CpswNussVbuspIetHoldCntLdValRegSpec> {
        IetHoldCntLdValW::new(self, 0)
    }
}
#[doc = "IET Hold Count Load Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspIetHoldCntLdValRegSpec;
impl crate::RegisterSpec for CpswNussVbuspIetHoldCntLdValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspIetHoldCntLdValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspIetHoldCntLdValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_IET_HOLD_CNT_LD_VAL_REG to value 0x0100"]
impl crate::Resettable for CpswNussVbuspIetHoldCntLdValRegSpec {
    const RESET_VALUE: u32 = 0x0100;
}
