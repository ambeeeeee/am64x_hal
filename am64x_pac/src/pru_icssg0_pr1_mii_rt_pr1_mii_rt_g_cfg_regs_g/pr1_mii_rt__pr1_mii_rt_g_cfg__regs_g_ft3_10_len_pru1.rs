#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_10_len_pru1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_10LenPru1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_10_len_pru1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_10LenPru1Spec>;
#[doc = "Field `FT3_10_LEN_START_BIT_PRU1` reader - 8:0\\]
Defines relative bit offset from the HIT byte location upto 512 bit offset the exraction to determine the on the fly length byte offset"]
pub type Ft3_10LenStartBitPru1R = crate::FieldReader<u16>;
#[doc = "Field `FT3_10_LEN_START_BIT_PRU1` writer - 8:0\\]
Defines relative bit offset from the HIT byte location upto 512 bit offset the exraction to determine the on the fly length byte offset"]
pub type Ft3_10LenStartBitPru1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `FT3_10_LEN_SIZE_BIT_PRU1` reader - 19:16\\]
Defines number of bits to extract the length for the auto skip function"]
pub type Ft3_10LenSizeBitPru1R = crate::FieldReader;
#[doc = "Field `FT3_10_LEN_SIZE_BIT_PRU1` writer - 19:16\\]
Defines number of bits to extract the length for the auto skip function"]
pub type Ft3_10LenSizeBitPru1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FT3_10_LEN_BIG_EN_PRU1` reader - 24:24\\]
Enable Big Indian on Length"]
pub type Ft3_10LenBigEnPru1R = crate::BitReader;
#[doc = "Field `FT3_10_LEN_BIG_EN_PRU1` writer - 24:24\\]
Enable Big Indian on Length"]
pub type Ft3_10LenBigEnPru1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Defines relative bit offset from the HIT byte location upto 512 bit offset the exraction to determine the on the fly length byte offset"]
    #[inline(always)]
    pub fn ft3_10_len_start_bit_pru1(&self) -> Ft3_10LenStartBitPru1R {
        Ft3_10LenStartBitPru1R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines number of bits to extract the length for the auto skip function"]
    #[inline(always)]
    pub fn ft3_10_len_size_bit_pru1(&self) -> Ft3_10LenSizeBitPru1R {
        Ft3_10LenSizeBitPru1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable Big Indian on Length"]
    #[inline(always)]
    pub fn ft3_10_len_big_en_pru1(&self) -> Ft3_10LenBigEnPru1R {
        Ft3_10LenBigEnPru1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Defines relative bit offset from the HIT byte location upto 512 bit offset the exraction to determine the on the fly length byte offset"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_10_len_start_bit_pru1(
        &mut self,
    ) -> Ft3_10LenStartBitPru1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_10LenPru1Spec> {
        Ft3_10LenStartBitPru1W::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines number of bits to extract the length for the auto skip function"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_10_len_size_bit_pru1(
        &mut self,
    ) -> Ft3_10LenSizeBitPru1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_10LenPru1Spec> {
        Ft3_10LenSizeBitPru1W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable Big Indian on Length"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_10_len_big_en_pru1(
        &mut self,
    ) -> Ft3_10LenBigEnPru1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_10LenPru1Spec> {
        Ft3_10LenBigEnPru1W::new(self, 24)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_10_len_pru1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_10_len_pru1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_10_len_pru1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_10LenPru1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_10LenPru1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_10_len_pru1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_10LenPru1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_10_len_pru1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_10LenPru1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_10_len_pru1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_10LenPru1Spec {
    const RESET_VALUE: u32 = 0;
}
