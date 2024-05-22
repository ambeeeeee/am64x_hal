#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_121` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi121Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_121` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi121Spec>;
#[doc = "Field `PI_BIST_ADDR_MASK_9_1` reader - 1:0\\]
Defines an address to be masked during the BIST operation.."]
pub type PiBistAddrMask9_1R = crate::FieldReader;
#[doc = "Field `PI_BIST_ADDR_MASK_9_1` writer - 1:0\\]
Defines an address to be masked during the BIST operation.."]
pub type PiBistAddrMask9_1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_BIST_MODE` reader - 10:8\\]
Sets the BIST data checking mode. 'b00 indicates MOVI13N mode. 'b01 indicates March C mode. 'b10 indicates GALPAT mode. 'b11 indicates PRBS mode. 'b100 indicates programmable March data check mode."]
pub type PiBistModeR = crate::FieldReader;
#[doc = "Field `PI_BIST_MODE` writer - 10:8\\]
Sets the BIST data checking mode. 'b00 indicates MOVI13N mode. 'b01 indicates March C mode. 'b10 indicates GALPAT mode. 'b11 indicates PRBS mode. 'b100 indicates programmable March data check mode."]
pub type PiBistModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PI_BIST_ADDR_MODE` reader - 17:16\\]
Sets the address traversing order of BIST. 'b00 indicates fast column order \\[burst-column-bank-row-rank\\]. 'b01 indicates fast row order \\[burst-row-column-bank-rank\\]. 'b10 indicates fast bank order \\[burst-bank-column-row-rank\\]."]
pub type PiBistAddrModeR = crate::FieldReader;
#[doc = "Field `PI_BIST_ADDR_MODE` writer - 17:16\\]
Sets the address traversing order of BIST. 'b00 indicates fast column order \\[burst-column-bank-row-rank\\]. 'b01 indicates fast row order \\[burst-row-column-bank-rank\\]. 'b10 indicates fast bank order \\[burst-bank-column-row-rank\\]."]
pub type PiBistAddrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_BIST_PAT_MODE` reader - 25:24\\]
Sets the pattern mode of BIST. 'b00 indicates using built-in pattern. 'b01 indicates checkerboard pattern, each data transfer inverts the last data transfer based on the built-in pattern. 'b10 indicates using both user pattern and built-in pattern. 'b11 indicates using pi lfsr random pattern."]
pub type PiBistPatModeR = crate::FieldReader;
#[doc = "Field `PI_BIST_PAT_MODE` writer - 25:24\\]
Sets the pattern mode of BIST. 'b00 indicates using built-in pattern. 'b01 indicates checkerboard pattern, each data transfer inverts the last data transfer based on the built-in pattern. 'b10 indicates using both user pattern and built-in pattern. 'b11 indicates using pi lfsr random pattern."]
pub type PiBistPatModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Defines an address to be masked during the BIST operation.."]
    #[inline(always)]
    pub fn pi_bist_addr_mask_9_1(&self) -> PiBistAddrMask9_1R {
        PiBistAddrMask9_1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Sets the BIST data checking mode. 'b00 indicates MOVI13N mode. 'b01 indicates March C mode. 'b10 indicates GALPAT mode. 'b11 indicates PRBS mode. 'b100 indicates programmable March data check mode."]
    #[inline(always)]
    pub fn pi_bist_mode(&self) -> PiBistModeR {
        PiBistModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Sets the address traversing order of BIST. 'b00 indicates fast column order \\[burst-column-bank-row-rank\\]. 'b01 indicates fast row order \\[burst-row-column-bank-rank\\]. 'b10 indicates fast bank order \\[burst-bank-column-row-rank\\]."]
    #[inline(always)]
    pub fn pi_bist_addr_mode(&self) -> PiBistAddrModeR {
        PiBistAddrModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Sets the pattern mode of BIST. 'b00 indicates using built-in pattern. 'b01 indicates checkerboard pattern, each data transfer inverts the last data transfer based on the built-in pattern. 'b10 indicates using both user pattern and built-in pattern. 'b11 indicates using pi lfsr random pattern."]
    #[inline(always)]
    pub fn pi_bist_pat_mode(&self) -> PiBistPatModeR {
        PiBistPatModeR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Defines an address to be masked during the BIST operation.."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_addr_mask_9_1(
        &mut self,
    ) -> PiBistAddrMask9_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi121Spec> {
        PiBistAddrMask9_1W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Sets the BIST data checking mode. 'b00 indicates MOVI13N mode. 'b01 indicates March C mode. 'b10 indicates GALPAT mode. 'b11 indicates PRBS mode. 'b100 indicates programmable March data check mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_mode(&mut self) -> PiBistModeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi121Spec> {
        PiBistModeW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Sets the address traversing order of BIST. 'b00 indicates fast column order \\[burst-column-bank-row-rank\\]. 'b01 indicates fast row order \\[burst-row-column-bank-rank\\]. 'b10 indicates fast bank order \\[burst-bank-column-row-rank\\]."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_addr_mode(
        &mut self,
    ) -> PiBistAddrModeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi121Spec> {
        PiBistAddrModeW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Sets the pattern mode of BIST. 'b00 indicates using built-in pattern. 'b01 indicates checkerboard pattern, each data transfer inverts the last data transfer based on the built-in pattern. 'b10 indicates using both user pattern and built-in pattern. 'b11 indicates using pi lfsr random pattern."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_pat_mode(&mut self) -> PiBistPatModeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi121Spec> {
        PiBistPatModeW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_121\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_121::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_121::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi121Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi121Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_121::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi121Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_121::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi121Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_121 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi121Spec {
    const RESET_VALUE: u32 = 0;
}
