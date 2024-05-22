#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rmwrmcnt` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcntSpec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rmwrmcnt` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcntSpec>;
#[doc = "Field `RMW_EVENT_CNT` reader - 15:0\\]
RMW event cnt"]
pub type RmwEventCntR = crate::FieldReader<u16>;
#[doc = "Field `RMW_EVENT_CNT` writer - 15:0\\]
RMW event cnt"]
pub type RmwEventCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RM_EVENT_CNT` reader - 31:16\\]
RM event cnt"]
pub type RmEventCntR = crate::FieldReader<u16>;
#[doc = "Field `RM_EVENT_CNT` writer - 31:16\\]
RM event cnt"]
pub type RmEventCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
RMW event cnt"]
    #[inline(always)]
    pub fn rmw_event_cnt(&self) -> RmwEventCntR {
        RmwEventCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
RM event cnt"]
    #[inline(always)]
    pub fn rm_event_cnt(&self) -> RmEventCntR {
        RmEventCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
RMW event cnt"]
    #[inline(always)]
    #[must_use]
    pub fn rmw_event_cnt(&mut self) -> RmwEventCntW<Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcntSpec> {
        RmwEventCntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
RM event cnt"]
    #[inline(always)]
    #[must_use]
    pub fn rm_event_cnt(&mut self) -> RmEventCntW<Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcntSpec> {
        RmEventCntW::new(self, 16)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rmwrmcnt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcntSpec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcntSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rmwrmcnt to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcntSpec {
    const RESET_VALUE: u32 = 0;
}
