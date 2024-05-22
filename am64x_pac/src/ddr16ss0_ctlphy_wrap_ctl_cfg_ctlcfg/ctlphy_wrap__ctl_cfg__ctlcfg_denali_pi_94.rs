#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_94` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi94Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_94` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi94Spec>;
#[doc = "Field `PI_BANK_MUX_1` reader - 4:0\\]
Command pin BANK_1 mux selector"]
pub type PiBankMux1R = crate::FieldReader;
#[doc = "Field `PI_BANK_MUX_1` writer - 4:0\\]
Command pin BANK_1 mux selector"]
pub type PiBankMux1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_DATA_BYTE_SWAP_EN` reader - 8:8\\]
DATA pin swap function enable"]
pub type PiDataByteSwapEnR = crate::BitReader;
#[doc = "Field `PI_DATA_BYTE_SWAP_EN` writer - 8:8\\]
DATA pin swap function enable"]
pub type PiDataByteSwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DATA_BYTE_SWAP_SLICE0` reader - 16:16\\]
DATA pin 0 mux selector"]
pub type PiDataByteSwapSlice0R = crate::BitReader;
#[doc = "Field `PI_DATA_BYTE_SWAP_SLICE0` writer - 16:16\\]
DATA pin 0 mux selector"]
pub type PiDataByteSwapSlice0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DATA_BYTE_SWAP_SLICE1` reader - 24:24\\]
DATA pin 1 mux selector"]
pub type PiDataByteSwapSlice1R = crate::BitReader;
#[doc = "Field `PI_DATA_BYTE_SWAP_SLICE1` writer - 24:24\\]
DATA pin 1 mux selector"]
pub type PiDataByteSwapSlice1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Command pin BANK_1 mux selector"]
    #[inline(always)]
    pub fn pi_bank_mux_1(&self) -> PiBankMux1R {
        PiBankMux1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
DATA pin swap function enable"]
    #[inline(always)]
    pub fn pi_data_byte_swap_en(&self) -> PiDataByteSwapEnR {
        PiDataByteSwapEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DATA pin 0 mux selector"]
    #[inline(always)]
    pub fn pi_data_byte_swap_slice0(&self) -> PiDataByteSwapSlice0R {
        PiDataByteSwapSlice0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DATA pin 1 mux selector"]
    #[inline(always)]
    pub fn pi_data_byte_swap_slice1(&self) -> PiDataByteSwapSlice1R {
        PiDataByteSwapSlice1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Command pin BANK_1 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_bank_mux_1(&mut self) -> PiBankMux1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi94Spec> {
        PiBankMux1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DATA pin swap function enable"]
    #[inline(always)]
    #[must_use]
    pub fn pi_data_byte_swap_en(
        &mut self,
    ) -> PiDataByteSwapEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi94Spec> {
        PiDataByteSwapEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
DATA pin 0 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_data_byte_swap_slice0(
        &mut self,
    ) -> PiDataByteSwapSlice0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi94Spec> {
        PiDataByteSwapSlice0W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
DATA pin 1 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_data_byte_swap_slice1(
        &mut self,
    ) -> PiDataByteSwapSlice1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi94Spec> {
        PiDataByteSwapSlice1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_94\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_94::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_94::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi94Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi94Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_94::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi94Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_94::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi94Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_94 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi94Spec {
    const RESET_VALUE: u32 = 0;
}
