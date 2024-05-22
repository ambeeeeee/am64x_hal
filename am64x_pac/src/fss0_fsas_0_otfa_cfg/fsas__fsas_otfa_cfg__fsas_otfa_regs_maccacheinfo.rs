#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_maccacheinfo` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfoSpec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_maccacheinfo` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfoSpec>;
#[doc = "Field `CACHE_MISS_EVENT_CNT` reader - 15:0\\]
MAC Cache Miss event cnt"]
pub type CacheMissEventCntR = crate::FieldReader<u16>;
#[doc = "Field `CACHE_MISS_EVENT_CNT` writer - 15:0\\]
MAC Cache Miss event cnt"]
pub type CacheMissEventCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
MAC Cache Miss event cnt"]
    #[inline(always)]
    pub fn cache_miss_event_cnt(&self) -> CacheMissEventCntR {
        CacheMissEventCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
MAC Cache Miss event cnt"]
    #[inline(always)]
    #[must_use]
    pub fn cache_miss_event_cnt(
        &mut self,
    ) -> CacheMissEventCntW<Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfoSpec> {
        CacheMissEventCntW::new(self, 0)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_maccacheinfo\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfoSpec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfoSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_maccacheinfo to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfoSpec {
    const RESET_VALUE: u32 = 0;
}
