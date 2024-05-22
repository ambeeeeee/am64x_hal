#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_44` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl44Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_44` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl44Spec>;
#[doc = "Field `TBST_INT_INTERVAL` reader - 2:0\\]
DRAM burst interrupt interval value in cycles."]
pub type TbstIntIntervalR = crate::FieldReader;
#[doc = "Field `TBST_INT_INTERVAL` writer - 2:0\\]
DRAM burst interrupt interval value in cycles."]
pub type TbstIntIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TCCD` reader - 12:8\\]
DRAM CAS-to-CAS value in cycles."]
pub type TccdR = crate::FieldReader;
#[doc = "Field `TCCD` writer - 12:8\\]
DRAM CAS-to-CAS value in cycles."]
pub type TccdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCCD_L_F0` reader - 20:16\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=0"]
pub type TccdLF0R = crate::FieldReader;
#[doc = "Field `TCCD_L_F0` writer - 20:16\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=0"]
pub type TccdLF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRRD_F0` reader - 31:24\\]
DRAM TRRD value in cycles. FC=0"]
pub type TrrdF0R = crate::FieldReader;
#[doc = "Field `TRRD_F0` writer - 31:24\\]
DRAM TRRD value in cycles. FC=0"]
pub type TrrdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
DRAM burst interrupt interval value in cycles."]
    #[inline(always)]
    pub fn tbst_int_interval(&self) -> TbstIntIntervalR {
        TbstIntIntervalR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM CAS-to-CAS value in cycles."]
    #[inline(always)]
    pub fn tccd(&self) -> TccdR {
        TccdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=0"]
    #[inline(always)]
    pub fn tccd_l_f0(&self) -> TccdLF0R {
        TccdLF0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TRRD value in cycles. FC=0"]
    #[inline(always)]
    pub fn trrd_f0(&self) -> TrrdF0R {
        TrrdF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
DRAM burst interrupt interval value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tbst_int_interval(
        &mut self,
    ) -> TbstIntIntervalW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl44Spec> {
        TbstIntIntervalW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM CAS-to-CAS value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tccd(&mut self) -> TccdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl44Spec> {
        TccdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tccd_l_f0(&mut self) -> TccdLF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl44Spec> {
        TccdLF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TRRD value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn trrd_f0(&mut self) -> TrrdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl44Spec> {
        TrrdF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl44Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_44::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl44Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_44::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_44 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl44Spec {
    const RESET_VALUE: u32 = 0;
}
