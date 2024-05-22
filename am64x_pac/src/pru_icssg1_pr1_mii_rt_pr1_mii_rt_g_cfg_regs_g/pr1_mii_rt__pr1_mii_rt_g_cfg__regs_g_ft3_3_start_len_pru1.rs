#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_3_start_len_pru1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_3StartLenPru1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_3_start_len_pru1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_3StartLenPru1Spec>;
#[doc = "Field `FT3_OFFSET` reader - 14:0\\]
Determine which byte to start FT3_\\[N\\]P\\[63:0\\]
operation it will start on the next byte after FT3_T completes if set to zero"]
pub type Ft3OffsetR = crate::FieldReader<u16>;
#[doc = "Field `FT3_OFFSET` writer - 14:0\\]
Determine which byte to start FT3_\\[N\\]P\\[63:0\\]
operation it will start on the next byte after FT3_T completes if set to zero"]
pub type Ft3OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `FT3_OFFSET_START` reader - 18:16\\]
Defines which byte within FT3_\\[N\\]P\\[63:0\\]
to start the compare"]
pub type Ft3OffsetStartR = crate::FieldReader;
#[doc = "Field `FT3_OFFSET_START` writer - 18:16\\]
Defines which byte within FT3_\\[N\\]P\\[63:0\\]
to start the compare"]
pub type Ft3OffsetStartW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FT3_OFFSET_END` reader - 22:20\\]
Defines which byte within FT3_\\[N\\]P\\[63:0\\]
to end the compare at set the Valid flag, rule ft3_offset_end >= ft3_offset_start To disable pattern compare set start and end to zero"]
pub type Ft3OffsetEndR = crate::FieldReader;
#[doc = "Field `FT3_OFFSET_END` writer - 22:20\\]
Defines which byte within FT3_\\[N\\]P\\[63:0\\]
to end the compare at set the Valid flag, rule ft3_offset_end >= ft3_offset_start To disable pattern compare set start and end to zero"]
pub type Ft3OffsetEndW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:14 - 14:0\\]
Determine which byte to start FT3_\\[N\\]P\\[63:0\\]
operation it will start on the next byte after FT3_T completes if set to zero"]
    #[inline(always)]
    pub fn ft3_offset(&self) -> Ft3OffsetR {
        Ft3OffsetR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Defines which byte within FT3_\\[N\\]P\\[63:0\\]
to start the compare"]
    #[inline(always)]
    pub fn ft3_offset_start(&self) -> Ft3OffsetStartR {
        Ft3OffsetStartR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Defines which byte within FT3_\\[N\\]P\\[63:0\\]
to end the compare at set the Valid flag, rule ft3_offset_end >= ft3_offset_start To disable pattern compare set start and end to zero"]
    #[inline(always)]
    pub fn ft3_offset_end(&self) -> Ft3OffsetEndR {
        Ft3OffsetEndR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - 14:0\\]
Determine which byte to start FT3_\\[N\\]P\\[63:0\\]
operation it will start on the next byte after FT3_T completes if set to zero"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_offset(&mut self) -> Ft3OffsetW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_3StartLenPru1Spec> {
        Ft3OffsetW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Defines which byte within FT3_\\[N\\]P\\[63:0\\]
to start the compare"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_offset_start(
        &mut self,
    ) -> Ft3OffsetStartW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_3StartLenPru1Spec> {
        Ft3OffsetStartW::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Defines which byte within FT3_\\[N\\]P\\[63:0\\]
to end the compare at set the Valid flag, rule ft3_offset_end >= ft3_offset_start To disable pattern compare set start and end to zero"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_offset_end(
        &mut self,
    ) -> Ft3OffsetEndW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_3StartLenPru1Spec> {
        Ft3OffsetEndW::new(self, 20)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_3_start_len_pru1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_3_start_len_pru1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_3_start_len_pru1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_3StartLenPru1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_3StartLenPru1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_3_start_len_pru1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_3StartLenPru1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_3_start_len_pru1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_3StartLenPru1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_3_start_len_pru1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_3StartLenPru1Spec {
    const RESET_VALUE: u32 = 0;
}
